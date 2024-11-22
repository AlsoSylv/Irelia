//! Module containing all the data on the websocket LCU bindings

mod error;
mod impls;
pub mod types;
mod utils;

use impls::Returns;
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{ops::ControlFlow, sync::Arc, thread};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::util::NonBlockingResult;
use tungstenite::{client::IntoClientRequest, Connector, Message, WebSocket};

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::utils::{process_info::get_running_client, setup_tls::connector};
use crate::ws::types::{Event, EventKind, RequestType};
use crate::ws::utils::EventMap;

pub use error::Error as WebSocketError;

/// Type alias for the websocket stream type
pub type WebSocketStream = WebSocket<MaybeTlsStream<StreamOwned<ClientConnection, TcpStream>>>;

/// Struct representing a connection to the LCU websocket
pub struct LcuWebSocket {
    ws_sender: Sender<ChannelMessage>,
    handle: JoinHandle<()>,
    id_free_list: EventMap<(usize, Vec<usize>)>,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
/// This is the ID of the subscriber when it's inserted into the list, corresponding to the index it's stored at
pub struct SubscriberID(usize);

enum ChannelMessage {
    Subscribe(RequestType, EventKind, Box<dyn Subscriber + Send>),
    Unsubscribe(SubscriberID, EventKind),
    Abort,
}

#[derive(PartialEq, Eq)]
/// Enum representing what to do next, either continue the loop or attempt to reconnect
pub enum Flow {
    TryReconnect,
    Continue,
}

/// Behavior if a Mutex subscriber is poisoned
pub enum PoisonBehavior {
    /// Panics with the poison error
    Panic,
    /// Breaks the event loop if poisoned when `on_event` is called 
    /// Ignores otherwise waiting for `on_event` to be called
    Break,
    /// Ignores it and moves on
    Ignore,
    /// Clears the poison
    Clear,
}

/// trait for a subscriber to an endpoint for the websocket
pub trait Subscriber {
    /// Defines what to do if the mutex gets poisoned
    /// By default, the subscriber will panic, bringing the connection with it
    /// This only matters if the subscriber is a mutex, otherwise this is never called
    fn on_poison(&self) -> PoisonBehavior {
        PoisonBehavior::Panic
    }

    /// Callback run when the subscriber is added
    /// Default behavior is to do nothing
    fn on_subscribe(&mut self, _event_kind: &EventKind, _request_code: &RequestType) {}

    /// Callback for when the `EventKind` occurs
    /// Set `_continues` to false if you want to break the loop
    /// Defaults to true
    fn on_event(&mut self, event: &Event, _continues: &mut bool);

    /// Callback run when the subscriber is removed
    /// Default behavior is to do nothing
    fn on_unsubscribe(&mut self, _event_kind: &EventKind) {}
}

/// Error handler trait, called when the websocket connection errors in an unexpected way
pub trait ErrorHandler: Send {
    /// Callback whenever an unexpected error occurs during the event loop
    fn on_error(&mut self, error: WebSocketError) -> ControlFlow<(), Flow>;

    /// Callback run when the websocket connects or reconnects
    /// Default behavior is to set the TCP socket to nonblocking
    ///
    /// If this function errors, the result will be piped directly to the error handler
    ///
    /// # Errors
    /// By default, this returns an error if the socket cannot be set to nonblocking
    fn on_connect(&mut self, socket: &mut WebSocketStream) -> Result<(), WebSocketError> {
        match socket.get_ref() {
            MaybeTlsStream::Plain(_) => unimplemented!("The stream is always encrypted"),
            MaybeTlsStream::Rustls(stream_owned) => {
                stream_owned.sock.sock.set_nonblocking(true)?;
            }
            _ => unimplemented!("NativeTls is not supported"),
        }

        Ok(())
    }

    /// Callback run when the websocket connection timesout without
    /// receiving a message, default behavior is to sleep for half a second
    fn on_timeout(&mut self) {
        thread::sleep(Duration::from_millis(500));
    }
}

/// This is a zero sized struct which calls `eprintln!()` and then breaks on error
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn on_error(&mut self, error: WebSocketError) -> ControlFlow<(), Flow> {
        eprintln!("{error}");
        ControlFlow::Break(())
    }
}

impl Default for LcuWebSocket {
    #[must_use]
    /// Creates a new connection to the LCU websocket using the default error handler
    fn default() -> Self {
        Self::new()
    }
}

impl LcuWebSocket {
    #[must_use]
    /// Creates a new connection to the LCU websocket using the default error handler
    pub fn new() -> Self {
        Self::new_with_error_handler(DefaultErrorHandler)
    }

    #[must_use]
    /// Creates a new connection to the LCU websocket
    pub fn new_with_error_handler(error_handler: impl ErrorHandler + 'static) -> Self {
        let (ws_sender, ws_receiver) = std::sync::mpsc::channel::<ChannelMessage>();

        let handle = thread::spawn(move || {
            let tls = Arc::new(connector().clone());

            let mut error_handler = error_handler;
            let ws_receiver = ws_receiver;

            event_loop(&mut error_handler, &ws_receiver, &tls);
        });

        Self {
            ws_sender,
            handle,
            id_free_list: EventMap::new(),
        }
    }

    /// Subscribes to a specific event kind using the subscriber
    ///
    /// Returns `None` is the websocket connection has already been closed previously
    pub fn subscribe(
        &mut self,
        event_kind: EventKind,
        subscriber: impl Subscriber + Send + 'static,
    ) -> Option<SubscriberID> {
        let (next_id, returned) = self.id_free_list.get_mut(&event_kind);

        self.ws_sender
            .send(ChannelMessage::Subscribe(
                RequestType::Subscribe,
                event_kind,
                Box::new(subscriber),
            ))
            .ok()?;

        let id = if returned.is_empty() {
            let tmp = *next_id;
            *next_id += 1;
            tmp
        } else {
            returned.remove(0)
        };

        Some(SubscriberID(id))
    }

    pub fn subscribe_closure<R: Returns>(
        &mut self,
        event_kind: EventKind,
        subscribe_closure: impl Fn(&Event) -> R + Send + 'static,
    ) -> Option<SubscriberID> {
        let (next_id, returned) = self.id_free_list.get_mut(&event_kind);

        self.ws_sender
            .send(ChannelMessage::Subscribe(
                RequestType::Subscribe,
                event_kind,
                Box::new(subscribe_closure),
            ))
            .ok()?;

        let id = if returned.is_empty() {
            let tmp = *next_id;
            *next_id += 1;
            tmp
        } else {
            returned.remove(0)
        };

        Some(SubscriberID(id))
    }

    /// Unsubscribe to a new API event
    ///
    /// If all subscribers have been removed, this will unsubscribe from the event as a whole
    ///
    /// Returns `None` if the connection to the websocket was already closed
    pub fn unsubscribe(&mut self, event_kind: EventKind, id: SubscriberID) -> Option<()> {
        let (_, returned) = self.id_free_list.get_mut(&event_kind);

        self.ws_sender
            .send(ChannelMessage::Unsubscribe(id, event_kind))
            .ok()?;

        returned.push(id.0);

        Some(())
    }

    #[must_use]
    /// Terminate the event loop
    pub fn abort(self) -> Option<()> {
        self.ws_sender.send(ChannelMessage::Abort).ok()
    }

    #[must_use]
    /// Checks whether the underlying thread is finished or not
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

// Workaround for lifetime issues with closures
pub fn force<R: Returns, F: FnMut(&Event) -> R + Send>(f: F) -> F {
    f
}

type SubscriberMap = EventMap<Vec<Option<Box<dyn Subscriber>>>>;

fn event_loop(
    error_handler: &mut impl ErrorHandler,
    receiver: &Receiver<ChannelMessage>,
    tls: &Arc<ClientConfig>,
) {
    // The stare of the websocket
    let mut maybe_stream: Option<WebSocketStream> = None;
    let mut subscribers = SubscriberMap::new();
    let mut control_flow = ControlFlow::Continue(Flow::Continue);
    let mut abort = false;

    while control_flow.is_continue() {
        if let Some(stream) = &mut maybe_stream {
            if let Ok(message) = receiver.try_recv() {
                // Variable to determine if a message should be sent to the websocket, only one can be sent at a time
                let mut ws_message = None;

                match message {
                    ChannelMessage::Subscribe(code, event_kind, mut subscriber) => {
                        let subscribers = subscribers.get_mut(&event_kind);

                        // If the map is empty, we are not taking messages for this endpoint, so we have to subscribe
                        if subscribers.is_empty() {
                            let endpoint_str = event_kind.to_string();

                            let command = format!("[{}, \"{endpoint_str}\"]", code as u8);

                            ws_message = Some(Message::Text(command));
                        }

                        // Let the subscriber update its own state
                        subscriber.on_subscribe(&event_kind, &code);

                        if let Some(idx) = subscribers.iter().position(Option::is_none) {
                            subscribers[idx] = Some(subscriber);
                        } else {
                            subscribers.push(Some(subscriber));
                        }
                    }
                    ChannelMessage::Unsubscribe(subscriber_id, event_kind) => {
                        let subscribers = subscribers.get_mut(&event_kind);
                        let subscriber = &mut subscribers[subscriber_id.0];
                        if let Some(subscriber) = subscriber {
                            subscriber.on_unsubscribe(&event_kind);
                        }

                        *subscriber = None;

                        if subscribers.iter().flatten().count() == 0 {
                            let unsub = format!(
                                "[{}, \"{}\"]",
                                RequestType::Unsubscribe as u8,
                                event_kind.to_string()
                            );

                            ws_message = Some(Message::Text(unsub));
                        }
                    }
                    ChannelMessage::Abort => {
                        abort = true;
                        ws_message = Some(Message::Close(None));
                    }
                }

                if let Some(Err(e)) = ws_message.map(|m| stream.send(m)) {
                    control_flow = error_handler.on_error(e.into());
                }

                // If we're supposed to abort, we should not receive any new messages from the socket.
                if abort {
                    break;
                }
            };

            // Else if the `control_flow` is still to continue, we take out next message
            if control_flow == ControlFlow::Continue(Flow::Continue) {
                control_flow = receive_message(stream, &mut subscribers, error_handler)
                    .unwrap_or_else(|e| error_handler.on_error(e));
            }
        } else {
            connect(tls, error_handler).map_or_else(
                |e| control_flow = error_handler.on_error(e),
                |stream| maybe_stream = Some(stream),
            );
        }

        // If the `control_flow` is to try and reconnect, we make the stream `None` before the start of the next run
        if control_flow == ControlFlow::Continue(Flow::TryReconnect) {
            maybe_stream = None;
        }
    }

    if control_flow.is_break() {
        // Ignore if it's already closed
        let maybe_stream = maybe_stream.filter(WebSocket::can_write);
        if let Some(Err(e)) = maybe_stream.map(|mut stream| stream.send(Message::Close(None))) {
            let _ = error_handler.on_error(e.into());
        }
    }
}

fn receive_message(
    stream: &mut WebSocketStream,
    subscribers: &mut SubscriberMap,
    error_handler: &mut impl ErrorHandler,
) -> Result<ControlFlow<(), Flow>, WebSocketError> {
    let read = stream
        .read()
        .no_block()?
        .filter(|msg| !msg.is_empty())
        .map(Message::into_data);

    if let Some(data) = read {
        let json = serde_json::from_slice::<Event>(&data)?;
        let subscribers = subscribers.get_mut(&json.1);

        for subscriber in subscribers.iter_mut().flatten() {
            let mut continues = true;

            subscriber.on_event(&json, &mut continues);

            if !continues {
                return Ok(ControlFlow::Break(()));
            }
        }
    } else {
        error_handler.on_timeout();
    }

    Ok(ControlFlow::Continue(Flow::Continue))
}

fn connect(
    tls: &Arc<ClientConfig>,
    error_handler: &mut impl ErrorHandler,
) -> Result<WebSocketStream, WebSocketError> {
    const TIMEOUT: Duration = Duration::from_millis(100);

    let (addr, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;
    let addr = SocketAddr::V4(addr);

    let str_req = format!("wss://{addr}");

    let mut request = str_req.into_client_request()?;

    request.headers_mut().insert("Authorization", auth?);

    let tcp_stream = TcpStream::connect_timeout(&addr, TIMEOUT)?;

    let addr = ServerName::IpAddress(addr.ip().into());

    let client_connection = ClientConnection::new(tls.clone(), addr).unwrap();
    let rustls_stream = StreamOwned::new(client_connection, tcp_stream);

    let (mut stream, _) = tungstenite::client_tls_with_config(
        request.clone(),
        rustls_stream,
        None,
        Some(Connector::Rustls(tls.clone())),
    )
    .expect("The TLS handshake should never fail");

    error_handler.on_connect(&mut stream)?;

    Ok(stream)
}

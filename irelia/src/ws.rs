//! Module containing all the data on the websocket LCU bindings

mod error;
pub mod types;
mod utils;

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{ops::ControlFlow, sync::Arc, thread};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::util::NonBlockingResult;
use tungstenite::{client::IntoClientRequest, http::HeaderValue, Connector, Message, WebSocket};

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::utils::{process_info::get_running_client, setup_tls::connector};
use crate::ws::types::{Event, EventKind, RequestType};
use crate::ws::utils::EventMap;

pub use error::Error as WebSocketError;

type WebSocketStream = WebSocket<MaybeTlsStream<StreamOwned<ClientConnection, TcpStream>>>;

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
    Subscribe(RequestType, EventKind, Box<dyn Subscriber>),
    Unsubscribe(SubscriberID, EventKind),
    Abort,
}

#[derive(PartialEq, Eq)]
/// Enum representing what to do next, either continue the loop or attempt to reconnect
pub enum Flow {
    TryReconnect,
    Continue,
}

/// trait for a subscriber to an endpoint for the websocket
pub trait Subscriber: Send + Sync {
    /// Callback for when the `EventKind` occurs
    fn on_event(&mut self, event: &Event) -> ControlFlow<(), Flow>;
}

/// Error handler trait, called when the websocket connection errors in an unexpected way
pub trait ErrorHandler: Send + Sync {
    /// Callback whenever an unexpected error occurs during the event loop
    fn on_error(&mut self, error: WebSocketError) -> ControlFlow<(), Flow>;
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
            let tls = connector();
            let tls = Arc::new(tls.clone());

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
        subscriber: impl Subscriber + 'static,
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

fn event_loop(
    error_handler: &mut impl ErrorHandler,
    receiver: &Receiver<ChannelMessage>,
    tls: &Arc<ClientConfig>,
) {
    type SubscriberMap = EventMap<Vec<Option<Box<dyn Subscriber>>>>;

    let mut maybe_stream = None;
    let mut subscribers = SubscriberMap::new();

    'outer: loop {
        if let Some(stream) = &mut maybe_stream {
            if let Ok(message) = receiver.try_recv() {
                match message {
                    ChannelMessage::Subscribe(code, event_kind, subscriber) => {
                        let subscribers = subscribers.get_mut(&event_kind);
                        if subscribers.is_empty() {
                            let endpoint_str = event_kind.to_string();

                            let command = format!("[{}, \"{endpoint_str}\"]", code.clone() as u8);

                            if !send_command(error_handler, stream, tls, command) {
                                break 'outer;
                            }
                        }

                        let mut iter = subscribers.iter().enumerate();

                        let idx = iter
                            .find_map(|(idx, subscriber)| {
                                if subscriber.is_none() {
                                    Some(idx)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(subscribers.len());

                        if idx == subscribers.len() {
                            subscribers.push(Some(subscriber));
                        } else {
                            subscribers[idx] = Some(subscriber);
                        }
                    }
                    ChannelMessage::Unsubscribe(subscriber_id, event_kind) => {
                        let subscribers = subscribers.get_mut(&event_kind);

                        subscribers[subscriber_id.0] = None;

                        if subscribers.iter().flatten().count() == 0 {
                            let unsub = format!(
                                "[{}, \"{}\"]",
                                RequestType::Unsubscribe as u8,
                                event_kind.to_string()
                            );

                            if !send_command(error_handler, stream, tls, unsub) {
                                break 'outer;
                            }
                        }
                    }
                    ChannelMessage::Abort => break 'outer,
                }
            };

            let read = stream.read();

            match read.no_block() {
                Ok(Some(message)) => {
                    let data = message.into_data();
                    if !data.is_empty() {
                        match serde_json::from_slice::<Event>(&data) {
                            Ok(json) => {
                                let subscribers = subscribers.get_mut(&json.1);

                                for subscriber in subscribers.iter_mut().flatten() {
                                    let mut control = subscriber.on_event(&json);

                                    if !budget_recursive(&mut control, tls, stream, error_handler) {
                                        break 'outer;
                                    }
                                }
                            }
                            Err(e) => {
                                let mut control = error_handler.on_error(e.into());

                                if !budget_recursive(&mut control, tls, stream, error_handler) {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                Ok(None) => thread::sleep(Duration::from_millis(10)),
                Err(e) => {
                    if error_handler.on_error(e.into()) == ControlFlow::Break(()) {
                        break 'outer;
                    }
                }
            }
        } else {
            match connect(tls) {
                Ok(stream) => maybe_stream = Some(stream),
                Err(e) => {
                    if error_handler.on_error(e) == ControlFlow::Break(()) {
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn send_command(
    error_handler: &mut impl ErrorHandler,
    stream: &mut WebSocketStream,
    tls: &Arc<ClientConfig>,
    command: String,
) -> bool {
    if let Err(e) = stream.send(Message::Text(command)) {
        let mut control = error_handler.on_error(e.into());

        #[rustfmt::skip]
        let continues = budget_recursive(&mut control, tls, stream, error_handler);

        if !continues {
            return false;
        }
    }

    true
}

fn budget_recursive(
    c: &mut ControlFlow<(), Flow>,
    tls: &Arc<ClientConfig>,
    stream: &mut WebSocketStream,
    f: &mut impl ErrorHandler,
) -> bool {
    while *c != ControlFlow::Continue(Flow::Continue) {
        if *c == ControlFlow::Break(()) {
            return false;
        }

        match connect(tls) {
            Ok(new_stream) => {
                *stream = new_stream;
                break;
            }
            Err(e) => {
                *c = f.on_error(e);
            }
        }
    }

    true
}

fn connect(tls: &Arc<ClientConfig>) -> Result<WebSocketStream, WebSocketError> {
    const TIMEOUT: Duration = Duration::from_millis(100);

    let (addr, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;

    let str_req = format!("wss://{addr}");

    let auth_header = HeaderValue::from_str(&auth).unwrap();

    let mut request = str_req.into_client_request()?;

    request.headers_mut().insert("Authorization", auth_header);

    let tcp_stream = TcpStream::connect_timeout(&addr, TIMEOUT)?;

    let addr = ServerName::IpAddress(addr.ip().into());

    let client_connection = ClientConnection::new(tls.clone(), addr).unwrap();
    let rustls_stream = StreamOwned::new(client_connection, tcp_stream);

    let (stream, _) = tungstenite::client_tls_with_config(
        request.clone(),
        rustls_stream,
        None,
        Some(Connector::Rustls(tls.clone())),
    )
    .expect("The TLS handshake should never fail");

    let MaybeTlsStream::Rustls(stream_inner) = stream.get_ref() else {
        unreachable!();
    };

    stream_inner.sock.sock.set_nonblocking(true)?;

    Ok(stream)
}

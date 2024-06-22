//! Module containing all the data on the websocket LCU bindings

pub mod types;
mod utils;

use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{ops::ControlFlow, sync::Arc, thread};

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::util::NonBlockingResult;
use tungstenite::{client::IntoClientRequest, http::HeaderValue, Connector, Message, WebSocket};

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::ws::types::{Event, EventKind, RequestType};
use crate::ws::utils::EventMap;
use crate::{
    process_info,
    utils::{process_info::get_running_client, setup_tls::connector},
};

type WebSocketStream = WebSocket<MaybeTlsStream<StreamOwned<ClientConnection, TcpStream>>>;

/// Struct representing a connection to the LCU websocket
pub struct LcuWebSocket {
    ws_sender: Sender<ChannelMessage>,
    handle: JoinHandle<()>,
    id_free_list: EventMap<(usize, Vec<usize>)>,
}

#[derive(PartialEq)]
pub enum Flow {
    TryReconnect,
    Continue,
}

pub trait Subscriber: Send + Sync {
    fn on_event(&mut self, event: &Event) -> ControlFlow<(), Flow>;
}

pub trait ErrorHandler: Send + Sync {
    fn on_error(&mut self, error: WebsocketError) -> ControlFlow<(), Flow>;
}

/// This is a zero sized struct which calls `eprintln!()` and then breaks on error
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn on_error(&mut self, error: WebsocketError) -> ControlFlow<(), Flow> {
        eprintln!("{error}");
        ControlFlow::Break(())
    }
}

enum ChannelMessage {
    Subscribe(RequestType, EventKind, Box<dyn Subscriber>),
    Unsubscribe(SubscriberID, EventKind),
    Abort,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SubscriberID(usize);

impl Default for LcuWebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl LcuWebSocket {
    #[must_use]
    /// Creates a new connection to the LCU websocket using the default error handler
    ///
    /// # Errors
    /// This function will return an error if the LCU is not running,
    /// or if it cannot connect to the websocket
    ///
    /// # Panics
    ///
    /// If the auth header returned is somehow invalid (though I have not seen this in practice)
    pub fn new() -> Self {
        Self::new_with_error_handler(DefaultErrorHandler)
    }

    #[must_use]
    /// Creates a new connection to the LCU websocket
    ///
    /// # Errors
    /// This function will return an error if the LCU is not running,
    /// or if it cannot connect to the websocket
    ///
    /// # Panics
    ///
    /// If the auth header returned is somehow invalid (though I have not seen this in practice)
    pub fn new_with_error_handler(
        error_handler: impl ErrorHandler + 'static,
    ) -> Self {
        let tls = connector();
        let tls = Arc::new(tls.clone());

        let (ws_sender, ws_receiver) = std::sync::mpsc::channel::<ChannelMessage>();

        let handle = thread::spawn(move || {
            let ws_receiver = ws_receiver;
            let tls = tls;

            event_loop(error_handler, &ws_receiver, &tls);
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

        returned.push(id.0);

        self.ws_sender
            .send(ChannelMessage::Unsubscribe(id, event_kind))
            .ok()
    }

    #[must_use]
    /// Terminate the event loop
    pub fn abort(self) -> Option<()> {
        self.ws_sender.send(ChannelMessage::Abort).ok()
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

fn event_loop(
    mut error_handler: impl ErrorHandler,
    ws_receiver: &Receiver<ChannelMessage>,
    tls: &Arc<ClientConfig>,
) {
    type SubscriberMap = EventMap<Vec<Option<Box<dyn Subscriber>>>>;

    let mut maybe_stream = None;

    let mut subscribers: SubscriberMap = SubscriberMap::new();
    let error_handler: &mut dyn ErrorHandler = &mut error_handler;

    'outer: loop {
        if let Some(stream) = &mut maybe_stream {
            if let Ok(message) = ws_receiver.try_recv() {
                match message {
                    ChannelMessage::Subscribe(code, event_kind, subscriber) => {
                        let subscribers = subscribers.get_mut(&event_kind);

                        if subscribers.is_empty() {
                            let endpoint_str = event_kind.to_string();

                            let command = format!("[{}, \"{endpoint_str}\"]", code.clone() as u8);

                            println!("{command}");

                            let continues = send_command(error_handler, stream, tls, command);
                            if !continues {
                                break 'outer;
                            }
                        }

                        let mut idx = subscribers.len();

                        for (first_none_idx, maybe_subscriber) in subscribers.iter_mut().enumerate()
                        {
                            if maybe_subscriber.is_none() {
                                idx = first_none_idx;
                                break;
                            }
                        }

                        if idx == subscribers.len() {
                            subscribers.push(Some(subscriber));
                        } else {
                            subscribers[idx] = Some(subscriber);
                        }
                    }
                    ChannelMessage::Unsubscribe(subscriber_id, event_kind) => {
                        let subscribers = subscribers.get_mut(&event_kind);

                        if subscribers.iter().flatten().count() == 0 {
                            let unsub = format!(
                                "[{}, \"{}\"]",
                                RequestType::Unsubscribe as u8,
                                event_kind.to_string()
                            );
                            let continues = send_command(error_handler, stream, tls, unsub);
                            if !continues {
                                break;
                            }
                        }

                        subscribers[subscriber_id.0] = None;
                    }
                    ChannelMessage::Abort => {
                        break 'outer;
                    }
                }
            };

            let read = stream.read();

            if let Ok(maybe_message) = read.no_block() {
                if let Some(message) = maybe_message {
                    let data = message.into_data();
                    if !data.is_empty() {
                        let maybe_json = serde_json::from_slice::<Event>(&data);

                        match maybe_json {
                            Ok(json) => {
                                let subscribers = subscribers.get_mut(&json.1);

                                for subscriber in subscribers.iter_mut().flatten() {
                                    let mut control = subscriber.on_event(&json);
                                    #[rustfmt::skip]
                                    let continues = budget_recursive(&mut control, tls, stream, error_handler);
                                    if !continues {
                                        break 'outer;
                                    }
                                }
                            }
                            Err(e) => {
                                let mut control = error_handler.on_error(e.into());

                                #[rustfmt::skip]
                                let continues = budget_recursive(&mut control, tls, stream, error_handler);

                                if !continues {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        } else {
            match connect(tls) {
                Ok(stream) => maybe_stream = Some(stream),
                Err(e) => {
                    let control = error_handler.on_error(e);

                    if control == ControlFlow::Break(()) {
                        break;
                    }
                }
            }
        }
    }
}

fn send_command(
    error_handler: &mut dyn ErrorHandler,
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
    f: &mut dyn ErrorHandler,
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

#[derive(Debug)]
pub enum WebsocketError {
    Tungstenite(tungstenite::Error),
    ProcessInfo(process_info::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
}

impl Display for WebsocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            WebsocketError::Tungstenite(e) => e.to_string(),
            WebsocketError::ProcessInfo(e) => e.to_string(),
            WebsocketError::SerdeJson(e) => e.to_string(),
            WebsocketError::Io(e) => e.to_string(),
        };

        f.write_str(&string)
    }
}

impl std::error::Error for WebsocketError {}

impl From<std::io::Error> for WebsocketError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<tungstenite::Error> for WebsocketError {
    fn from(value: tungstenite::Error) -> Self {
        Self::Tungstenite(value)
    }
}

impl From<serde_json::Error> for WebsocketError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<process_info::Error> for WebsocketError {
    fn from(value: process_info::Error) -> Self {
        Self::ProcessInfo(value)
    }
}

fn connect(tls: &Arc<ClientConfig>) -> Result<WebSocketStream, WebsocketError> {
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

    stream_inner.sock.sock.set_nonblocking(true).unwrap();

    Ok(stream)
}

#[cfg(test)]
mod test {
    use crate::ws::types::Event;
    use serde_json::json;

    #[test]
    fn test_deserialize() {
        let json = json!([5, "OnJsonApiEvent", {
            "data": {},
            "eventType": "Create",
            "uri": "/Example/Uri"
        }]);
        let event: Event = serde_json::from_value(json).unwrap();
        println!("{event:?}");
    }
}

//! Module containing all the data on the websocket LCU bindings

pub mod types;
mod utils;

use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{ops::ControlFlow, sync::Arc, thread};

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use tungstenite::handshake::client::Request;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{client::IntoClientRequest, http::HeaderValue, Connector, Message, WebSocket};

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::ws::types::{Event, EventKind, RequestType};
use crate::ws::utils::EventMap;
use crate::{
    utils::{process_info::get_running_client, setup_tls::connector},
    Error,
};

type WebSocketStream = WebSocket<MaybeTlsStream<StreamOwned<ClientConnection, TcpStream>>>;

/// Struct representing a connection to the LCU websocket
pub struct LCUWebSocket {
    ws_sender: Sender<ChannelMessage>,
    handle: JoinHandle<()>,
    id_free_list: EventMap<(usize, Vec<usize>)>,
    url: String,
    auth_header: String,
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
    fn on_error(&mut self, error: Error) -> ControlFlow<(), Flow>;
}

/// This is a zero sized struct which calls `eprintln!()` and then breaks on error
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn on_error(&mut self, error: Error) -> ControlFlow<(), Flow> {
        eprintln!("{error}");
        ControlFlow::Break(())
    }
}

enum ChannelMessage {
    Subscribe(RequestType, EventKind, Box<dyn Subscriber>),
    Unsubscribe(SubscriberID, EventKind),
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SubscriberID(usize);

impl LCUWebSocket {
    /// Creates a new connection to the LCU websocket using the default error handler
    ///
    /// # Errors
    /// This function will return an error if the LCU is not running,
    /// or if it cannot connect to the websocket
    ///
    /// # Panics
    ///
    /// If the auth header returned is somehow invalid (though I have not seen this in practice)
    pub fn new() -> Result<Self, Error> {
        Self::new_with_error_handler(DefaultErrorHandler)
    }

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
    ) -> Result<Self, Error> {
        let tls = connector();
        let tls = Arc::new(tls.clone());
        let (url, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;

        let str_req = format!("wss://{url}");

        let auth_header = HeaderValue::from_str(&auth).unwrap();

        let mut request = str_req.as_str().into_client_request()?;

        request.headers_mut().insert("Authorization", auth_header);

        let (ws_sender, ws_receiver) = std::sync::mpsc::channel::<ChannelMessage>();

        let url_clone = url.clone();

        let handle = thread::spawn(move || {
            let ws_receiver = ws_receiver;
            let request = request;
            let url = url_clone;
            let tls = tls;

            event_loop(&request, error_handler, &ws_receiver, &tls, &url);
        });

        Ok(Self {
            ws_sender,
            handle,
            url,
            id_free_list: EventMap::new(),
            auth_header: auth,
        })
    }

    #[must_use]
    /// Returns a reference to the URL in use
    pub fn url(&self) -> &str {
        &self.url
    }

    #[must_use]
    /// Returns a reference to the auth header in use
    pub fn auth_header(&self) -> &str {
        &self.auth_header
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
    pub fn join(self) -> Option<()> {
        self.handle.join().ok()
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

//noinspection DuplicatedCode
#[allow(clippy::too_many_lines)]
fn event_loop(
    request: &Request,
    mut error_handler: impl ErrorHandler,
    ws_receiver: &Receiver<ChannelMessage>,
    tls: &Arc<ClientConfig>,
    url: &str,
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
                }
            };

            let read = stream.read();

            if let Ok(message) = read {
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
            }
        } else {
            use std::str::FromStr;

            const TIMEOUT: Duration = Duration::from_millis(100);

            let socket_addr = SocketAddr::from_str(url).unwrap();

            // TODO: This is where the real connection error would be, and it needs to be handled
            let tcp_stream = TcpStream::connect_timeout(&socket_addr, TIMEOUT).expect("TODO");
            tcp_stream.set_read_timeout(Some(TIMEOUT)).unwrap();

            let addr = ServerName::IpAddress(socket_addr.ip().into());

            let client_connection = ClientConnection::new(tls.clone(), addr).unwrap();
            let rustls_stream = StreamOwned::new(client_connection, tcp_stream);

            let (stream, _) = tungstenite::client_tls_with_config(
                request.clone(),
                rustls_stream,
                None,
                Some(Connector::Rustls(tls.clone())),
            )
            .expect("The TLS handshake should never fail");

            maybe_stream = Some(stream);
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
        println!("{e}");
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

        let rec = reconnect(tls, stream);
        if let Err(e) = rec {
            *c = f.on_error(e);
        } else {
            break;
        }
    }

    true
}

// TODO: This needs to become a more general "connect" function
// TODO: TCP stream needs a timeout
// TODO: TCP stream should be set not to block
// TODO: Code cleanup
fn reconnect(tls: &Arc<ClientConfig>, web_socket: &mut WebSocketStream) -> Result<(), Error> {
    let (url, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;

    let tcp_stream = TcpStream::connect(&url).expect("Need to handle");
    let client_connection = ClientConnection::new(tls.clone(), "127.0.0.1".try_into().unwrap())
        .expect("Would have already failed?");
    let rustls_stream = StreamOwned::new(client_connection, tcp_stream);

    let str_req = format!("wss://{url}");

    let auth_header = HeaderValue::from_str(&auth).unwrap();

    let mut request = str_req.into_client_request()?;

    request.headers_mut().insert("Authorization", auth_header);

    let (new_socket, _) =
        tungstenite::client_tls(request.clone(), rustls_stream).expect("Need to fix this too");

    *web_socket = new_socket;

    Ok(())
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

//! Module containing all the data on the websocket LCU bindings

mod types;

use std::collections::HashMap;
use std::{ops::ControlFlow, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rustls::ClientConfig;
use tokio::sync::mpsc::Receiver;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, http::HeaderValue, Message},
    Connector, MaybeTlsStream, WebSocketStream,
};

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::ws::types::{Event, EventKind, RequestType};
use crate::{
    utils::{process_info::get_running_client, setup_tls::setup_tls_connector},
    Error,
};

/// Struct representing a connection to the LCU websocket
pub struct LCUWebSocket {
    ws_sender: UnboundedSender<ChannelMessage>,
    handle: JoinHandle<()>,
    id_receiver: Receiver<usize>,
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
    fn on_error(&mut self, error: Error) -> ControlFlow<(), Flow> {
        panic!("{error:?}")
    }
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {}

type Writer = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type Reader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

enum ChannelMessage {
    Subscribe(RequestType, EventKind, Box<dyn Subscriber>),
    Unsubscribe(SubscriberID, EventKind),
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SubscriberID(usize);

impl LCUWebSocket {
    #[allow(clippy::too_many_lines)]
    /// Creates a new connection to the LCU websocket
    ///
    /// # Errors
    /// This function will return an error if the LCU is not running,
    /// or if it cannot connect to the websocket
    ///
    /// # Panics
    ///
    /// If the auth header returned is somehow invalid (though I have not seen this in practice)
    pub async fn new(error_handler: Option<impl ErrorHandler + 'static>) -> Result<Self, Error> {
        let tls = setup_tls_connector();
        let tls = Arc::new(tls);
        let connector = Some(Connector::Rustls(tls.clone()));
        let (url, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;
        let str_req = format!("wss://{url}");

        let auth_header = HeaderValue::from_str(&auth).unwrap();

        let mut request = str_req.as_str().into_client_request()?;

        request.headers_mut().insert("Authorization", auth_header);

        let (stream, _) = connect_async_tls_with_config(request, None, false, connector).await?;

        let (ws_sender, mut ws_receiver) = mpsc::unbounded_channel::<ChannelMessage>();
        let (id_sender, id_receiver) = mpsc::channel(10);
        let (mut write, mut read) = stream.split();

        let handle = tokio::spawn(async move {
            type SubscriberMap = HashMap<EventKind, Vec<Option<Box<dyn Subscriber>>>>;

            let mut subscribers: SubscriberMap = HashMap::new();
            let mut error_handler = error_handler;
            let error_handler: &mut dyn ErrorHandler = if let Some(handler) = &mut error_handler {
                handler
            } else {
                &mut DefaultErrorHandler
            };

            'outer: loop {
                if let Ok(message) = ws_receiver.try_recv() {
                    match message {
                        ChannelMessage::Subscribe(code, endpoint, subscriber) => {
                            if let Some(subscribers) = subscribers.get_mut(&endpoint) {
                                let mut idx = subscribers.len();

                                for (first_none_idx, maybe_subscriber) in
                                    subscribers.iter_mut().enumerate()
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

                                id_sender.send(idx).await.unwrap();
                            } else {
                                let endpoint_str = endpoint.to_string();

                                let command =
                                    format!("[{}, \"{endpoint_str}\"]", code.clone() as u8);

                                if let Err(e) = write.send(command.into()).await {
                                    let mut control = error_handler.on_error(e.into());

                                    #[rustfmt::skip]
                                        let continues = budget_recursive(&mut control, &tls, &mut write, &mut read, error_handler).await;

                                    if !continues {
                                        break;
                                    }
                                };

                                subscribers.insert(endpoint, vec![Some(subscriber)]);
                            }
                        }
                        ChannelMessage::Unsubscribe(subscriber_id, event_kind) => {
                            if let Some(subscribers) = subscribers.get_mut(&event_kind) {
                                if subscribers.iter().flatten().count() == 0 {
                                    let unsub = format!(
                                        "[{}, \"{}\"]",
                                        RequestType::Unsubscribe as u8,
                                        event_kind.to_string()
                                    );
                                    if let Err(e) = write.send(unsub.into()).await {
                                        let mut control = error_handler.on_error(e.into());

                                        #[rustfmt::skip]
                                            let continues = budget_recursive(&mut control, &tls, &mut write, &mut read, error_handler).await;

                                        if !continues {
                                            break;
                                        }
                                    }
                                }

                                subscribers[subscriber_id.0] = None;
                            }
                        }
                    }
                };

                if let Some(Ok(message)) = read.next().await {
                    let data = message.into_data();
                    if !data.is_empty() {
                        let maybe_json = serde_json::from_slice::<Event>(&data);
                        match maybe_json {
                            Ok(json) => {
                                if let Some(subscribers) = subscribers.get_mut(&json.1) {
                                    for subscriber in subscribers.iter_mut().flatten() {
                                        let mut control = subscriber.on_event(&json);

                                        #[rustfmt::skip]
                                            let continues = budget_recursive(&mut control, &tls, &mut write, &mut read, error_handler).await;

                                        if !continues {
                                            break 'outer;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let mut control = error_handler.on_error(e.into());

                                #[rustfmt::skip]
                                let continues = budget_recursive(&mut control, &tls, &mut write, &mut read, error_handler).await;

                                if !continues {
                                    break;
                                }
                            }
                        }
                    }
                }

                println!("H");
            }
        });

        Ok(Self {
            ws_sender,
            handle,
            url,
            auth_header: auth,
            id_receiver,
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
        self.ws_sender
            .send(ChannelMessage::Subscribe(
                RequestType::Subscribe,
                event_kind,
                Box::new(subscriber),
            ))
            .ok()?;
        let id = self.id_receiver.blocking_recv()?;
        Some(SubscriberID(id))
    }

    pub async fn subscribe_async(
        &mut self,
        event_kind: EventKind,
        subscriber: impl Subscriber + 'static,
    ) -> Option<SubscriberID> {
        self.ws_sender
            .send(ChannelMessage::Subscribe(
                RequestType::Subscribe,
                event_kind,
                Box::new(subscriber),
            ))
            .ok()?;
        let id = self.id_receiver.recv().await?;
        Some(SubscriberID(id))
    }

    /// Unsubscribe to a new API event
    ///
    /// If all subscribers have been removed, this will unsubscribe from the event as a whole
    ///
    /// Returns `None` if the connection to the websocket was already closed
    pub fn unsubscribe(&mut self, endpoint: EventKind, id: SubscriberID) -> Option<()> {
        self.ws_sender
            .send(ChannelMessage::Unsubscribe(id, endpoint))
            .ok()
    }

    /// Terminate the event loop
    pub fn terminate(&self) {
        self.handle.abort();
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

async fn budget_recursive(
    c: &mut ControlFlow<(), Flow>,
    tls: &Arc<ClientConfig>,
    write: &mut Writer,
    read: &mut Reader,
    f: &mut dyn ErrorHandler,
) -> bool {
    while *c != ControlFlow::Continue(Flow::Continue) {
        if *c == ControlFlow::Break(()) {
            return false;
        }

        let tls = tls.clone();
        let rec = reconnect(tls, write, read).await;
        if let Err(e) = rec {
            *c = f.on_error(e);
        } else {
            break;
        }
    }

    true
}

async fn reconnect(
    tls: Arc<ClientConfig>,
    write: &mut Writer,
    read: &mut Reader,
) -> Result<(), Error> {
    let (url, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;
    let str_req = format!("wss://{url}");

    let auth_header = HeaderValue::from_str(&auth).unwrap();

    let mut request = str_req.into_client_request()?;

    request.headers_mut().insert("Authorization", auth_header);

    let connector = Connector::Rustls(tls.clone());
    let (stream, _) = connect_async_tls_with_config(request, None, false, Some(connector)).await?;
    (*write, *read) = stream.split();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{DefaultErrorHandler, Flow, LCUWebSocket, Subscriber};
    use crate::ws::types::{Event, EventKind};
    use serde_json::json;
    use std::ops::ControlFlow;
    use std::time::Duration;

    #[tokio::test]
    async fn it_inits() {
        struct EventCounter(u32);

        impl Subscriber for EventCounter {
            fn on_event(&mut self, event: &Event) -> ControlFlow<(), Flow> {
                println!("{event:?}");
                self.0 += 1;
                println!("{}", self.0);

                ControlFlow::Continue(Flow::Continue)
            }
        }

        let mut ws_client = LCUWebSocket::new(None::<DefaultErrorHandler>)
            .await
            .unwrap();

        ws_client.subscribe_async(EventKind::JsonApiEvent, EventCounter(0)).await;

        while !ws_client.is_finished() {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

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

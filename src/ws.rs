use std::collections::HashSet;

use futures_util::{SinkExt, Stream, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, http::HeaderValue},
    Connector,
};

use crate::{
    utils::{process_info::get_port_and_auth, setup_tls::TLS_CONNECTOR},
    Error,
};

/// ```rs
/// async fn web_socket() {
///     use irelia::ws::LCUWebSocket;
///
///     let ws = LCUWebSocket::new().unwrap();
///     ws.subscribe("OnJsonApiEvent");
///     loop {
///         let data = ws.client_reciver.unwrap();
///     }
/// }
/// ```
pub struct LCUWebSocket {
    ws_sender: UnboundedSender<(RequestType, EventType)>,
    handle: JoinHandle<()>,
    client_reciver: UnboundedReceiver<Result<Value, Error>>,
}

#[derive(PartialEq, Clone)]
pub enum RequestType {
    Welcome = 0,
    Prefix = 1,
    Call = 2,
    CallResult = 3,
    CallError = 4,
    Subscribe = 5,
    Unsubscribe = 6,
    Publish = 7,
    Event = 8,
}

#[derive(Eq, Hash, PartialEq, Clone)]
/// Different event types that can be passed to the
/// subscribe and unsubscribe methods.
pub enum EventType {
    OnJsonApiEvent,
    OnLcdEvent,
    OnJsonApiEventCallback(String),
    OnLcdEventCallback(String),
}

impl LCUWebSocket {
    /// Connect to the LCU Web Socket, Error if it fails or the client is not running
    ///
    /// ## Example:
    /// ```rs
    /// fn main() {
    ///     let mut websocket = ws::LCUWebSocket::new().await.unwrap();
    ///
    ///     websocket.subscribe(ws::EventType::OnJsonApiEventCallback(String::from(
    ///         "/lol-champ-select/v1/current-champion",
    ///     )));
    ///
    ///     loop {
    ///         if let Some(event) = websocket.next().await {
    ///             println!("{:?}", event);
    ///         }
    ///     }
    /// }```
    ///
    pub async fn new() -> Result<Self, Error> {
        let tls = TLS_CONNECTOR.clone();
        let connector = Connector::NativeTls(tls);
        let port_pass = get_port_and_auth()?;
        let mut url = format!("wss://127.0.0.1:{}", port_pass.0)
            .into_client_request()
            .unwrap();
        url.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&format!("Basic {}", port_pass.1)).unwrap(),
        );
        let Ok((stream, _)) = connect_async_tls_with_config(url, None, Some(connector)).await else {
            return Err(Error::LCUStoppedRunning);
        };
        let (mut write, mut read) = stream.split();
        let (ws_sender, mut ws_reciver) = mpsc::unbounded_channel::<(RequestType, EventType)>();
        let (client_sender, client_reciver) = mpsc::unbounded_channel::<Result<Value, Error>>();

        let handle: JoinHandle<()> = tokio::spawn(async move {
            let mut active_commands: HashSet<String> = HashSet::new();
            loop {
                if let Ok((code, endpoint)) = ws_reciver.try_recv() {
                    let endpoint = match endpoint {
                        EventType::OnJsonApiEvent => String::from("OnJsonApiEvent"),
                        EventType::OnLcdEvent => String::from("OnLcdEvent"),
                        EventType::OnJsonApiEventCallback(callback) => {
                            format!("OnJsonApiEvent{}", callback.replace('/', "_"))
                        }
                        EventType::OnLcdEventCallback(callback) => {
                            format!("OnLcdEvent{}", callback.replace('/', "_"))
                        }
                    };

                    let command = format!("[{}, \"{endpoint}\"]", code.clone() as u8);

                    if code == RequestType::Subscribe {
                        active_commands.insert(endpoint.clone());
                    } else if code == RequestType::Unsubscribe {
                        active_commands.remove(&endpoint);
                    };

                    if write.send(command.into()).await.is_err() {
                        client_sender.send(Err(Error::LCUStoppedRunning)).unwrap();
                    };
                };

                if let Some(Ok(data)) = read.next().await {
                    if let Ok(json) = &serde_json::from_slice::<Value>(&data.into_data()) {
                        if let Some(endpoint) = json[1].as_str() {
                            if active_commands.contains(endpoint) {
                                client_sender.send(Ok(json.to_owned())).unwrap();
                            }
                        } else {
                            client_sender.send(Ok(json.to_owned())).unwrap();
                        }
                    };
                };
            }
        });

        Ok(Self {
            ws_sender,
            handle,
            client_reciver,
        })
    }

    /// Subscribe to a new API event
    pub fn subscribe(&mut self, endpoint: EventType) {
        self.request(RequestType::Subscribe, endpoint);
    }

    /// Unsubscribe to a new API event
    pub fn unsubscribe(&mut self, endpoint: EventType) {
        self.request(RequestType::Unsubscribe, endpoint);
    }

    /// Terminate the event loop
    pub fn terminate(&self) {
        self.handle.abort();
    }

    /// Allows you to make a gneric
    /// request to the websocket socket
    pub fn request(&mut self, code: RequestType, endpoint: EventType) {
        let _ = &self.ws_sender.send((code, endpoint));
    }
}

impl Stream for LCUWebSocket {
    type Item = Result<Value, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.client_reciver.poll_recv(cx)
    }
}

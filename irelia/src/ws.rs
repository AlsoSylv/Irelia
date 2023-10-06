//! Module containing all the data on the websocket LCU bindings

use std::{collections::HashSet, sync::Arc};

use futures_util::{SinkExt, Stream, StreamExt};
use serde_json::Value;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, http::HeaderValue},
    Connector,
};

use crate::{
    utils::{process_info::get_running_client, setup_tls::setup_tls_connector},
    LCUError,
};

/// Different LCU WebSocket request types
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
    OnLcdsEvent,
    OnLog,
    OnRegionLocaleChanged,
    OnServiceProxyAsyncEvent,
    OnServiceProxyMethodEvent,
    OnServiceProxyUuidEvent,
    OnJsonApiEventCallback(String),
    OnLcdsEventCallback(String),
}

/// Struct representing a connection to the LCU websocket
pub struct LCUWebSocket {
    ws_sender: UnboundedSender<(RequestType, EventType)>,
    handle: JoinHandle<()>,
    client_reciever: UnboundedReceiver<Result<Vec<Value>, LCUError>>,
    url: String,
    auth_header: String,
}

impl LCUWebSocket {
    /// Creates a new connection to the LCU websocket
    pub async fn new() -> Result<Self, LCUError> {
        let tls = setup_tls_connector();
        let connector = Connector::Rustls(Arc::new(tls));
        let (url, auth_header) = get_running_client()?;
        let mut req = format!("wss://{}", url).into_client_request().unwrap();
        req.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );

        let (stream, _) = connect_async_tls_with_config(req, None, false, Some(connector))
            .await
            .map_err(LCUError::WebsocketError)?;

        let (mut write, mut read) = stream.split();
        let (ws_sender, mut ws_reciever) = mpsc::unbounded_channel::<(RequestType, _)>();
        let (client_sender, client_reciever) = mpsc::unbounded_channel();

        let handle = tokio::spawn(async move {
            let mut active_commands = HashSet::new();
            loop {
                if let Ok((code, endpoint)) = ws_reciever.try_recv() {
                    let endpoint = match endpoint {
                        EventType::OnJsonApiEvent => String::from("OnJsonApiEvent"),
                        EventType::OnLcdsEvent => String::from("OnLcdsEvent"),
                        EventType::OnLog => String::from("OnLog"),
                        EventType::OnRegionLocaleChanged => String::from("OnRegionLocaleChanged"),
                        EventType::OnServiceProxyAsyncEvent => {
                            String::from("OnServiceProxyAsyncEvent")
                        }
                        EventType::OnServiceProxyMethodEvent => {
                            String::from("OnServiceProxyMethodEvent")
                        }
                        EventType::OnServiceProxyUuidEvent => {
                            String::from("OnServiceProxyUuidEvent")
                        }
                        EventType::OnJsonApiEventCallback(callback) => {
                            format!("OnJsonApiEvent{}", callback.replace('/', "_"))
                        }
                        EventType::OnLcdsEventCallback(callback) => {
                            format!("OnLcdsEvent{}", callback.replace('/', "_"))
                        }
                    };

                    let command = format!("[{}, \"{endpoint}\"]", code.clone() as u8);

                    if code == RequestType::Subscribe {
                        active_commands.insert(endpoint.clone());
                    } else if code == RequestType::Unsubscribe {
                        active_commands.remove(&endpoint);
                    };

                    if write.send(command.into()).await.is_err() {
                        client_sender
                            .send(Err(LCUError::LCUProcessNotRunning))
                            .unwrap();
                    };
                };

                if let Some(Ok(data)) = read.next().await {
                    println!("A");
                    if let Ok(json) = &serde_json::from_slice::<Vec<Value>>(&data.into_data()) {
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
            client_reciever,
            url,
            auth_header,
        })
    }

    /// Returns a reference to the URL in use
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns a reference to the auth header in use
    pub fn auth_header(&self) -> &str {
        &self.auth_header
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
    type Item = Result<Vec<Value>, LCUError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.client_reciever.poll_recv(cx)
    }
}

#[cfg(test)]
mod test {
    use futures_util::StreamExt;
    use tokio;

    use super::LCUWebSocket;

    #[tokio::test]
    #[ignore]
    async fn it_inits() {
        let mut _ws_client = LCUWebSocket::new().await.unwrap();
        _ws_client.subscribe(crate::ws::EventType::OnJsonApiEvent);

        loop {
            while let Some(event) = _ws_client.next().await {
                println!("{:?}", event)
            }
        }
    }
}

//! Module containing all the data on the websocket LCU bindings

use std::{collections::HashSet, ops::ControlFlow, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rustls::ClientConfig;
use serde_json::Value;
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
    url: String,
    auth_header: String,
}

#[derive(PartialEq)]
pub enum Flow {
    TryReconnect,
    Continue,
}

impl LCUWebSocket {
    /// Creates a new connection to the LCU websocket
    pub async fn new(
        f: impl Fn(Result<&[Value], LCUError>) -> ControlFlow<(), Flow> + Send + 'static,
    ) -> Result<Self, LCUError> {
        let tls = setup_tls_connector();
        let tls = Arc::new(tls);
        let connector = Connector::Rustls(tls.clone());
        let (url, auth_header) = get_running_client()?;
        let str_req = format!("wss://{}", url);
        let mut req = str_req.as_str().into_client_request().unwrap();
        req.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );

        let (stream, _) = connect_async_tls_with_config(req, None, false, Some(connector))
            .await
            .map_err(LCUError::WebsocketError)?;

        let (ws_sender, mut ws_reciever) = mpsc::unbounded_channel::<(RequestType, _)>();

        let handle = tokio::spawn(async move {
            let str_req = str_req;
            let mut active_commands = HashSet::new();
            let (mut write, mut read) = stream.split();

            'outer: loop {
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
                        let mut c = f(Err(LCUError::LCUProcessNotRunning));
                        while c != ControlFlow::Continue(Flow::Continue) {
                            if c == ControlFlow::Continue(Flow::TryReconnect) {
                                let tls = tls.clone();
                                let rec = reconnect(&str_req, tls, &mut write, &mut read).await;
                                if let Err(e) = rec {
                                    c = f(Err(e));
                                } else {
                                    break;
                                }
                            } else {
                                break 'outer;
                            }
                        }
                    };
                };

                if let Some(Ok(data)) = read.next().await {
                    if let Ok(json) = &serde_json::from_slice::<Vec<Value>>(&data.into_data()) {
                        let json = if let Some(endpoint) = json[1].as_str() {
                            if active_commands.contains(endpoint) {
                                json
                            } else {
                                continue;
                            }
                        } else {
                            json
                        };

                        let mut c = f(Ok(json));
                        while c != ControlFlow::Continue(Flow::Continue) {
                            if c == ControlFlow::Continue(Flow::TryReconnect) {
                                let tls = tls.clone();
                                let rec = reconnect(&str_req, tls, &mut write, &mut read).await;
                                if let Err(e) = rec {
                                    c = f(Err(e));
                                } else {
                                    break;
                                }
                            } else {
                                break 'outer;
                            }
                        }
                    };
                };
            }
        });

        Ok(Self {
            ws_sender,
            handle,
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

    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    /// Allows you to make a gneric
    /// request to the websocket socket
    pub fn request(&mut self, code: RequestType, endpoint: EventType) {
        let _ = &self.ws_sender.send((code, endpoint));
    }
}

async fn reconnect(
    str_req: &str,
    tls: Arc<ClientConfig>,
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: &mut SplitStream<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    >,
) -> Result<(), LCUError> {
    let req = str_req.into_client_request().unwrap();
    let connector = Connector::Rustls(tls.clone());
    let (stream, _) = connect_async_tls_with_config(req, None, false, Some(connector))
        .await
        .map_err(LCUError::WebsocketError)?;
    (*write, *read) = stream.split();
    Ok(())
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::LCUWebSocket;

    // #[ignore = "This does not need to be run often"]
    #[tokio::test]
    async fn it_inits() {
        let mut ws_client = LCUWebSocket::new(|values| {
            println!("{values:?}");
            std::ops::ControlFlow::Continue(crate::ws::Flow::Continue)
        })
        .await
        .unwrap();
        ws_client.subscribe(crate::ws::EventType::OnJsonApiEvent);

        while !ws_client.is_finished() {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

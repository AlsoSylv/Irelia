//! Module containing all the data on the websocket LCU bindings

use std::{collections::HashSet, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
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
    utils::{process_info::get_running_client, setup_tls::TLS_CONFIG},
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

pub enum WSControl {
    Refresh,
    Continue,
}

/// Struct representing a connection to the LCU websocket
pub struct LCUWebSocket {
    ws_sender: UnboundedSender<(RequestType, EventType)>,
    handle: JoinHandle<()>,
    url: String,
    auth_header: String,
}

impl LCUWebSocket {
    /// Creates a new connection to the LCU websocket
    pub async fn new<F>(event_loop: F) -> Result<Self, LCUError>
    where
        F: Fn(Result<&Vec<Value>, LCUError>) -> WSControl + Send + Sync + 'static,
    {
        let tls = TLS_CONFIG.clone();
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
                        let mut last_error = Some(LCUError::LCUProcessNotRunning);
                        handle_event_loop(
                            &mut last_error,
                            &event_loop,
                            &mut write,
                            &mut read,
                            None,
                        )
                        .await;
                    };
                };

                if let Some(Ok(data)) = read.next().await {
                    let mut last_error = None;
                    if let Ok(json) = &serde_json::from_slice::<Vec<Value>>(&data.into_data()) {
                        if let Some(endpoint) = json[1].as_str() {
                            if active_commands.contains(endpoint) {
                                handle_event_loop(
                                    &mut last_error,
                                    &event_loop,
                                    &mut write,
                                    &mut read,
                                    Some(json),
                                )
                                .await;
                            }
                        } else {
                            handle_event_loop(
                                &mut last_error,
                                &event_loop,
                                &mut write,
                                &mut read,
                                Some(json),
                            )
                            .await;
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

    /// Allows you to make a gneric
    /// request to the websocket socket
    pub fn request(&mut self, code: RequestType, endpoint: EventType) {
        let _ = &self.ws_sender.send((code, endpoint));
    }
}

async fn handle_event_loop<F>(
    last_error: &mut Option<LCUError>,
    event_loop: &F,
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    value: Option<&Vec<Value>>,
) where
    F: Fn(Result<&Vec<Value>, LCUError>) -> WSControl + Send + Sync + 'static,
{
    'handle_event_loop: loop {
        if let Some(err) = last_error.take() {
            match event_loop(Err(err)) {
                WSControl::Refresh => match get_running_client() {
                    Ok((url, auth)) => {
                        let tls = TLS_CONFIG.clone();
                        let connector = Connector::Rustls(Arc::new(tls));
                        let mut req = format!("wss://{}", url).into_client_request().unwrap();

                        req.headers_mut()
                            .insert("Authorization", HeaderValue::from_str(&auth).unwrap());

                        let res =
                            connect_async_tls_with_config(req, None, false, Some(connector)).await;

                        match res {
                            Ok((stream, _)) => {
                                (*write, *read) = stream.split();
                                break 'handle_event_loop;
                            }
                            Err(err) => {
                                *last_error = Some(LCUError::WebsocketError(err));
                                continue 'handle_event_loop;
                            }
                        }
                    }
                    Err(err) => {
                        *last_error = Some(err);
                        continue 'handle_event_loop;
                    }
                },
                WSControl::Continue => break 'handle_event_loop,
            };
        } else if let Some(json) = value {
            match event_loop(Ok(json)) {
                WSControl::Refresh => match get_running_client() {
                    Ok((url, auth)) => {
                        let tls = TLS_CONFIG.clone();
                        let connector = Connector::Rustls(Arc::new(tls));
                        let mut req = format!("wss://{}", url).into_client_request().unwrap();

                        req.headers_mut()
                            .insert("Authorization", HeaderValue::from_str(&auth).unwrap());

                        let res =
                            connect_async_tls_with_config(req, None, false, Some(connector)).await;

                        match res {
                            Ok((stream, _)) => {
                                (*write, *read) = stream.split();
                                break 'handle_event_loop;
                            }
                            Err(err) => {
                                *last_error = Some(LCUError::WebsocketError(err));
                                continue 'handle_event_loop;
                            }
                        }
                    }
                    Err(err) => {
                        *last_error = Some(err);
                        continue 'handle_event_loop;
                    }
                },
                WSControl::Continue => break 'handle_event_loop,
            };
        } else {
            break 'handle_event_loop;
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use tokio;

    use super::LCUWebSocket;

    #[tokio::test]
    #[ignore]
    async fn it_inits() {
        let _ws_client = LCUWebSocket::new(|vec| {
            match vec {
                Ok(_events) => {
                    // Do event handling here
                    crate::ws::WSControl::Continue
                }
                Err(err) => match err {
                    crate::LCUError::SerdeJsonError(err) => panic!("{}", err),
                    crate::LCUError::WebsocketError(err) => panic!("{}", err),
                    crate::LCUError::LCUProcessNotRunning => crate::ws::WSControl::Refresh,
                    crate::LCUError::PortNotFound => crate::ws::WSControl::Refresh,
                    crate::LCUError::AuthTokenNotFound => crate::ws::WSControl::Refresh,
                },
            }
        })
        .await
        .unwrap();
    }
}

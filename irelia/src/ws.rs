//! Module containing all the data on the websocket LCU bindings

use std::borrow::Cow;
use std::fmt::Formatter;
use std::{ops::ControlFlow, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rustls::ClientConfig;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize};
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

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::{
    utils::{process_info::get_running_client, setup_tls::setup_tls_connector},
    Error,
};

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

#[derive(Debug, Deserialize)]
pub struct Event(pub RequestType, pub EventType, pub Value);

impl LCUWebSocket {
    /// Creates a new connection to the LCU websocket
    ///
    /// # Errors
    /// This function will return an error if the LCU is not running,
    /// or if it cannot connect to the websocket
    ///
    /// # Panics
    ///
    /// If the auth header returned is somehow invalid (though I have not seen this in practice)
    pub async fn new(
        f: impl Fn(Result<Event, Error>) -> ControlFlow<(), Flow> + Send + Sync + 'static,
    ) -> Result<Self, Error> {
        let tls = setup_tls_connector();
        let tls = Arc::new(tls);
        let connector = Some(Connector::Rustls(tls.clone()));
        let (url, auth) = get_running_client(GAME_PROCESS_NAME, CLIENT_PROCESS_NAME, false)?;
        let str_req = format!("wss://{url}");

        let auth_header = HeaderValue::from_str(&auth).unwrap();

        let mut request = str_req.as_str().into_client_request()?;

        request.headers_mut().insert("Authorization", auth_header);

        let (stream, _) = connect_async_tls_with_config(request, None, false, connector).await?;

        let (ws_sender, mut ws_receiver) = mpsc::unbounded_channel::<(RequestType, EventType)>();

        let handle = tokio::spawn(async move {
            let (mut write, mut read) = stream.split();

            loop {
                if let Ok((code, endpoint)) = ws_receiver.try_recv() {
                    let endpoint_str = endpoint.to_string();

                    let command = format!("[{}, \"{endpoint_str}\"]", code as u8);

                    if let Err(e) = write.send(command.into()).await {
                        let mut c = f(Err(e.into()));
                        if !budget_recursive(&mut c, &tls, &f, &mut write, &mut read).await {
                            break;
                        };
                    };
                };

                if let Some(Ok(data)) = read.next().await {
                    let data = data.into_data();
                    let maybe_json = serde_json::from_slice::<Event>(&data).map_err(Error::from);
                    let mut c = f(maybe_json);
                    if !budget_recursive(&mut c, &tls, &f, &mut write, &mut read).await {
                        break;
                    };
                };
            }
        });

        Ok(Self {
            ws_sender,
            handle,
            url,
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

    /// Subscribe to a new API event
    pub fn subscribe(&mut self, endpoint: EventType) {
        self.request(RequestType::Subscribe, endpoint);
    }

    /// Unsubscribe to a new API event
    ///
    /// Note: Just because you unsubscribe doesn't mean
    /// You will immediately stop receiving these events
    pub fn unsubscribe(&mut self, endpoint: EventType) {
        self.request(RequestType::Unsubscribe, endpoint);
    }

    /// Terminate the event loop
    pub fn terminate(&self) {
        self.handle.abort();
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    /// Allows you to make a generic
    /// request to the websocket socket
    pub fn request(&mut self, code: RequestType, endpoint: EventType) {
        let _ = &self.ws_sender.send((code, endpoint));
    }
}

async fn budget_recursive(
    c: &mut ControlFlow<(), Flow>,
    tls: &Arc<ClientConfig>,
    f: &(impl Fn(Result<Event, Error>) -> ControlFlow<(), Flow> + Sync + Send + 'static),
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> bool {
    while *c != ControlFlow::Continue(Flow::Continue) {
        if *c == ControlFlow::Continue(Flow::TryReconnect) {
            let tls = tls.clone();
            let rec = reconnect(tls, write, read).await;
            if let Err(e) = rec {
                *c = f(Err(e));
            } else {
                break;
            }
        } else {
            return false;
        }
    }

    true
}

async fn reconnect(
    tls: Arc<ClientConfig>,
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<(), Error> {
    let (url, auth) = get_running_client(GAME_PROCESS_NAME, CLIENT_PROCESS_NAME, false)?;
    let str_req = format!("wss://{url}");

    let auth_header = HeaderValue::from_str(&auth).unwrap();

    let mut request = str_req.as_str().into_client_request()?;

    request.headers_mut().insert("Authorization", auth_header);

    let connector = Connector::Rustls(tls.clone());
    let (stream, _) = connect_async_tls_with_config(request, None, false, Some(connector)).await?;
    (*write, *read) = stream.split();
    Ok(())
}


/// Different LCU websocket request types
#[derive(Debug, PartialEq, Clone)]
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

impl<'de> Deserialize<'de> for RequestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct U8Visitor;

        impl<'a> Visitor<'a> for U8Visitor {
            type Value = RequestType;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A u8, which maps to a request type")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(
                    match v {
                        0 => RequestType::Welcome,
                        1 => RequestType::Prefix,
                        2 => RequestType::Call,
                        3 => RequestType::CallResult,
                        4 => RequestType::CallError,
                        5 => RequestType::Subscribe,
                        6 => RequestType::Unsubscribe,
                        7 => RequestType::Publish,
                        8 => RequestType::Event,
                        _ => unreachable!()
                    }
                )
            }
        }

        deserializer.deserialize_u8(U8Visitor)
    }
}

impl Serialize for RequestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_u8(self.clone() as u8)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> Visitor<'de> for StringVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Expecting an LCU Event")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                if let Some((event, callback)) = v.split_once('_') {
                    match EventType::from_str(event) {
                        EventType::OnJsonApiEvent => {
                            Ok(EventType::OnJsonApiEventCallback(callback.to_string()))
                        }
                        EventType::OnLcdsEvent => {
                            Ok(EventType::OnLcdsEventCallback(callback.to_string()))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    Ok(EventType::from_str(v))
                }
            }
        }

        deserializer.deserialize_str(StringVisitor)
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EventType {
    fn to_string(&self) -> Cow<'static, str> {
        match self {
            EventType::OnJsonApiEvent => "OnJsonApiEvent".into(),
            EventType::OnLcdsEvent => "OnLcdsEvent".into(),
            EventType::OnLog => "OnLog".into(),
            EventType::OnRegionLocaleChanged => "OnRegionLocaleChanged".into(),
            EventType::OnServiceProxyAsyncEvent => "OnServiceProxyAsyncEvent".into(),
            EventType::OnServiceProxyMethodEvent => "OnServiceProxyMethodEvent".into(),
            EventType::OnServiceProxyUuidEvent => "OnServiceProxyUuidEvent".into(),
            EventType::OnJsonApiEventCallback(callback) => {
                format!("OnJsonApiEvent{}", callback.replace('/', "_")).into()
            }
            EventType::OnLcdsEventCallback(callback) => {
                format!("OnLcdsEvent{}", callback.replace('/', "_")).into()
            }
        }
    }

    fn from_str(event: &str) -> EventType {
        match event {
            "OnJsonApiEvent" => EventType::OnJsonApiEvent,
            "OnLcdsEvent" => EventType::OnLcdsEvent,
            "OnLog" => EventType::OnLog,
            "OnRegionLocaleChanged" => EventType::OnRegionLocaleChanged,
            "OnServiceProxyAsyncEvent" => EventType::OnServiceProxyAsyncEvent,
            "OnServiceProxyMethodEvent" => EventType::OnServiceProxyMethodEvent,
            "OnServiceProxyUuidEvent" => EventType::OnServiceProxyUuidEvent,
            event => unreachable!("{event}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::LCUWebSocket;
    use std::time::Duration;

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

//! Module containing all the data on the websocket LCU bindings

mod types;

use std::{ops::ControlFlow, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rustls::ClientConfig;
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
    ws_sender: UnboundedSender<(RequestType, EventKind)>,
    handle: JoinHandle<()>,
    url: String,
    auth_header: String,
}

#[derive(PartialEq)]
pub enum Flow {
    TryReconnect,
    Continue,
}

type Writer = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type Reader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Callback = dyn FnMut(Result<Event, Error>) -> ControlFlow<(), Flow> + Send + 'static;

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
        mut f: impl FnMut(Result<Event, Error>) -> ControlFlow<(), Flow> + Send + 'static,
    ) -> Result<Self, Error> {
        let tls = setup_tls_connector();
        let tls = Arc::new(tls);
        let connector = Some(Connector::Rustls(tls.clone()));
        let (url, auth) = get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, false)?;
        let str_req = format!("wss://{url}");

        let auth_header = HeaderValue::from_str(&auth).unwrap();

        let mut request = str_req.as_str().into_client_request()?;

        request.headers_mut().insert("Authorization", auth_header);

        let (stream, _) = connect_async_tls_with_config(request, None, false, connector).await?;

        let (ws_sender, mut ws_receiver) = mpsc::unbounded_channel::<(RequestType, EventKind)>();
        let (mut write, mut read) = stream.split();

        let handle = tokio::spawn(async move {
            loop {
                if let Ok((code, endpoint)) = ws_receiver.try_recv() {
                    let endpoint_str = endpoint.to_string();

                    let command = format!("[{}, \"{endpoint_str}\"]", code as u8);

                    let send_command = write.send(command.into()).await;

                    if let Err(e) = send_command {
                        let mut c = f(Err(e.into()));
                        if !budget_recursive(&mut c, &tls, &mut write, &mut read, &mut f).await {
                            break;
                        };
                    };
                };

                if let Some(Ok(message)) = read.next().await {
                    let data = message.into_data();
                    let maybe_json = serde_json::from_slice(&data).map_err(Error::from);
                    let mut c = f(maybe_json);
                    if !budget_recursive(&mut c, &tls, &mut write, &mut read, &mut f).await {
                        break;
                    };
                }
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
    pub fn subscribe(&mut self, endpoint: EventKind) {
        self.request(RequestType::Subscribe, endpoint);
    }

    /// Unsubscribe to a new API event
    ///
    /// Note: Just because you unsubscribe doesn't mean
    /// You will immediately stop receiving these events
    pub fn unsubscribe(&mut self, endpoint: EventKind) {
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
    pub fn request(&mut self, code: RequestType, endpoint: EventKind) {
        let _ = &self.ws_sender.send((code, endpoint));
    }
}

async fn budget_recursive(
    c: &mut ControlFlow<(), Flow>,
    tls: &Arc<ClientConfig>,
    write: &mut Writer,
    read: &mut Reader,
    f: &mut Callback,
) -> bool {
    while *c != ControlFlow::Continue(Flow::Continue) {
        if *c == ControlFlow::Break(()) {
            return false;
        }

        let tls = tls.clone();
        let rec = reconnect(tls, write, read).await;
        if let Err(e) = rec {
            *c = f(Err(e));
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
    use super::LCUWebSocket;
    use crate::ws::types::Event;
    use serde_json::json;
    use std::time::Duration;

    #[ignore = "This does not need to be run often"]
    #[tokio::test]
    async fn it_inits() {
        let mut ws_client = LCUWebSocket::new(Box::new(|values| {
            println!("{values:?}");
            std::ops::ControlFlow::Continue(crate::ws::Flow::Continue)
        }))
        .await
        .unwrap();
        ws_client.subscribe(crate::ws::EventKind::JsonApiEvent);

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

use std::collections::HashSet;

use futures_util::{SinkExt, Stream, StreamExt};
use serde_json::Value;
use tokio::spawn;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{tungstenite::client::IntoClientRequest, Connector};

use crate::utils::setup_tls::TLS_CONNECTOR;
use crate::{utils::process_info::get_port_and_auth, Error};

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
    ws_sender: UnboundedSender<(u8, String)>,
    handle: JoinHandle<()>,
    client_reciver: UnboundedReceiver<Result<Value, Error>>,
}

impl LCUWebSocket {
    /// Connect to the LCU Web Socket, Error if it fails or the client is not running
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
        let (ws_sender, mut ws_reciver) = mpsc::unbounded_channel::<(u8, String)>();
        let (client_sender, client_reciver) = mpsc::unbounded_channel::<Result<Value, Error>>();

        let handle: JoinHandle<()> = spawn(async move {
            let mut active_commands: HashSet<String> = HashSet::new();
            loop {
                if let Ok((code, endpoint)) = ws_reciver.try_recv() {
                    if code == 5 {
                        active_commands.insert(endpoint.clone());
                    } else if code == 6 {
                        active_commands.remove(&endpoint);
                    };

                    let command = format!("[{}, \"{}\"]", code, endpoint);
                    if write.send(command.into()).await.is_err() {
                        client_sender.send(Err(Error::LCUStoppedRunning)).unwrap();
                    };
                };
                if let Some(Ok(data)) = read.next().await {
                    if let Ok(json) = serde_json::from_slice::<Value>(&data.into_data()) {
                        if active_commands.contains(json[1].as_str().unwrap()) {
                            client_sender.send(Ok(json)).unwrap();
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
    pub fn subscribe(&mut self, endpoint: &str) {
        self.generic_request(5, endpoint);
    }

    /// Unsubscribe to a new API event
    pub fn unsubscribe(&mut self, endpoint: &str) {
        self.generic_request(6, endpoint);
    }

    /// Terminate the event loop
    pub fn terminate(&self) {
        self.handle.abort();
    }

    fn generic_request(&mut self, code: u8, endpoint: &str) {
        let _ = &self.ws_sender.send((code, endpoint.to_owned()));
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

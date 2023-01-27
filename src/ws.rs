use std::collections::HashSet;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::spawn;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{tungstenite::client::IntoClientRequest, Connector};

use crate::{
    utils::{process_info::get_port_and_auth, setup_tls::setup_tls_connector},
    Error,
};

pub struct LCUWebSocket {
    ws_sender: UnboundedSender<(u8, String)>,
    handle: JoinHandle<()>,
    pub client_reciver: UnboundedReceiver<Result<Value, Error>>,
}

impl LCUWebSocket {
    pub async fn new() -> Result<Self, Error> {
        let tls = setup_tls_connector();
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

    pub fn subscribe(&mut self, endpoint: &str) {
        self.generic_request(5, endpoint);
    }

    pub fn unsubscribe(&mut self, endpoint: &str) {
        self.generic_request(6, endpoint);
    }

    pub fn terminate(&self) {
        self.handle.abort();
    }

    fn generic_request(&mut self, code: u8, endpoint: &str) {
        let _ = &self.ws_sender.send((code, endpoint.to_owned()));
    }
}

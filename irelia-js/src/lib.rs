#![deny(clippy::all)]

use std::collections::HashMap;

use napi_derive::napi;

use irelia::RequestClient;
use irelia::rest::LcuClient;
use napi::{Error, Status};
use serde_json::Value;

#[napi(js_name = "LcuClient")]
pub struct JsLcuClient {
    client: LcuClient,
    request_client: RequestClient,
}

#[napi]
impl JsLcuClient {
    #[napi(constructor)]
    /// `try` to establish a new connection to the LCU API, throwing an error otherwise
    pub fn new() -> Result<Self, Error> {
        let request_client = RequestClient::new();
        let client = LcuClient::new(false)
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
            .unwrap();

        Ok(JsLcuClient {
            client,
            request_client,
        })
    }

    #[napi(factory, js_name = "new")]
    /// `try` to establish a new connection to the LCU API, throwing an error otherwise
    /// `force_lock_file` - whether the lock file should ALWAYS be read
    pub fn new_force_lock_file(force_lock_file: bool) -> Result<Self, Error> {
        let request_client = RequestClient::new();
        let client = LcuClient::new(force_lock_file)
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
            .unwrap();

        Ok(JsLcuClient {
            client,
            request_client,
        })
    }

    #[napi]
    pub async fn head(&self, endpoint: String) -> Result<HashMap<String, String>, Error> {
        let head = self
            .client
            .head(endpoint, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))?;

        let (parts, _) = head.into_parts();

        let headers = parts.headers;

        let mut map: HashMap<String, String> = HashMap::with_capacity(headers.len());

        headers.into_iter().for_each(|(name, value)| {
            if let Some(name) = name {
                map.insert(name.to_string(), value.to_str().unwrap().to_string());
            }
        });

        Ok(map)
    }

    #[napi]
    pub async fn delete(&self, endpoint: String) -> Result<Option<Value>, Error> {
        self.client
            .delete(endpoint, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi]
    /// Get an endpoint from the LCU
    pub async fn get(&self, endpoint: String) -> Result<Option<Value>, Error> {
        self.client
            .get(endpoint, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi]
    pub async fn patch(&self, endpoint: String, body: Value) -> Result<Option<Value>, Error> {
        self.client
            .patch(endpoint, Some(body), &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi(js_name = "patch")]
    pub async fn patch_no_body(&self, endpoint: String) -> Result<Option<Value>, Error> {
        self.client
            .patch(endpoint, None::<()>, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi]
    pub async fn post(&self, endpoint: String, body: Value) -> Result<Option<Value>, Error> {
        self.client
            .post(endpoint, Some(body), &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi(js_name = "post")]
    pub async fn post_no_body(&self, endpoint: String) -> Result<Option<Value>, Error> {
        self.client
            .post(endpoint, None::<()>, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi]
    pub async fn put(&self, endpoint: String, body: Value) -> Result<Option<Value>, Error> {
        self.client
            .put(endpoint, Some(body), &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi(js_name = "put")]
    pub async fn put_no_body(&self, endpoint: String) -> Result<Option<Value>, Error> {
        self.client
            .put(endpoint, None::<()>, &self.request_client)
            .await
            .map_err(|e| Error::new(Status::Unknown, e.to_string()))
    }

    #[napi]
    pub fn url(&self) -> &str {
        self.client.url()
    }

    #[napi]
    pub fn auth_header(&self) -> &str {
        self.client.auth_header()
    }
}

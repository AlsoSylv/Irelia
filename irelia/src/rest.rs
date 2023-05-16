use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    utils::{process_info::get_running_client, requests::RequestClient},
    Error,
};

/// Struct representing a connection to the LCU
pub struct LCUClient<'a> {
    client: &'a RequestClient<'a>,
    url: String,
    auth_header: Option<String>,
}

/// Enum representing the different requests that can be sent to the LCU
pub enum RequestType<'a> {
    Delete,
    Get,
    Head,
    Patch(Option<&'a dyn erased_serde::Serialize>),
    Post(Option<&'a dyn erased_serde::Serialize>),
    Put(Option<&'a dyn erased_serde::Serialize>),
}

/// Struct representing a batched request, taking the
/// request type and endpoint
pub struct BatchRequests<'a> {
    pub request_type: RequestType<'a>,
    pub endpoint: &'a str,
}

impl BatchRequests<'_> {
    /// Creates a new batched request, which can be wrapped in a slice and send to the LCU
    pub fn new<'a>(request_type: RequestType<'a>, endpoint: &'a str) -> BatchRequests<'a> {
        BatchRequests {
            request_type,
            endpoint,
        }
    }
}

impl LCUClient<'_> {
    /// Attempts to create a connection to the LCU, errors if it fails
    /// to spin up the child process, or fails to get data from the client.
    /// On Linux, this can happen well the client launches, but on windows
    /// it should be possible to connect well it spins up.
    pub fn new<'a>(client: &'a RequestClient) -> Result<LCUClient<'a>, Error> {
        let port_pass = get_running_client()?;
        Ok(LCUClient {
            client,
            url: port_pass.0,
            auth_header: Some(port_pass.1),
        })
    }

    /// System for batching requests to the LCU by sending a slice
    /// The buffer size is how many requests can be operated on at
    /// the same time, returns a vector with all the replies
    pub async fn batched<'a, R>(
        &self,
        requests: &[BatchRequests<'a>],
        buffer_size: usize,
    ) -> Vec<Result<Option<R>, Error>>
    where
        R: DeserializeOwned,
    {
        futures_util::stream::iter(requests.iter().map(|request| async move {
            match &request.request_type {
                RequestType::Delete => self.delete(request.endpoint).await,
                RequestType::Get => self.get(request.endpoint).await,
                RequestType::Head => self.head(request.endpoint).await,
                RequestType::Patch(body) => self.patch(request.endpoint, *body).await,
                RequestType::Post(body) => self.post(request.endpoint, *body).await,
                RequestType::Put(body) => self.put(request.endpoint, *body).await,
            }
        }))
        .buffered(buffer_size)
        .collect()
        .await
    }

    /// Sends a delete request to the LCU
    pub async fn delete<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_request::<(), R>(endpoint, "DELETE", None).await
    }

    /// Sends a get request to the LCU
    pub async fn get<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_request::<(), R>(endpoint, "GET", None).await
    }

    /// Sends a head request to the LCU
    pub async fn head<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_request::<(), R>(endpoint, "HEAD", None).await
    }

    /// Sends a patch request to the LCU
    pub async fn patch<T, R>(&self, endpoint: &str, body: Option<T>) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_request(endpoint, "PATCH", body).await
    }

    /// Sends a post request to the LCU
    pub async fn post<T, R>(&self, endpoint: &str, body: Option<T>) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_request(endpoint, "POST", body).await
    }

    /// Sends a put request to the LCU
    pub async fn put<T, R>(&self, endpoint: &str, body: Option<T>) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_request(endpoint, "PUT", body).await
    }

    async fn lcu_request<T, R>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
    ) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.client
            .request_template(
                &self.url,
                endpoint,
                method,
                body,
                self.auth_header.as_deref(),
                |bytes| {
                    if bytes.is_empty() {
                        Ok(None)
                    } else {
                        serde_json::from_slice(&bytes)
                            .map(|value| Ok(Some(value)))
                            .map_or_else(|err| Err(Error::SerdeJsonError(err)), Ok)?
                    }
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::requests::RequestClient;

    #[tokio::test]
    async fn batch_test() {
        use crate::rest::{BatchRequests, LCUClient, RequestType};

        let page = serde_json::json!(
            {
              "blocks": [
                {
                  "items": [
                    {
                      "count": 1,
                      "id": "3153"
                    },
                  ],
                  "type": "Final Build"
                }
              ],
              "title": "Test Build",
            }
        );
        let client = RequestClient::new();

        let client = LCUClient::new(&client).unwrap();
        let id = &client
            .get::<serde_json::Value>("/lol-summoner/v1/current-summoner")
            .await
            .unwrap()
            .unwrap()["summonerId"];

        let endpoint = format!("/lol-item-sets/v1/item-sets/{id}/sets");
        let mut json = client
            .get::<serde_json::Value>(&endpoint)
            .await
            .unwrap()
            .unwrap();
        json["itemSets"].as_array_mut().unwrap().push(page);

        let req = BatchRequests {
            request_type: RequestType::Put(Some(&json)),
            endpoint: &format!("/lol-item-sets/v1/item-sets/{id}/sets"),
        };

        let res = client.batched::<serde_json::Value>(&[req], 1).await;

        println!("{:?}", res);

        let a = client
            .put::<_, serde_json::Value>(
                &format!("/lol-item-sets/v1/item-sets/{id}/sets"),
                Some(json),
            )
            .await;
        println!("{:?}", a);
    }
}

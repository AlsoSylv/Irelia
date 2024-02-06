use irelia::{rest::LCUClient, LCUError, RequestClient};
use serde_json::Value;
use tokio::{runtime::Runtime, task::JoinHandle};

pub struct Crt {
    rt: Runtime,
}

impl Crt {
    pub fn new(rt: Runtime) -> Self {
        Self { rt }
    }

    pub fn rt(&self) -> &Runtime {
        &self.rt
    }
}

pub struct CFuture {
    fut: JoinHandle<Result<Option<Value>, LCUError>>,
}

impl CFuture {
    pub fn new(fut: JoinHandle<Result<Option<Value>, LCUError>>) -> Self {
        Self { fut }
    }

    pub fn take_fut(self) -> JoinHandle<Result<Option<Value>, LCUError>> {
        self.fut
    }

    pub fn fut(&self) -> &JoinHandle<Result<Option<Value>, LCUError>> {
        &self.fut
    }
}

pub struct CRequestClient {
    client: RequestClient,
}

impl CRequestClient {
    pub fn new(client: RequestClient) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &RequestClient {
        &self.client
    }
}

pub struct CLCUClient {
    client: LCUClient,
}

impl CLCUClient {
    pub fn new(client: LCUClient) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &LCUClient {
        &self.client
    }
}

pub struct CLCUResponse {
    res: i8,
    error: Option<LCUError>,
}

impl CLCUResponse {
    pub fn new(res: i8, error: Option<LCUError>) -> Self {
        Self { res, error }
    }

    pub fn res(&self) -> i8 {
        self.res
    }

    pub fn error(&self) -> &Option<LCUError> {
        &self.error
    }
}

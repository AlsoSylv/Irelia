extern crate irelia;
extern crate tokio;

use irelia::rest::batch::Request;

#[tokio::main]
async fn main() {
    use irelia::rest::batch::Builder;

    // Create the required clients
    let request_client = irelia::RequestClient::new();
    let lcu_client = irelia::rest::LcuClient::new(false).unwrap();

    // Create a handful of requests
    let request_1 = Request::get("/example/endpoint/");
    let request_2 = Request::get("/example/endpoint/");
    let request_3 = Request::get("/example/endpoint/");
    let request_4 = Request::get("/example/endpoint/");

    let builder = Builder::new();
    let with_request_client = builder
        .with_client(&request_client)
        .request(request_1)
        .request(request_2)
        .request(request_3)
        .request(request_4);
    // Execute two requests at a time
    let with_buffer_size = with_request_client.with_buffer_size(2);
    let with_lcu_client = with_buffer_size.with_lcu_client(&lcu_client);
    let _results = with_lcu_client.execute::<serde_json::Value>().await;
}

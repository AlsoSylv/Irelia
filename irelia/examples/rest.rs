extern crate irelia;
extern crate tokio;

#[tokio::main]
async fn main() {
    let request_client = irelia::RequestClient::new();
    let lcu_client = irelia::rest::LcuClient::new(false).unwrap();

    let value: serde_json::Value = lcu_client
        .get("Example", &request_client)
        .await
        .unwrap()
        .unwrap();

    let _value = &value["example_index"];
}

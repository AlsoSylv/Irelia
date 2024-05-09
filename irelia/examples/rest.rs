extern crate irelia;
extern crate tokio;

#[tokio::main]
async fn main() {
    let request_client = irelia::RequestClient::new();
    let lcu_client = irelia::rest::LcuClient::new(false).unwrap();

    let current_summoner: Option<serde_json::Value> = lcu_client
        .get("/lol-summoner/v1/current-summoner", &request_client)
        .await
        .unwrap();

    if let Some(value) = current_summoner {
        let summoner_id = &value["summonerId"];

        println!("{summoner_id}");
    }
}

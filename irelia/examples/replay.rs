extern crate irelia;
extern crate tokio;

use irelia::replay::ReplayClient;
use irelia::RequestClient;

#[tokio::main]
async fn main() {
    let request_client = RequestClient::new();
    let replay_client = ReplayClient::new();

    let mut renderer = replay_client.get_render(&request_client).await.unwrap();

    renderer.fog_of_war = false;

    let mut renderer = replay_client
        .post_render(&renderer, &request_client)
        .await
        .unwrap();

    renderer.fog_of_war = true;

    replay_client
        .post_render(&renderer, &request_client)
        .await
        .unwrap();
}

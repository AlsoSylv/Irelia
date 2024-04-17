extern crate irelia;
extern crate tokio;

use irelia::replay::types::Sequence;
use irelia::replay::ReplayClient;
use irelia::RequestClient;

#[tokio::main]
async fn main() {
    let request_client = RequestClient::new();
    let replay_client = ReplayClient::new();

    let process_info = replay_client.game(&request_client).await;

    println!("{process_info:?}");

    let mut renderer = replay_client.get_render(&request_client).await.unwrap();

    println!("{renderer:?}");

    renderer.fog_of_war = Some(false);

    let renderer = replay_client.post_render(renderer, &request_client).await;

    println!("{renderer:?}");

    let sequence = Sequence::default();

    let sequence = replay_client.post_sequence(sequence, &request_client).await;

    println!("{sequence:?}");

    let sequence = replay_client.get_sequence(&request_client).await;

    println!("{sequence:?}");
}

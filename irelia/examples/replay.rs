extern crate irelia;
extern crate tokio;

use irelia::replay::ReplayClient;
use irelia::RequestClient;

#[tokio::main]
async fn main() {
    let request_client = RequestClient::new();
    let replay_client = ReplayClient::new();

    let process_info = replay_client.game(&request_client).await;

    println!("{process_info:?}");

    let all_game_data = replay_client.all_game_data(&request_client).await;

    println!("{all_game_data:?}");

    let mut renderer = replay_client.get_render(&request_client).await.unwrap();

    println!("{renderer:?}");

    renderer.fog_of_war = false;

    renderer.skybox_offset = 1.0;
    renderer.nav_grid_offset = 1.0;

    let renderer = replay_client.post_render(renderer, &request_client).await;

    println!("{renderer:?}");
}

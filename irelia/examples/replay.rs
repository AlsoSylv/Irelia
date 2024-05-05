extern crate irelia;
extern crate tokio;

use irelia::replay::ReplayClient;
use irelia::replay::types::{Frame, FrameList};
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

    let record = replay_client.get_recording(&request_client).await.unwrap();

    let mut sequence = FrameList::new();

    sequence.push(Frame::from_render_recording("TestSequence", &renderer, &record));
    
    sequence[0].camera_position = None;
    
    println!("\n\n{sequence:#?}");

    let sequence = replay_client.post_sequence(sequence, &request_client).await.unwrap();

    println!("\n\n{:#}", serde_json::to_value(sequence).unwrap());
}

extern crate irelia;
extern crate tokio;

use irelia::replay::types::{Frame, FrameList};
use irelia::replay::ReplayClient;
use irelia::RequestClient;

#[tokio::main]
async fn main() {
    let request_client = RequestClient::new();
    let replay_client = ReplayClient::new();

    let mut renderer = replay_client.get_render(&request_client).await.unwrap();

    renderer.fog_of_war = false;

    let record = replay_client.get_recording(&request_client).await.unwrap();

    let mut sequence = FrameList::new();

    sequence.push(Frame::from_render_recording(
        "TestSequence",
        &renderer,
        &record,
    ));

    let mut renderer = replay_client.get_render(&request_client).await.unwrap();

    renderer.fog_of_war = true;

    let record = replay_client.get_recording(&request_client).await.unwrap();

    sequence.push(Frame::from_render_recording(
        "TestSequence",
        &renderer,
        &record,
    ));

    sequence.push(Frame::from_render_recording(
        "TestSequence",
        &renderer,
        &record,
    ));

    println!("\n\n{sequence:?}");

    let sequence = replay_client
        .post_sequence(sequence, &request_client)
        .await
        .unwrap();

    println!("\n\n{}", serde_json::to_value(sequence).unwrap());
}

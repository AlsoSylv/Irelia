//! Example of how to record a clip following a specific player

extern crate irelia;
extern crate tokio;

use irelia::replay::types::{HudCameraMode, Vector3f};
use irelia::replay::ReplayClient;
use irelia::RequestClient;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let replay_client = RequestClient::new();

    let mut renderer = replay_client.get_render().await.unwrap();

    println!("{renderer:?}");

    renderer.camera_mode = HudCameraMode::Fps;
    renderer.selection_offset = Vector3f {
        x: 0.0,
        y: 1911.85,
        z: -1200.0,
    };
    renderer.camera_attached = true;
    renderer.selection_name = "Example".into();

    let renderer = replay_client.post_render(&renderer).await.unwrap();

    println!("{renderer:?}");

    let mut recording = replay_client.get_recording().await.unwrap();

    recording.height = 1920;
    recording.width = 1080;
    recording.recording = true;

    let mut playback = replay_client.get_playback().await.unwrap();
    playback.paused = false;

    let playback = replay_client.post_playback(playback);
    let recording = replay_client.post_recording(recording);

    let (playback, recording) = tokio::join!(playback, recording);

    playback.unwrap();
    let mut recording = recording.unwrap();

    tokio::time::sleep(Duration::from_secs(40)).await;

    recording.recording = false;
    let _ = replay_client.post_recording(recording).await.unwrap();
}

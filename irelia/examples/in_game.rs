extern crate irelia;
extern crate tokio;

#[tokio::main]
async fn main() {
    use irelia::in_game::GameClient;

    let in_game_client = irelia::requests::new();

    let active_player = in_game_client.active_player().await.unwrap();

    println!("{active_player:?}");

    let all_game_data = in_game_client.all_game_data().await;

    println!("{all_game_data:?}");
}

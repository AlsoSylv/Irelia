extern crate irelia;
extern crate tokio;

#[tokio::main]
async fn main() {
    let request_client = irelia::RequestClient::new();
    let in_game_client = irelia::in_game::GameClient::new();

    let active_player = in_game_client.active_player(&request_client).await.unwrap();

    println!("{active_player:?}");
    
    let all_game_data = in_game_client.all_game_data(&request_client).await;

    println!("{all_game_data:?}");
}

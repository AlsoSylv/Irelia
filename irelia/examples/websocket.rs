use irelia::ws::types::{Event, EventKind};
use irelia::ws::{Flow, LCUWebSocket, Subscriber};
use std::ops::ControlFlow;
use std::time::Duration;

#[tokio::main]
async fn main() {
    #[derive(Debug)]
    struct EventCounter(u32);

    impl Subscriber for EventCounter {
        fn on_event(&mut self, event: &Event) -> ControlFlow<(), Flow> {
            println!("{event:?}");
            self.0 += 1;
            println!("{}", self.0);

            ControlFlow::Continue(Flow::Continue)
        }
    }

    let mut ws_client = LCUWebSocket::new().unwrap();

    let id = ws_client
        .subscribe(
            EventKind::JsonApiEventCallback("/lol-champ-select/v1/current-champion".into()),
            EventCounter(0),
        )
        .unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    ws_client
        .unsubscribe(
            EventKind::JsonApiEventCallback("/lol-champ-select/v1/current-champion".into()),
            id,
        )
        .unwrap();

    ws_client.terminate();
}

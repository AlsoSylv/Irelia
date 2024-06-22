use irelia::ws::types::{Event, EventKind};
use irelia::ws::{Flow, LCUWebSocket, Subscriber};
use std::ops::ControlFlow;
use std::thread;
use std::time::Duration;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_target(false)
        .init();
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
        .subscribe(EventKind::JsonApiEvent, EventCounter(0))
        .unwrap();

    while !ws_client.is_finished() {}

    thread::sleep(Duration::from_secs(15));

    ws_client.unsubscribe(EventKind::JsonApiEvent, id).unwrap();

    println!("Done!");
}

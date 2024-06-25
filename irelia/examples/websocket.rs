use irelia::ws::types::{Event, EventKind};
use irelia::ws::{Flow, LcuWebSocket, Subscriber};
use std::ops::ControlFlow;
use std::thread;
use std::time::Duration;

fn main() {
    struct EventCounter(u32);

    impl Subscriber for EventCounter {
        fn on_event(&mut self, event: &Event) -> ControlFlow<(), Flow> {
            println!("{event:?}");
            println!("{}", self.0);
            self.0 += 1;

            ControlFlow::Continue(Flow::Continue)
        }
    }

    let mut ws_client = LcuWebSocket::new();

    let id = ws_client
        .subscribe(EventKind::JsonApiEvent, EventCounter(0))
        .unwrap();

    thread::sleep(Duration::from_secs(15));

    ws_client.unsubscribe(EventKind::JsonApiEvent, id).unwrap();

    ws_client.abort().unwrap();

    println!("Done!");
}

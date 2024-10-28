use std::sync::{Arc, Mutex, RwLock, TryLockError};

use super::{types::Event, Subscriber};

impl<T> Subscriber for Arc<Mutex<T>>
where
    T: Subscriber,
{
    fn on_subscribe(
        &mut self,
        event_kind: &super::types::EventKind,
        request_code: &super::types::RequestType,
    ) {
        match self.try_lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_subscribe(event_kind, request_code);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.try_lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.try_lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }
}

impl<T> Subscriber for Arc<RwLock<T>>
where
    T: Subscriber + Sync,
{
    fn on_subscribe(
        &mut self,
        event_kind: &super::types::EventKind,
        request_code: &super::types::RequestType,
    ) {
        match self.try_write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_subscribe(event_kind, request_code);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.try_write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.try_write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                panic!("Poisoned {poisoned:?}")
            }
            Err(TryLockError::WouldBlock) => {}
        }
    }
}

impl<F, R> Subscriber for F
where
    F: FnMut(&Event) -> R + Send,
    R: Returns,
{
    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        *continues = self(event).val();
    }
}

pub trait Returns {
    fn val(self) -> bool;
}

impl Returns for bool {
    fn val(self) -> bool {
        self
    }
}

impl Returns for () {
    fn val(self) -> bool {
        true
    }
}

impl Returns for Result<(), ()> {
    fn val(self) -> bool {
        self.is_ok()
    }
}

impl Returns for Option<()> {
    fn val(self) -> bool {
        self.is_some()
    }
}

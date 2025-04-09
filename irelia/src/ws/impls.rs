use std::{
    ops::ControlFlow,
    sync::{Arc, Mutex, RwLock},
};

use super::{Subscriber, types::Event};

impl<T> Subscriber for Arc<Mutex<T>>
where
    T: Subscriber,
{
    fn on_subscribe(
        &mut self,
        event_kind: &super::types::EventKind,
        request_code: &super::types::RequestType,
    ) {
        match self.lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_subscribe(event_kind, request_code);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore => {}
                super::PoisonBehavior::Break => *continues = false,
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
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
        match self.write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_subscribe(event_kind, request_code);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore => {}
                super::PoisonBehavior::Break => *continues = false,
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(poisoned) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
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

/// Trait for return values of closures
///
/// Default implementations for `()`, `bool`, `Option`, `Result`, and `ControlFlow` exist
/// The implementation for `Result` and `ControlFlow` inverts the value on `Err` and `Break` respectively
pub trait Returns {
    fn val(self) -> bool;
}

impl Returns for () {
    fn val(self) -> bool {
        true
    }
}

impl Returns for bool {
    fn val(self) -> bool {
        self
    }
}

impl<T> Returns for Option<T>
where
    T: Returns,
{
    fn val(self) -> bool {
        if let Some(v) = self { v.val() } else { false }
    }
}

impl<T, U> Returns for Result<T, U>
where
    T: Returns,
    U: Returns,
{
    fn val(self) -> bool {
        match self {
            Ok(t) => t.val(),
            Err(u) => !u.val(),
        }
    }
}

impl<T, U> Returns for ControlFlow<T, U>
where
    T: Returns,
    U: Returns,
{
    fn val(self) -> bool {
        match self {
            ControlFlow::Continue(t) => t.val(),
            ControlFlow::Break(u) => u.val(),
        }
    }
}

use std::{
    ops::ControlFlow,
    sync::{Arc, Mutex, RwLock, TryLockError},
};

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
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.try_lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore => {}
                super::PoisonBehavior::Break => *continues = false,
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.try_lock() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
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
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_event(&mut self, event: &Event, continues: &mut bool) {
        match self.try_write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_event(event, continues);
            }
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore => {}
                super::PoisonBehavior::Break => *continues = false,
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
            Err(TryLockError::WouldBlock) => {}
        }
    }

    fn on_unsubscribe(&mut self, event_kind: &super::types::EventKind) {
        match self.try_write() {
            Ok(mut guard) => {
                let t = &mut *guard;
                t.on_unsubscribe(event_kind);
            }
            Err(TryLockError::Poisoned(poisoned)) => match self.on_poison() {
                super::PoisonBehavior::Clear => self.clear_poison(),
                super::PoisonBehavior::Ignore | super::PoisonBehavior::Break => {}
                super::PoisonBehavior::Panic => panic!("{poisoned}"),
            },
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
        if let Some(v) = self {
            v.val()
        } else {
            false
        }
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

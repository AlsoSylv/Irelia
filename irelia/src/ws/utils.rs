use crate::ws::types::EventKind;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Index;

#[derive(Default, Debug)]
pub struct EventMap<V> {
    json_api_event: V,
    lcds_event: V,
    log: V,
    region_locale_changed: V,
    service_proxy_async_event: V,
    service_proxy_method_event: V,
    service_proxy_uuid_event: V,
    json_api_event_callback: HashMap<String, V>,
    lcds_event_callback: HashMap<String, V>,
}

impl<V: Debug + Default> EventMap<V> {
    pub fn new() -> Self {
        Self {
            json_api_event: V::default(),
            lcds_event: V::default(),
            log: V::default(),
            region_locale_changed: V::default(),
            service_proxy_async_event: V::default(),
            service_proxy_method_event: V::default(),
            service_proxy_uuid_event: V::default(),
            json_api_event_callback: HashMap::new(),
            lcds_event_callback: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get_mut(&mut self, event_kind: &EventKind) -> &mut V {
        let events = match event_kind {
            EventKind::JsonApiEvent => &mut self.json_api_event,
            EventKind::LcdsEvent => &mut self.lcds_event,
            EventKind::Log => &mut self.log,
            EventKind::RegionLocaleChanged => &mut self.region_locale_changed,
            EventKind::ServiceProxyAsyncEvent => &mut self.service_proxy_async_event,
            EventKind::ServiceProxyMethodEvent => &mut self.service_proxy_method_event,
            EventKind::ServiceProxyUuidEvent => &mut self.service_proxy_uuid_event,
            EventKind::JsonApiEventCallback(key) => {
                let mut key = key.replace('/', "_");
                if &key[0..1] == "_" {
                    key.remove(0);
                }

                if !self.json_api_event_callback.contains_key(&key) {
                    self.json_api_event_callback
                        .insert(key.clone(), V::default());
                }

                self.json_api_event_callback.get_mut(&key).unwrap()
            }
            EventKind::LcdsEventCallback(key) => {
                let mut key = key.replace('/', "_");
                if &key[0..1] == "_" {
                    key.remove(0);
                }

                if !self.lcds_event_callback.contains_key(&key) {
                    self.lcds_event_callback.insert(key.clone(), V::default());
                }

                self.lcds_event_callback.get_mut(&key).unwrap()
            }
        };

        events
    }
}

impl<V> Index<&EventKind> for EventMap<V> {
    type Output = V;

    fn index(&self, index: &EventKind) -> &Self::Output {
        match index {
            EventKind::JsonApiEvent => &self.json_api_event,
            EventKind::LcdsEvent => &self.lcds_event,
            EventKind::Log => &self.log,
            EventKind::RegionLocaleChanged => &self.region_locale_changed,
            EventKind::ServiceProxyAsyncEvent => &self.service_proxy_async_event,
            EventKind::ServiceProxyMethodEvent => &self.service_proxy_method_event,
            EventKind::ServiceProxyUuidEvent => &self.service_proxy_uuid_event,
            EventKind::JsonApiEventCallback(key) => {
                let mut key = key.replace('/', "_");
                if &key[0..1] == "_" {
                    key.remove(0);
                }

                &self.json_api_event_callback[&key]
            }
            EventKind::LcdsEventCallback(key) => {
                let mut key = key.replace('/', "_");
                if &key[0..1] == "_" {
                    key.remove(0);
                }

                &self.lcds_event_callback[&key]
            }
        }
    }
}

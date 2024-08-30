//! Types that the Websocket will respond with

use serde::de::{Error, Visitor};
use serde::{
    Deserialize as DeserializeTrait, Deserializer, Serialize as SerializeTrait, Serializer,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
/// Directly corresponds to the tuple that the websocket emits for events
pub struct Event(pub RequestType, pub EventKind, pub EventData);

/// Different LCU websocket request types
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum RequestType {
    Welcome = 0,
    Prefix = 1,
    Call = 2,
    CallResult = 3,
    CallError = 4,
    Subscribe = 5,
    Unsubscribe = 6,
    Publish = 7,
    Event = 8,
}
#[derive(Debug, Eq, PartialEq, Clone)]
/// Different event types that can be passed to the
/// subscribe and unsubscribe methods.
pub enum EventKind {
    JsonApiEvent,
    LcdsEvent,
    Log,
    RegionLocaleChanged,
    ServiceProxyAsyncEvent,
    ServiceProxyMethodEvent,
    ServiceProxyUuidEvent,
    JsonApiEventCallback(String),
    LcdsEventCallback(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// The data connected to the event, this consists of three fields, the data, event type, and uri the event is from
pub struct EventData {
    pub data: Value,
    pub event_type: String,
    pub uri: String,
}

impl<'de> DeserializeTrait<'de> for RequestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u64::deserialize(deserializer)?;

        Ok(match num {
            0 => Self::Welcome,
            1 => Self::Prefix,
            2 => Self::Call,
            3 => Self::CallResult,
            4 => Self::CallError,
            5 => Self::Subscribe,
            6 => Self::Unsubscribe,
            7 => Self::Publish,
            8 => Self::Event,
            _ => unreachable!("Only numbers 0-8 are valid"),
        })
    }
}

impl SerializeTrait for RequestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl<'de> DeserializeTrait<'de> for EventKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl<'a> Visitor<'a> for StrVisitor {
            type Value = EventKind;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A string in the format of an LCU event kind")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Some((event, callback)) = v.split_once('_') {
                    match EventKind::from_str(event) {
                        EventKind::JsonApiEvent => {
                            Ok(EventKind::JsonApiEventCallback(callback.to_string()))
                        }
                        EventKind::LcdsEvent => {
                            Ok(EventKind::LcdsEventCallback(callback.to_string()))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    Ok(EventKind::from_str(v))
                }
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}

impl SerializeTrait for EventKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EventKind {
    pub(super) fn to_string(&self) -> Cow<'static, str> {
        match self {
            Self::JsonApiEvent => "OnJsonApiEvent".into(),
            Self::LcdsEvent => "OnLcdsEvent".into(),
            Self::Log => "OnLog".into(),
            Self::RegionLocaleChanged => "OnRegionLocaleChanged".into(),
            Self::ServiceProxyAsyncEvent => "OnServiceProxyAsyncEvent".into(),
            Self::ServiceProxyMethodEvent => "OnServiceProxyMethodEvent".into(),
            Self::ServiceProxyUuidEvent => "OnServiceProxyUuidEvent".into(),
            Self::JsonApiEventCallback(callback) => {
                let mut callback = callback.replace('/', "_");
                if &callback[0..1] == "_" {
                    callback.remove(0);
                }
                format!("OnJsonApiEvent_{callback}").into()
            }
            Self::LcdsEventCallback(callback) => {
                let mut callback = callback.replace('/', "_");
                callback.remove(0);

                format!("OnLcdsEvent_{callback}").into()
            }
        }
    }

    fn from_str(event: &str) -> Self {
        match event {
            "OnJsonApiEvent" => Self::JsonApiEvent,
            "OnLcdsEvent" => Self::LcdsEvent,
            "OnLog" => Self::Log,
            "OnRegionLocaleChanged" => Self::RegionLocaleChanged,
            "OnServiceProxyAsyncEvent" => Self::ServiceProxyAsyncEvent,
            "OnServiceProxyMethodEvent" => Self::ServiceProxyMethodEvent,
            "OnServiceProxyUuidEvent" => Self::ServiceProxyUuidEvent,
            event => unreachable!("{}", event),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Event, EventData, EventKind, RequestType};
    use serde_json::{json, Map, Value};

    #[test]
    fn test_deserialize() {
        let json = json!([5, "OnJsonApiEvent", {
            "data": {},
            "eventType": "Create",
            "uri": "/Example/Uri"
        }]);

        let event: Event = serde_json::from_value(json).unwrap();

        let baseline_event = Event(
            RequestType::Subscribe,
            EventKind::JsonApiEvent,
            EventData {
                data: Value::Object(Map::new()),
                event_type: "Create".into(),
                uri: "/Example/Uri".into(),
            },
        );

        assert_eq!(event, baseline_event);
    }
}

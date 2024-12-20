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
    JsonApiEvent { callback: Option<Cow<'static, str>> },
    LcdsEvent { callback: Option<Cow<'static, str>> },
    Log,
    RegionLocaleChanged,
    ServiceProxyAsyncEvent,
    ServiceProxyMethodEvent,
    ServiceProxyUuidEvent,
}

impl EventKind {
    #[must_use]
    pub const fn json_api_event() -> Self {
        Self::JsonApiEvent { callback: None }
    }

    #[must_use]
    pub const fn json_api_event_callback_str(callback: &'static str) -> Self {
        Self::JsonApiEvent {
            callback: Some(Cow::Borrowed(callback)),
        }
    }

    #[must_use]
    pub fn json_api_event_callback(callback: impl Into<Cow<'static, str>>) -> Self {
        Self::JsonApiEvent {
            callback: Some(callback.into()),
        }
    }

    #[must_use]
    pub const fn lcds_event() -> Self {
        Self::LcdsEvent { callback: None }
    }

    #[must_use]
    pub const fn lcds_event_callback_str(callback: &'static str) -> Self {
        Self::LcdsEvent {
            callback: Some(Cow::Borrowed(callback)),
        }
    }

    #[must_use]
    pub fn lcds_event_callback(callback: impl Into<Cow<'static, str>>) -> Self {
        Self::LcdsEvent {
            callback: Some(callback.into()),
        }
    }
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
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> DeserializeTrait<'de> for EventKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = EventKind;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A string in the format of an LCU event kind")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut event_kind = v;
                let mut callback = None;

                if let Some(idx) = v.find('_') {
                    event_kind = &v[0..idx];
                    callback = Some(&v[idx + 1..]);
                }

                let event_kind = EventKind::from_str(event_kind);

                if let Some(endpoint) = callback {
                    let endpoint = endpoint.to_string();

                    if matches!(event_kind, EventKind::JsonApiEvent { callback: None }) {
                        Ok(EventKind::json_api_event_callback(endpoint))
                    } else if matches!(event_kind, EventKind::LcdsEvent { callback: None }) {
                        Ok(EventKind::lcds_event_callback(endpoint))
                    } else {
                        // It is not possible for it to not match either case
                        unreachable!()
                    }
                } else {
                    Ok(event_kind)
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
            Self::JsonApiEvent { callback: None } => "OnJsonApiEvent".into(),
            Self::JsonApiEvent {
                callback: Some(callback),
            } => {
                let callback = callback.replace('/', "_");
                let callback = if let Some(callback) = callback.strip_prefix('_') {
                    callback
                } else {
                    &callback
                };
                format!("OnJsonApiEvent_{callback}").into()
            }
            Self::LcdsEvent { callback: None } => "OnLcdsEvent".into(),
            Self::LcdsEvent {
                callback: Some(callback),
            } => {
                let callback = callback.replace('/', "_");
                let callback = if let Some(callback) = callback.strip_prefix('_') {
                    callback
                } else {
                    &callback
                };
                format!("OnLcdsEvent_{callback}").into()
            }
            Self::Log => "OnLog".into(),
            Self::RegionLocaleChanged => "OnRegionLocaleChanged".into(),
            Self::ServiceProxyAsyncEvent => "OnServiceProxyAsyncEvent".into(),
            Self::ServiceProxyMethodEvent => "OnServiceProxyMethodEvent".into(),
            Self::ServiceProxyUuidEvent => "OnServiceProxyUuidEvent".into(),
        }
    }

    fn from_str(event: &str) -> Self {
        match event {
            "OnJsonApiEvent" => Self::JsonApiEvent { callback: None },
            "OnLcdsEvent" => Self::LcdsEvent { callback: None },
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
            EventKind::JsonApiEvent { callback: None },
            EventData {
                data: Value::Object(Map::new()),
                event_type: "Create".into(),
                uri: "/Example/Uri".into(),
            },
        );

        assert_eq!(event, baseline_event);

        let json = json!([5, "OnJsonApiEvent_example", {
            "data": {},
            "eventType": "Create",
            "uri": "/Example/Uri"
        }]);

        let event: Event = serde_json::from_value(json).unwrap();

        let baseline_event = Event(
            RequestType::Subscribe,
            EventKind::JsonApiEvent {
                callback: Some("example".into()),
            },
            EventData {
                data: Value::Object(Map::new()),
                event_type: "Create".into(),
                uri: "/Example/Uri".into(),
            },
        );

        assert_eq!(event, baseline_event);
    }
}

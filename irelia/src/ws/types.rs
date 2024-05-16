use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub struct Event(pub RequestType, pub EventKind, pub EventData);

/// Different LCU websocket request types
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    pub data: Value,
    pub event_type: String,
    pub uri: String,
}

impl<'de> Deserialize<'de> for RequestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u64::deserialize(deserializer)?;

        Ok(match num {
            0 => RequestType::Welcome,
            1 => RequestType::Prefix,
            2 => RequestType::Call,
            3 => RequestType::CallResult,
            4 => RequestType::CallError,
            5 => RequestType::Subscribe,
            6 => RequestType::Unsubscribe,
            7 => RequestType::Publish,
            8 => RequestType::Event,
            _ => unreachable!("Only numbers 0-8 are valid"),
        })
    }
}

impl Serialize for RequestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for EventKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: &str = Deserialize::deserialize(deserializer)?;

        if let Some((event, callback)) = v.split_once('_') {
            match EventKind::from_str(event) {
                EventKind::JsonApiEvent => {
                    Ok(EventKind::JsonApiEventCallback(callback.to_string()))
                }
                EventKind::LcdsEvent => Ok(EventKind::LcdsEventCallback(callback.to_string())),
                _ => unreachable!(),
            }
        } else {
            Ok(EventKind::from_str(v))
        }
    }
}

impl Serialize for EventKind {
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
            EventKind::JsonApiEvent => "OnJsonApiEvent".into(),
            EventKind::LcdsEvent => "OnLcdsEvent".into(),
            EventKind::Log => "OnLog".into(),
            EventKind::RegionLocaleChanged => "OnRegionLocaleChanged".into(),
            EventKind::ServiceProxyAsyncEvent => "OnServiceProxyAsyncEvent".into(),
            EventKind::ServiceProxyMethodEvent => "OnServiceProxyMethodEvent".into(),
            EventKind::ServiceProxyUuidEvent => "OnServiceProxyUuidEvent".into(),
            EventKind::JsonApiEventCallback(callback) => {
                format!("OnJsonApiEvent{}", callback.replace('/', "_")).into()
            }
            EventKind::LcdsEventCallback(callback) => {
                format!("OnLcdsEvent{}", callback.replace('/', "_")).into()
            }
        }
    }

    fn from_str(event: &str) -> EventKind {
        match event {
            "OnJsonApiEvent" => EventKind::JsonApiEvent,
            "OnLcdsEvent" => EventKind::LcdsEvent,
            "OnLog" => EventKind::Log,
            "OnRegionLocaleChanged" => EventKind::RegionLocaleChanged,
            "OnServiceProxyAsyncEvent" => EventKind::ServiceProxyAsyncEvent,
            "OnServiceProxyMethodEvent" => EventKind::ServiceProxyMethodEvent,
            "OnServiceProxyUuidEvent" => EventKind::ServiceProxyUuidEvent,
            event => unreachable!("{}", event),
        }
    }
}

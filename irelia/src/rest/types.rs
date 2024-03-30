use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, HashMap<String, Operation>>,
    pub components: Components,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Components {
    pub schemas: HashMap<String, SchemaValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaValue {
    #[serde(rename = "type")]
    pub schema_type: Type,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub schema_enum: Option<Vec<String>>,
    pub additional_properties: PropertyAdditionalProperties,
    pub properties: Option<HashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    #[serde(rename = "type")]
    pub property_type: Option<Type>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    #[serde(rename = "$ref")]
    pub property_ref: Option<String>,
    pub additional_properties: Option<PropertyAdditionalProperties>,
    pub items: Option<Box<Property>>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyAdditionalProperties {
    Bool(bool),
    ItemsAdditionalProperties(Box<Property>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Double,
    Float,
    Int16,
    Int32,
    Int64,
    Int8,
    Uint16,
    Uint32,
    Uint64,
    Uint8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Array,
    Boolean,
    Integer,
    Number,
    Object,
    String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub description: Option<String>,
    pub operation_id: String,
    pub parameters: Vec<Parameter>,
    pub responses: Option<HashMap<String, Responses>>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub request_body: Option<HashMap<String, RequestBody>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: Option<bool>,
    pub schema: Option<Box<Property>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum In {
    Path,
    Query,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleSchema {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    pub content: Option<Content>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationJson {
    pub schema: Option<ApplicationJsonItems>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationJsonItems {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub additional_properties: Option<bool>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    pub items: Option<PurpleSchema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Responses {
    #[serde(rename = "2XX")]
    pub the_2_xx: Option<_2XX>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct _2XX {
    pub content: Option<Content>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledSchema {
    #[serde(rename = "type")]
    pub schema_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalPropertiesClass {
    #[serde(rename = "type")]
    pub additional_properties_type: Option<Type>,
    #[serde(rename = "$ref")]
    pub additional_properties_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

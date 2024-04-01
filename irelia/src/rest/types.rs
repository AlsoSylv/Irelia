use serde_derive::{Deserialize, Serialize};
use hashlink::LinkedHashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Schema {
    pub openapi: String,
    pub info: Info,
    pub paths: LinkedHashMap<String, LinkedHashMap<String, Operation>>,
    pub components: Components,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Components {
    pub schemas: LinkedHashMap<String, SchemaValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)] 
pub struct SchemaValue {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub schema_enum: Option<Vec<String>>,
    pub additional_properties: Option<PropertyAdditionalProperties>,
    pub properties: Option<LinkedHashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)] 
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
#[serde(deny_unknown_fields)] 
pub enum PropertyAdditionalProperties {
    Bool(bool),
    ItemsAdditionalProperties(Box<Property>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)] 
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
#[serde(deny_unknown_fields)] 
pub enum Type {
    Array,
    Boolean,
    Integer,
    Number,
    Object,
    String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Info {
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)] 
pub struct Operation {
    pub description: String,
    pub operation_id: String,
    pub parameters: Vec<Parameter>,
    pub responses: Option<LinkedHashMap<String, Responses>>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub request_body: Option<RequestBody>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Parameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: Option<bool>,
    pub schema: Option<Box<Property>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)] 
pub enum In {
    Path,
    Query,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RequestBody {
    pub content: Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Content {
    #[serde(rename = "application/json")]
    pub application_json: ApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct ApplicationJson {
    pub schema: Option<Property>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Responses {
    pub content: Option<Content>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Tag {
    pub name: String,
}

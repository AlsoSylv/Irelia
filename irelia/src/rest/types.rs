use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, Path>,
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
    pub description: SchemaDescription,
    #[serde(rename = "enum")]
    pub schema_enum: Option<Vec<String>>,
    pub additional_properties: bool,
    pub properties: Option<HashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SchemaDescription {
    #[serde(rename = "Allowable severity levels for log events.")]
    AllowableSeverityLevelsForLogEvents,
    #[serde(rename = "Describes a function.")]
    DescribesAFunction,
    #[serde(rename = "Describes a function parameter.")]
    DescribesAFunctionParameter,
    #[serde(rename = "Describes a log entry.")]
    DescribesALogEntry,
    #[serde(rename = "Describes a member of a struct.")]
    DescribesAMemberOfAStruct,
    #[serde(rename = "Describes a struct or enum type.")]
    DescribesAStructOrEnumType,
    #[serde(rename = "Describes an enumerator.")]
    DescribesAnEnumerator,
    #[serde(rename = "Describes an event.")]
    DescribesAnEvent,
    #[serde(rename = "Describes the exposed native API.")]
    DescribesTheExposedNativeApi,
    #[serde(rename = "Describes the type of a value.")]
    DescribesTheTypeOfAValue,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "Help format for binding functions and types.")]
    HelpFormatForBindingFunctionsAndTypes,
    #[serde(rename = "Help format for remoting functions and types.")]
    HelpFormatForRemotingFunctionsAndTypes,
    #[serde(rename = "Possible states of an asynchronous operation.")]
    PossibleStatesOfAnAsynchronousOperation,
    #[serde(rename = "Represents a cancelled asynchronous operation.")]
    RepresentsACancelledAsynchronousOperation,
    #[serde(rename = "Represents a failed asynchronous operation.")]
    RepresentsAFailedAsynchronousOperation,
    #[serde(rename = "Represents generic data for an asynchronous event.")]
    RepresentsGenericDataForAnAsynchronousEvent,
    #[serde(rename = "Represents generic data for an event.")]
    RepresentsGenericDataForAnEvent,
    #[serde(rename = "Represents the parameters of a call to a provided callback.")]
    RepresentsTheParametersOfACallToAProvidedCallback,
    #[serde(rename = "Serialization format for remoting requests and results.")]
    SerializationFormatForRemotingRequestsAndResults,
    #[serde(rename = "User Experience Settings Operating System Information")]
    UserExperienceSettingsOperatingSystemInformation,
    #[serde(rename = "User Experience Settings System Information")]
    UserExperienceSettingsSystemInformation,
    #[serde(rename = "Well-known privilege levels for bindable functions.")]
    WellKnownPrivilegeLevelsForBindableFunctions,
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
    pub items: Option<Box<ItemsAdditionalProperties>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyAdditionalProperties {
    Bool(bool),
    ItemsAdditionalProperties(Box<ItemsAdditionalProperties>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsAdditionalProperties {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub additional_properties: Option<bool>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    pub items: Option<Box<ItemsAdditionalProperties>>,
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
pub struct Path {
    pub post: Option<Post>,
    pub delete: Option<Delete>,
    pub get: Option<Get>,
    pub put: Option<Put>,
    pub patch: Option<Delete>,
    pub head: Option<Head>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    pub operation_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub parameters: Vec<DeleteParameter>,
    pub responses: DeleteResponses,
    pub request_body: Option<DeleteRequestBody>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: bool,
    pub schema: PurpleSchema,
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
pub struct DeleteRequestBody {
    pub content: PurpleContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleContent {
    #[serde(rename = "application/json")]
    pub application_json: PurpleApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleApplicationJson {
    pub schema: ApplicationJsonItems,
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
pub struct DeleteResponses {
    #[serde(rename = "2XX")]
    pub the_2_xx: Purple2Xx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Purple2Xx {
    pub content: Option<PurpleContent>,
    pub description: The2XxDescription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum The2XxDescription {
    #[serde(rename = "Success response")]
    SuccessResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get {
    pub operation_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub parameters: Vec<GetParameter>,
    pub responses: GetResponses,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: Option<bool>,
    pub schema: FluffySchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffySchema {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    pub items: Option<Box<ItemsAdditionalProperties>>,
    pub additional_properties: Option<TentacledSchema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledSchema {
    #[serde(rename = "type")]
    pub schema_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResponses {
    #[serde(rename = "2XX")]
    pub the_2_xx: Fluffy2Xx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fluffy2Xx {
    pub content: Option<FluffyContent>,
    pub description: The2XxDescription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyContent {
    #[serde(rename = "application/json")]
    pub application_json: FluffyApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyApplicationJson {
    pub schema: StickySchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickySchema {
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub items: Option<ApplicationJsonItems>,
    pub additional_properties: Option<PurpleAdditionalProperties>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleAdditionalProperties {
    ApplicationJsonItems(ApplicationJsonItems),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {
    pub operation_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub parameters: Vec<HeadParameter>,
    pub responses: HeadResponses,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadParameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: bool,
    pub schema: TentacledSchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadResponses {
    #[serde(rename = "2XX")]
    pub the_2_xx: Tentacled2Xx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tentacled2Xx {
    pub content: TentacledContent,
    pub description: The2XxDescription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledContent {
    #[serde(rename = "application/json")]
    pub application_json: TentacledApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledApplicationJson {
    pub schema: IndigoSchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoSchema {
    #[serde(rename = "type")]
    pub schema_type: Type,
    pub additional_properties: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub operation_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub parameters: Vec<PostParameter>,
    pub request_body: Option<PostRequestBody>,
    pub responses: PostResponses,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostParameter {
    #[serde(rename = "in")]
    pub parameter_in: In,
    pub name: String,
    pub required: bool,
    pub schema: Box<ItemsAdditionalProperties>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostRequestBody {
    pub content: StickyContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyContent {
    #[serde(rename = "application/json")]
    pub application_json: StickyApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyApplicationJson {
    pub schema: Option<IndecentSchema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentSchema {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    pub items: Option<PurpleSchema>,
    pub additional_properties: Option<FluffyAdditionalProperties>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyAdditionalProperties {
    Bool(bool),
    IndigoSchema(IndigoSchema),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostResponses {
    #[serde(rename = "2XX")]
    pub the_2_xx: Sticky2Xx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sticky2Xx {
    pub content: Option<IndigoContent>,
    pub description: The2XxDescription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoContent {
    #[serde(rename = "application/json")]
    pub application_json: IndigoApplicationJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoApplicationJson {
    pub schema: HilariousSchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousSchema {
    #[serde(rename = "type")]
    pub schema_type: Option<Type>,
    pub additional_properties: Option<TentacledAdditionalProperties>,
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    pub items: Option<ApplicationJsonItems>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledAdditionalProperties {
    AdditionalPropertiesClass(AdditionalPropertiesClass),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalPropertiesClass {
    #[serde(rename = "type")]
    pub additional_properties_type: Option<Type>,
    #[serde(rename = "$ref")]
    pub additional_properties_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Put {
    pub operation_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub parameters: Vec<PostParameter>,
    pub request_body: Option<DeleteRequestBody>,
    pub responses: DeleteResponses,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

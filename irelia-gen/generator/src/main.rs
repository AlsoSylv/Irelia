use std::borrow::Cow;
use std::str::FromStr;
use std::{fs::File, io::Write};
use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use rustfmt_wrapper::rustfmt;

fn main() {
    let raw: &[u8] = include_bytes!("swagger.json");
    let json: Schema = serde_json::from_slice(raw).unwrap();
    let mut enum_file = File::create("./irelia-gen/src/generated_enums.rs").unwrap();
    let mut struct_file = File::create("./irelia-gen/src/generated_structs.rs").unwrap();

    let mut enum_scope = Vec::new();

    let mut struct_scope = Vec::new();

    json.components.schemas.iter().for_each(|(k, v)| {
        if let Some(enums) = &v.schema_enum {
            let name = k.to_case(Case::Pascal);
            let name = TokenStream::from_str(&name).unwrap();

            let (rename_all, case) = if enums.iter().all(|s| s.is_case(Case::ScreamingSnake)) {
                ("SCREAMING_SNAKE_CASE",
                 Case::ScreamingSnake)
            } else if enums.iter().all(|s| s.is_case(Case::Kebab)) {
                ("kebab-case",
                 Case::Kebab)
            } else if enums.iter().all(|s| s.is_case(Case::Camel)) {
                ("camelCase",
                 Case::Camel)
            } else {
                ("PascalCase",
                 Case::Pascal)
            };

            let rename_all = quote! { #[serde(rename_all = #rename_all)] };

            let cases: Vec<_> = enums.iter().map(|str| {
                let name = str.to_case(Case::UpperCamel);
                let current_case = name.to_case(case);
                let rename = if current_case.as_str() != str {
                    Some(quote! { #[serde(rename = #str)] })
                } else {
                    None
                };

                let name = TokenStream::from_str(&name).unwrap();

                quote ! { #rename #name }
            }).collect();

            let quote = quote! {
                #[derive(Serialize, Deserialize)]
                #rename_all
                pub enum #name {
                    #(#cases),*
                }
            };

            enum_scope.push(quote);
        } else if let Some(fields) = &v.properties {
            let name = k.to_case(Case::UpperCamel);

            let rename = if &name != k {
                Some(quote! {
                    #[serde(rename = #k)]
                })
            } else {
                None
            };

            let (rename_all, case) = if fields.iter().all(|(s, _)| s.is_case(Case::ScreamingSnake))
            {
                ("SCREAMING_SNAKE_CASE", Case::ScreamingSnake)
            } else if fields.iter().all(|(s, _)| s.is_case(Case::Kebab)) {
                ("kebab-case", Case::Kebab)
            } else if fields.iter().all(|(s, _)| s.is_case(Case::Camel)) {
                ("camelCase", Case::Camel)
            } else {
                ("PascalCase", Case::Pascal)
            };

            let rename_all = quote! {
                #[serde(rename_all = #rename_all)]
            };

            let fields: Vec<_> = fields
                .iter()
                .map(|(path, prop)| {
                    let name = if path == "type" {
                        "_type".to_string()
                    } else if path == "async" {
                        "_async".to_string()
                    } else {
                        path.to_case(Case::Snake)
                    };

                    let field_ty: Cow<'_, str> = if let Some(ty) = &prop.property_type {
                        if ty.is_array() {
                            let items = prop.items.as_ref().unwrap();

                            if let Some(ty) = &items.property_type {
                                format!("Vec<{}>", ty.to_str(&items.format)).into()
                            } else if let Some(rf) = &items.property_ref {
                                let ty = rf.split('/').last().unwrap().to_case(Case::Pascal);
                                format!("Vec<{}>", ty).into()
                            } else {
                                unreachable!("{items:?}")
                            }
                        } else {
                            ty.to_str(&prop.format).into()
                        }
                    } else if let Some(rf) = &prop.property_ref {
                        let ty = rf.split('/').last().unwrap().to_case(Case::Pascal);
                        ty.into()
                    } else {
                        unreachable!("{prop:?}")
                    };

                    let mut rename = None;

                    if &name.to_case(case) != path {
                        rename = Some(quote! {
                            #[serde(rename = #path)]
                        })
                    }

                    let name = TokenStream::from_str(&name).unwrap();

                    let ty = TokenStream::from_str(&field_ty).unwrap();

                    quote! {
                        #rename
                        #name: #ty
                    }
                })
                .collect();

            let doc = if let Some(doc) = &v.description {
                if doc.is_empty() {
                    None
                } else {
                    Some(quote! {
                        #[doc = #doc]
                    })
                }
            } else {
                None
            };

            let name = TokenStream::from_str(&name).unwrap();

            let quote = quote! {
                #[derive(Serialize, Deserialize)]
                #rename
                #rename_all
                #doc
                pub struct #name {
                    #(#fields),*
                }
            };

            struct_scope.push(quote);
        } else if let Some(ty) = &v.schema_type {
            let name = k.to_case(Case::Pascal);
            let name = TokenStream::from_str(&name).unwrap();
            let ty = TokenStream::from_str(ty.to_str(&None)).unwrap();
            let ty = quote! {
                type #name = #ty;
            };
            struct_scope.push(ty);
        }
    });

    let structs = quote! {
        use serde_derive::{Deserialize, Serialize};
        use serde_json::Value;
        use crate::generated_enums::*;

        #(#struct_scope)*
    };

    let structs = rustfmt(structs).unwrap();

    struct_file.write_all(structs.as_bytes()).unwrap();

    let enums = quote! {
        use serde_derive::{Deserialize, Serialize};

        #(#enum_scope)*
    };

    let enums = rustfmt(enums).unwrap();

    enum_file
        .write_all(enums.as_bytes())
        .unwrap();

    let traits: Vec<_> = json.tags.iter().map(|plugin| {
        let name = plugin.name.to_case(Case::Pascal);
        let name = TokenStream::from_str(&name).unwrap();
        let functions: Vec<Vec<_>> = json.paths.iter().map(|(key, value)| {
            let functions: Vec<_> = value.iter().map(|(method, op)| {
                if op.tags.contains(&plugin.name) {
                    let name = op.operation_id.to_case(Case::Snake);
                    let mut ret = None;
                    let mut request_body: Option<Cow<'_, str>> = None;

                    let params: Vec<TokenStream> = op.parameters.iter().map(|param| {
                        let name = &param.name;
                        let name = name.to_case(Case::Snake);
                        let name = match name.as_str() {
                            "type" => "_type",
                            "+path" => "_path",
                            els => els,
                        };
                        let name = TokenStream::from_str(name).unwrap();
                        assert_ne!(param.schema, None);
                        let Some(props) = &param.schema else { unreachable!() };

                        let base: Cow<'_, str>;

                        if let Some(ty) = &props.property_type {
                            if ty.is_array() {
                                let items = props.items.as_ref().unwrap();
                                match items.as_ref() {
                                    PropertyAdditionalProperties::Bool(_) => unreachable!(),
                                    PropertyAdditionalProperties::ItemsAdditionalProperties(add) => {
                                        if let Some(ty) = &add.property_type {
                                            base = format!("Vec<{}>", ty.to_str(&add.format)).into();
                                        } else if let Some(rf) = &add.property_ref {
                                            let rf = rf.split('/').last().unwrap();
                                            let ty = rf.to_case(Case::Pascal);
                                            base = format!("Vec<{ty}>").into();
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                }
                            } else {
                                base = ty.to_str(&props.format).into();
                            }
                        } else if let Some(rf) = &props.property_ref {
                            base = default_ref(rf);
                        } else {
                            unreachable!()
                        }

                        format_arg(&name, &base)
                    }).collect();

                    if let Some(body) = &op.request_body {
                        if let Some(schema) = &body.content.application_json.schema {
                            if let Some(ty) = &schema.property_type {
                                if ty.is_array() {
                                    let items = schema.items.as_ref().unwrap();
                                    if let Some(ty) = &items.property_type {
                                        let ty = format!("Vec<{}>", ty.to_str(&items.format));
                                        request_body = Some(ty.into());
                                    }
                                    if let Some(rf) = &items.property_ref {
                                        let rf = rf.split('/').last().unwrap();
                                        let ty = rf.to_case(Case::Pascal);
                                        request_body = Some(format!("Vec<{ty}>").into());
                                    }
                                } else {
                                    request_body = Some(ty.to_str(&schema.format).into());
                                }

                                if let Some(rf) = &schema.property_ref {
                                    request_body = Some(handle_maybe_additional_props(&schema.additional_properties, &schema.format, rf));
                                }
                            }
                        }
                    }

                    if let Some(response) = &op.responses {
                        let response = &response["2XX"];
                        if let Some(content) = &response.content {
                            if let Some(props) = &content.application_json.schema {
                                if let Some(ty) = &props.property_type {
                                    let ty = type_to_cow(ty, &props.format, &props.items);
                                    ret = Some(ty);
                                }

                                if let Some(rf) = &props.property_ref {
                                    ret = Some(handle_maybe_additional_props(&props.additional_properties, &props.format, rf));
                                }
                            }
                        }
                    }

                    let body = if let Some(request_body) = request_body {
                        Some(TokenStream::from_str(&format!(", body: {request_body}")).unwrap())
                    } else {
                        None
                    };

                    let name = TokenStream::from_str(&name).unwrap();

                    let quote = if let Some(ret) = ret {
                        let ret = TokenStream::from_str(&ret).unwrap();
                        quote! { fn #name(&self #(#params)* #body) -> #ret; }
                    } else {
                        quote! { fn #name(&self #(#params)* #body); }
                    };

                    quote
                } else {
                    quote!()
                }
            }).collect();

            functions
        }).collect();

        quote! {
            pub trait #name {
                #(#(#functions)*)*
            }
        }
    }).collect();

    let plugins = quote! {
        use serde_json::Value;
        use crate::generated_enums::*;
        use crate::generated_structs::*;

        #(#traits)*
    };

    let output = rustfmt(plugins).unwrap();

    std::fs::write("./irelia-gen/src/plugin_traits.rs", output).unwrap();
}

fn handle_maybe_additional_props<'a>(
    additional_properties: &'a Option<PropertyAdditionalProperties>,
    format: &'a Option<Format>,
    rf: &'a str,
) -> Cow<'a, str> {
    let Some(add) = additional_properties else {
        return default_ref(rf);
    };

    match add {
        PropertyAdditionalProperties::Bool(true) => {
            let rf = rf.split('/').last().unwrap();
            let ty = rf.to_case(Case::Pascal);
            ty.into()
        }
        PropertyAdditionalProperties::Bool(false) => unreachable!(),
        PropertyAdditionalProperties::ItemsAdditionalProperties(add) => {
            handle_additional_props(add, format)
        }
    }
}

fn handle_additional_props<'a>(
    additional: &'a AdditionalProperties,
    format: &'a Option<Format>,
) -> Cow<'a, str> {
    assert_eq!(additional.additional_properties, None);
    assert_eq!(additional.items, None);

    if let Some(rf) = &additional.property_ref {
        let rf = rf.split('/').last().unwrap();
        let ty = rf.to_case(Case::Pascal);
        ty.into()
    } else if let Some(ty) = &additional.property_type {
        assert_ne!(*ty, Type::Array);
        ty.to_str(format).into()
    } else {
        todo!("????")
    }
}

fn default_ref(rf: &str) -> Cow<'_, str> {
    let rf = rf.split('/').last().unwrap();
    let rf = rf.to_case(Case::Pascal);
    rf.into()
}

fn format_arg(name: &TokenStream, ty: &str) -> TokenStream {
    let ty = TokenStream::from_str(&ty).unwrap();
    quote!(, #name: #ty)
}

fn type_to_cow<'a>(
    ty: &'a Type,
    format: &Option<Format>,
    additional: &'a Option<AdditionalProperties>,
) -> Cow<'a, str> {
    if ty.is_array() {
        let items = additional.as_ref().unwrap();
        let base_ty: Cow<'_, str>;
        if let Some(ty) = &items.property_type {
            base_ty = ty.to_str(&items.format).into();
        } else if let Some(rf) = &items.property_ref {
            let rf = rf.split('/').last().unwrap();
            base_ty = rf.to_case(Case::Pascal).into();
        } else {
            unreachable!()
        }
        format!("Vec<{}>", base_ty).into()
    } else {
        ty.to_str(format).into()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, HashMap<String, Operation>>,
    pub components: Components,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Components {
    pub schemas: HashMap<String, SchemaValue>,
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
    pub properties: Option<HashMap<String, Property>>,
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
    pub items: Option<AdditionalProperties>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum PropertyAdditionalProperties {
    Bool(bool),
    ItemsAdditionalProperties(AdditionalProperties),
}

// Avoid an alloc
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AdditionalProperties {
    #[serde(rename = "type")]
    pub property_type: Option<Type>,
    pub format: Option<Format>,
    pub minimum: Option<i64>,
    #[serde(rename = "$ref")]
    pub property_ref: Option<String>,
    pub additional_properties: Option<Box<PropertyAdditionalProperties>>,
    pub items: Option<Box<PropertyAdditionalProperties>>,
    pub required: Option<bool>,
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

impl Type {
    fn is_array(&self) -> bool {
        *self == Type::Array
    }

    fn to_str(&self, format: &Option<Format>) -> &str {
        if let Some(format) = format {
            match format {
                Format::Double => "f64",
                Format::Float => "f32",
                Format::Int16 => "i16",
                Format::Int32 => "i32",
                Format::Int64 => "i64",
                Format::Int8 => "i8",
                Format::Uint16 => "u16",
                Format::Uint32 => "u32",
                Format::Uint64 => "u64",
                Format::Uint8 => "u8",
            }
        } else {
            match self {
                Type::Boolean => "bool",
                Type::String => "String",
                Type::Object => "Value",
                ty => unreachable!("{:?}", ty),
            }
        }
    }
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
    pub responses: Option<HashMap<String, Responses>>,
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
    pub schema: Option<AdditionalProperties>,
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

use crate::error::AlxError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs, path::Path};

const INDENT: &str = "    ";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectConfig {
    pub endpoints: Vec<Endpoint>,
}

impl ProjectConfig {
    pub fn _parse(yaml: String) -> Result<Self, AlxError> {
        let config = serde_yaml::from_str::<Self>(&yaml)?;
        Ok(config)
    }

    pub fn write_config_lock(&self, format: ConfigFormat) -> Result<(), AlxError> {
        match format {
            ConfigFormat::Json => {
                let config = serde_json::to_string_pretty(self)?;
                fs::write(Path::new("./alx_lock.json"), config)?;
            }
            ConfigFormat::Yaml => {
                let config = serde_yaml::to_string(self)?;
                fs::write(Path::new("./alx_lock.yaml"), config)?;
            }
            ConfigFormat::Both => {
                let config = serde_json::to_string_pretty(self)?;
                fs::write(Path::new("./alx_lock.json"), config)?;
                let config = serde_yaml::to_string(self)?;
                fs::write(Path::new("./alx_lock.yaml"), config)?;
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Both,
}

impl Display for ProjectConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ep in &self.endpoints {
            writeln!(f, "- Endpoint: '{}'\nRoutes: ", ep.id)?;
            for r in &ep.routes {
                writeln!(f, "{INDENT}Method: {}", r.method)?;
                writeln!(f, "{INDENT}Path: {}", r.path)?;
                writeln!(f, "{INDENT}Handler: {:?}", r.handler)?;
                writeln!(
                    f,
                    "{INDENT}Service: {}",
                    r.service.as_ref().unwrap_or(&"null".to_string())
                )?;
                writeln!(
                    f,
                    "{INDENT}MW: {:?}\n",
                    r.middleware.as_ref().unwrap_or(&vec![])
                )?;
            }
        }
        Ok(())
    }
}

/// Defines an endpoint in the project structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub id: String,
    pub routes: Vec<RouteHandler>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteHandler {
    pub method: String,
    pub path: String,
    pub handler: Option<Handler>,
    pub middleware: Option<Vec<String>>,
    pub service: Option<String>,
}

impl From<(&Route, Option<&Handler>)> for RouteHandler {
    fn from((r, h): (&Route, Option<&Handler>)) -> Self {
        Self {
            method: r.method.to_string(),
            path: r.path.to_string(),
            handler: h.cloned(),
            middleware: r.middleware.clone(),
            service: r.service.clone(),
        }
    }
}

/// Intermediary struct for capturing setup functions
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Route {
    /// The HTTP method for the route
    pub method: String,
    /// The name of the designated handler for the route
    pub handler_name: String,
    /// The path to the resource
    pub path: String,
    /// The middleware wrapped around the route, if any
    pub middleware: Option<Vec<String>>,
    pub service: Option<String>,
}

/// Intermediary struct for capturing all handler functions
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Handler {
    pub name: String,
    pub inputs: Vec<HandlerInput>,
    pub bound: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct HandlerInput {
    #[serde(rename = "extractor")]
    pub ext_type: String,
    #[serde(rename = "data")]
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Data {
    pub wrapper_id: String,
    /// Contains the struct's fields where the first
    /// element is the field name and the second the field type
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub required: bool,
    pub validation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Extractor {
    #[serde(alias = "path", alias = "Path")]
    Path,
    #[serde(alias = "query", alias = "Query")]
    Query,
    #[serde(alias = "json", alias = "Json")]
    Json,
    #[serde(alias = "form", alias = "Form")]
    Form,
    #[serde(alias = "request", alias = "Request", alias = "HttpRequest")]
    Request,
    #[serde(alias = "string", alias = "String")]
    String,
    #[serde(alias = "bytes", alias = "Bytes")]
    Bytes,
    #[serde(alias = "payload", alias = "Payload")]
    Payload,
    #[serde(alias = "data", alias = "Data")]
    Data,
    Invalid,
}

impl From<String> for Extractor {
    fn from(s: String) -> Self {
        match s.as_str() {
            "path" | "Path" => Self::Path,
            "query" | "Query" => Self::Query,
            "json" | "Json" => Self::Json,
            "form" | "Form" => Self::Form,
            "request" | "Request" | "HttpRequest" => Self::Request,
            "string" | "String" => Self::String,
            "bytes" | "Bytes" => Self::Bytes,
            "payload" | "Payload" => Self::Payload,
            "data" | "Data" => Self::Data,
            _ => Self::Invalid,
        }
    }
}
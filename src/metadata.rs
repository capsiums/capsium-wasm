//! metadata.json model (CC 62001 §2). The canonical identity of a
//! package: name, version, guid, uuid, optional description, license,
//! author, repository, dependencies, modules, readOnly.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub guid: String,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub dependencies: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub modules: Vec<String>,
    #[serde(default, rename = "readOnly")]
    pub read_only: bool,
}

impl Metadata {
    /// Parse a metadata.json document. Errors propagate as PackageError
    /// so callers don't have to know about serde's error type.
    pub fn from_json(text: &str) -> Result<Self, crate::PackageError> {
        serde_json::from_str(text).map_err(crate::PackageError::from)
    }
}

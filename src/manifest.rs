//! manifest.json model (CC 62001 §3). Every content file with its
//! MIME type and visibility. Auto-generated at pack time; reactors
//! read it to know what to serve.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Canonical form: resources map keyed by package-relative path.
    #[serde(default)]
    pub resources: BTreeMap<String, ManifestResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestResource {
    #[serde(rename = "type")]
    pub mime_type: String,
    #[serde(default = "default_visibility")]
    pub visibility: String,
}

fn default_visibility() -> String {
    "exported".to_string()
}

impl Manifest {
    pub fn from_json(text: &str) -> Result<Self, crate::PackageError> {
        serde_json::from_str(text).map_err(crate::PackageError::from)
    }

    /// The MIME type the manifest declares for a path, or None when
    /// the path is not listed.
    pub fn mime_for(&self, path: &str) -> Option<&str> {
        self.resources.get(path).map(|r| r.mime_type.as_str())
    }
}

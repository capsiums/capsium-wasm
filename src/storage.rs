//! storage.json model (CC 62001 §5). Declares the datasets a
//! package exposes (the source file, optional schema).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    #[serde(default)]
    pub storage: Option<StorageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageData {
    #[serde(default, rename = "dataSets")]
    pub data_sets: BTreeMap<String, DatasetConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub source: String,
    #[serde(default, rename = "schemaFile")]
    pub schema_file: Option<String>,
    #[serde(default, rename = "schemaType")]
    pub schema_type: Option<String>,
}

impl Storage {
    pub fn from_json(text: &str) -> Result<Self, crate::PackageError> {
        serde_json::from_str(text).map_err(crate::PackageError::from)
    }

    pub fn dataset(&self, name: &str) -> Option<&DatasetConfig> {
        self.storage.as_ref()?.data_sets.get(name)
    }
}

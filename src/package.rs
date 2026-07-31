//! The Package facade: load from a {path -> bytes} map, parse
//! every config file, expose the typed model + verification result.

use std::collections::BTreeMap;
use thiserror::Error;

use crate::{Manifest, Metadata, Routes, Security, Storage};

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("metadata.json is missing or invalid")]
    MetadataInvalid(#[from] serde_json::Error),
    #[error("metadata.json not found in the package")]
    MetadataMissing,
}

/// One loaded Capsium package: typed config models + the raw file
/// map (for content reads the host still owns).
#[derive(Debug, Clone)]
pub struct Package {
    pub metadata: Metadata,
    pub manifest: Option<Manifest>,
    pub routes: Option<Routes>,
    pub storage: Option<Storage>,
    pub security: Option<Security>,
    pub files: BTreeMap<String, Vec<u8>>,
}

impl Package {
    /// Build a Package from a parsed file map (host has already
    /// unzipped the .cap). Required: metadata.json. Optional: every
    /// other config file; absence is not an error.
    pub fn from_files(files: BTreeMap<String, Vec<u8>>) -> Result<Self, PackageError> {
        let metadata_bytes = files
            .get("metadata.json")
            .ok_or(PackageError::MetadataMissing)?;
        let metadata = Metadata::from_json(&String::from_utf8_lossy(metadata_bytes))?;

        let manifest = files
            .get("manifest.json")
            .and_then(|b| Manifest::from_json(&String::from_utf8_lossy(b)).ok());
        let routes = files
            .get("routes.json")
            .and_then(|b| Routes::from_json(&String::from_utf8_lossy(b)).ok());
        let storage = files
            .get("storage.json")
            .and_then(|b| Storage::from_json(&String::from_utf8_lossy(b)).ok());
        let security = files
            .get("security.json")
            .and_then(|b| Security::from_json(&String::from_utf8_lossy(b)).ok());

        Ok(Self { metadata, manifest, routes, storage, security, files })
    }
}

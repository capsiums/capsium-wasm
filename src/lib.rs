//! Capsium core types: the package model that every host reactor
//! shares. Compiled to WASM via wasm-pack; bound from Ruby / Python /
//! Go / JS via thin host shims.
//!
//! See ARCHITECTURE.md and the spec at standards/ for the normative
//! definitions of these types. Keep this crate dependency-light so
//! the WASM artifact stays small enough for browser embedding.

#![forbid(unsafe_code)]

pub mod integrity;
pub mod manifest;
pub mod metadata;
pub mod package;
pub mod routes;
pub mod security;
pub mod storage;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use manifest::{Manifest, ManifestResource};
pub use metadata::Metadata;
pub use package::{Package, PackageError};
pub use routes::{Route, RouteKind, Routes};
pub use security::{IntegrityIssue, IntegrityReport, Security};
pub use storage::{DatasetConfig, Storage};

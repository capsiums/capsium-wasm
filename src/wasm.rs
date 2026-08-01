//! WASM bindings for capsium-core — exposed via `wasm-pack build --target web`
//! when the `wasm` feature is enabled. Hosts (browsers, Node, deno) import
//! these directly. The typed models below stay pure Rust so the same crate
//! builds as an rlib for host-language FFI.
//!
//! Every JS-facing function takes/returns plain JS objects via
//! serde-wasm-bindgen; the host never sees Rust types directly.

#![cfg(feature = "wasm")]

use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;

use crate::{
    integrity::{checksum_files, sha256_hex},
    package::Package,
    security::{IntegrityIssue, Security},
};

/// SHA-256 hex digest of the input bytes. Used at pack time to build
/// security.json's checksum map deterministically.
#[wasm_bindgen]
pub fn sha256(bytes: &[u8]) -> String {
    sha256_hex(bytes)
}

/// Walk every file in the map and return `{ path -> sha256_hex }`.
/// Used by `capsium packager` to populate security.json.
#[wasm_bindgen]
pub fn checksum_files_js(files: JsValue) -> Result<JsValue, JsValue> {
    let map: BTreeMap<String, Vec<u8>> = serde_wasm_bindgen::from_value(files).map_err(js_error)?;
    serde_wasm_bindgen::to_value(&checksum_files(&map)).map_err(js_error)
}

/// Parse a metadata.json document into a JS object. Throws on parse error.
#[wasm_bindgen]
pub fn parse_metadata(text: &str) -> Result<JsValue, JsValue> {
    let metadata = crate::Metadata::from_json(text).map_err(js_error)?;
    serde_wasm_bindgen::to_value(&metadata).map_err(js_error)
}

/// Parse a security.json document into a JS object.
#[wasm_bindgen]
pub fn parse_security(text: &str) -> Result<JsValue, JsValue> {
    let security = Security::from_json(text).map_err(js_error)?;
    serde_wasm_bindgen::to_value(&security).map_err(js_error)
}

/// Verify every declared checksum against the provided files map.
/// Returns `{ valid: bool, issues: [{ kind, path, ... }] }`.
#[wasm_bindgen]
pub fn verify_integrity(security_text: &str, files: JsValue) -> Result<JsValue, JsValue> {
    let security = Security::from_json(security_text).map_err(js_error)?;
    let files_map: BTreeMap<String, Vec<u8>> =
        serde_wasm_bindgen::from_value(files).map_err(js_error)?;
    let report = security.verify(&files_map);
    serde_wasm_bindgen::to_value(&IntegrityReportJs {
        valid: report.valid(),
        issues: report.issues.iter().map(issue_to_js).collect(),
    })
    .map_err(js_error)
}

/// Load a full Package from a `{ path -> bytes }` map. Returns the
/// package's identity + the raw files map for host-side content reads.
#[wasm_bindgen]
pub fn load_package(files: JsValue) -> Result<JsValue, JsValue> {
    let files_map: BTreeMap<String, Vec<u8>> =
        serde_wasm_bindgen::from_value(files).map_err(js_error)?;
    let pkg = Package::from_files(files_map).map_err(js_error)?;
    serde_wasm_bindgen::to_value(&PackageJs {
        name: pkg.metadata.name,
        version: pkg.metadata.version,
        guid: pkg.metadata.guid,
    })
    .map_err(js_error)
}

#[derive(serde::Serialize)]
struct IntegrityReportJs {
    valid: bool,
    issues: Vec<IntegrityIssueJs>,
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum IntegrityIssueJs {
    Missing {
        path: String,
    },
    Mismatch {
        path: String,
        expected: String,
        actual: String,
    },
    Unchecked {
        path: String,
    },
}

fn issue_to_js(issue: &IntegrityIssue) -> IntegrityIssueJs {
    match issue {
        IntegrityIssue::Missing { path } => IntegrityIssueJs::Missing { path: path.clone() },
        IntegrityIssue::Mismatch {
            path,
            expected,
            actual,
        } => IntegrityIssueJs::Mismatch {
            path: path.clone(),
            expected: expected.clone(),
            actual: actual.clone(),
        },
        IntegrityIssue::Unchecked { path } => IntegrityIssueJs::Unchecked { path: path.clone() },
    }
}

#[derive(serde::Serialize)]
struct PackageJs {
    name: String,
    version: String,
    guid: String,
}

fn js_error<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&e.to_string())
}

//! Integrity verification helpers exposed at the crate root for
//! host bindings. Hosts that already have a parsed package call
//! `verify_files(checksums, files)` directly; hosts loading from
//! raw bytes call `Package::from_files`.

use std::collections::BTreeMap;

pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Hash every file into a `{ path -> sha256_hex }` map. Used at
/// pack time to build security.json deterministically.
pub fn checksum_files(files: &BTreeMap<String, Vec<u8>>) -> BTreeMap<String, String> {
    files.iter().map(|(k, v)| (k.clone(), sha256_hex(v))).collect()
}

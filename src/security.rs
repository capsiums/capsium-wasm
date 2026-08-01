//! security.json model + integrity verification (CC 62001 §6).
//! A reactor MUST verify checksums at activation and refuse on
//! mismatch.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    #[serde(default)]
    pub security: Option<SecurityData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityData {
    #[serde(default, rename = "integrityChecks")]
    pub integrity_checks: Option<IntegrityChecks>,
    #[serde(default, rename = "digitalSignatures")]
    pub digital_signatures: Option<DigitalSignatures>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityChecks {
    #[serde(default, rename = "checksumAlgorithm")]
    pub checksum_algorithm: Option<String>,
    #[serde(default)]
    pub checksums: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignatures {
    #[serde(default, rename = "publicKey")]
    pub public_key: Option<String>,
    #[serde(default, rename = "signatureFile")]
    pub signature_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityIssue {
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

#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub issues: Vec<IntegrityIssue>,
}

impl IntegrityReport {
    pub fn valid(&self) -> bool {
        self.issues.is_empty()
    }
}

impl Security {
    pub fn from_json(text: &str) -> Result<Self, crate::PackageError> {
        serde_json::from_str(text).map_err(crate::PackageError::from)
    }

    /// Verify every declared checksum against the provided files map.
    /// Returns a report listing every issue (missing file, mismatch,
    /// unchecked). Empty issues = valid.
    pub fn verify(&self, files: &BTreeMap<String, Vec<u8>>) -> IntegrityReport {
        use sha2::{Digest, Sha256};
        let mut issues = Vec::new();
        let checksums = match self
            .security
            .as_ref()
            .and_then(|s| s.integrity_checks.as_ref())
        {
            Some(c) => &c.checksums,
            None => return IntegrityReport { issues },
        };
        let mut covered = std::collections::HashSet::new();
        for (path, expected) in checksums {
            covered.insert(path.clone());
            let Some(bytes) = files.get(path) else {
                issues.push(IntegrityIssue::Missing { path: path.clone() });
                continue;
            };
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let digest = hasher.finalize();
            let actual = format!("{:x}", digest);
            if actual != expected.to_lowercase() {
                issues.push(IntegrityIssue::Mismatch {
                    path: path.clone(),
                    expected: expected.clone(),
                    actual,
                });
            }
        }
        for path in files.keys() {
            if path == "security.json" || path == "signature.sig" {
                continue;
            }
            if !covered.contains(path) {
                issues.push(IntegrityIssue::Unchecked { path: path.clone() });
            }
        }
        IntegrityReport { issues }
    }
}

use capsium_core::{IntegrityIssue, Package, Security};

fn main() {
    let mut files = std::collections::BTreeMap::new();
    files.insert(
        "metadata.json".to_string(),
        br#"{"name":"demo","version":"0.1.0","guid":"https://example.com/demo"}"#.to_vec(),
    );
    files.insert("content/index.html".to_string(), b"<h1>hi</h1>".to_vec());

    let package = Package::from_files(files.clone()).expect("parse");
    println!("name:    {}", package.metadata.name);
    println!("version: {}", package.metadata.version);
    println!("files:   {}", package.files.len());

    // Verify a security.json with checksums for every covered file passes.
    let index_checksum = capsium_core::integrity::sha256_hex(&files["content/index.html"]);
    let metadata_checksum = capsium_core::integrity::sha256_hex(&files["metadata.json"]);
    let security_json = format!(
        r#"{{"security":{{"integrityChecks":{{"checksumAlgorithm":"SHA-256","checksums":{{"content/index.html":"{}","metadata.json":"{}"}}}}}}}}"#,
        index_checksum, metadata_checksum
    );
    let security = Security::from_json(&security_json).unwrap();
    let report = security.verify(&files);
    println!("integrity valid (clean): {}", report.valid());

    // Now tamper and verify the report flags it.
    files.insert("content/index.html".to_string(), b"<h1>nope</h1>".to_vec());
    let bad = security.verify(&files);
    println!("after tamper:    {} issues", bad.issues.len());
    assert!(bad
        .issues
        .iter()
        .any(|i| matches!(i, IntegrityIssue::Mismatch { .. })));
    println!("OK — mismatch detected");
}

//! wasm-bindgen tests for the JS-facing API in src/wasm.rs.
//! Run via `wasm-pack test --node --features wasm`.

#![cfg(all(test, feature = "wasm"))]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node);

#[wasm_bindgen_test]
fn sha256_of_known_input() {
    let bytes = b"hello";
    let got = capsium_core::wasm::sha256(bytes);
    assert_eq!(
        got,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[wasm_bindgen_test]
fn parse_metadata_returns_name() {
    let text = r#"{"name":"demo","version":"0.1.0","guid":"https://example.com/demo"}"#;
    let value = capsium_core::wasm::parse_metadata(text).unwrap();
    let obj = js_sys::Object::from(value);
    let name = js_sys::Reflect::get(&obj, &"name".into()).unwrap();
    assert_eq!(name.as_string().unwrap(), "demo");
}

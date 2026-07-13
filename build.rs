//! Build-time codegen of the HSM HTTP client from the committed
//! OpenAPI 3.0 spec via progenitor.

use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let spec_path = manifest_dir.join("src/hsm/ochami_api_docs.openapi3.json");
    let out_path = out_dir.join("hsm_generated.rs");

    println!("cargo:rerun-if-changed={}", spec_path.display());

    let src = fs::read_to_string(&spec_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spec_path.display()));

    let mut spec: openapiv3::OpenAPI = serde_json::from_str(&src)
        .unwrap_or_else(|e| panic!("parse {} as JSON: {e}", spec_path.display()));

    // Wrap info.description in ```text fence so rustdoc doesn't try to
    // compile embedded markdown code fences as doctests.
    if let Some(desc) = spec.info.description.as_mut() {
        let escaped = desc.replace("```", "``\u{200B}`");
        *desc = format!("```text\n{escaped}\n```");
    }

    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap_or_else(|e| {
        panic!(
            "progenitor codegen failed for {}: {e}. \
             Check the spec is valid OpenAPI 3.0 and contains no \
             progenitor-unsupported constructs.",
            spec_path.display()
        )
    });
    let ast: syn::File = syn::parse2(tokens).expect("generated tokens do not parse");
    let pretty = prettyplease::unparse(&ast);

    fs::write(&out_path, pretty)
        .unwrap_or_else(|e| panic!("write {}: {e}", out_path.display()));
}

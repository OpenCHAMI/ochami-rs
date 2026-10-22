# ochami-rs

[![CI](https://github.com/OpenCHAMI/ochami-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenCHAMI/ochami-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ochami-rs.svg)](https://crates.io/crates/ochami-rs)
[![Docs.rs](https://docs.rs/ochami-rs/badge.svg)](https://docs.rs/ochami-rs)
[![License](https://img.shields.io/crates/l/ochami-rs.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.99.1-blue)](https://www.rust-lang.org/)

Rust bindings for the [OpenCHAMI](https://openchami.org/) backend
(HSM, BSS, PCS). The crate implements the
[`manta-backend-dispatcher`](https://crates.io/crates/manta-backend-dispatcher)
traits, so it can be used as a drop-in OpenCHAMI backend for tools built
against that dispatcher (e.g. [Manta](https://github.com/eth-cscs/manta)),
or called directly.

## Status

`1.0.0-beta.16` is a **pre-1.0 beta**: breaking changes are still
expected between beta releases. If you're upgrading from
`1.0.0-beta.15`, read [`MIGRATION.md`](MIGRATION.md) first — the HSM
client surface changed substantially (see below). See
[`CHANGELOG.md`](CHANGELOG.md) for the full per-release history.

## Install

```toml
[dependencies]
ochami-rs = "1.0.0-beta.16"
```

## Usage

See the [`examples/`](examples/) directory for runnable end-to-end
demos (component listing, PCS power status, BSS boot-parameter
read-modify-write). Each reads connection settings from
`OCHAMI_URL` / `OCHAMI_TOKEN` / `OCHAMI_CA` and can be launched with
`cargo run --example <name>`.

The entry point is the [`Ochami`](src/backend_connector.rs) client,
constructed with a base URL and a root CA certificate (PEM bytes):

```rust,no_run
use ochami_rs::Ochami;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_cert = std::fs::read("/path/to/ca.pem")?;
    let ochami = Ochami::new("https://ochami.example.com", &root_cert);

    let token = "your-bearer-token";

    // Fetch every HSM component of type "Node".
    let components = ochami
        .hsm_component_get_all_nodes(token, None)
        .await?;

    println!("{components:#?}");
    Ok(())
}
```

Wrapper methods are grouped by HSM resource family and named
`hsm_<resource>_<verb>`, e.g. `hsm_component_get_one`,
`hsm_group_post`, `hsm_partition_get_all`. Most of them are thin async
methods on `Ochami` that return `Result<T, ochami_rs::error::Error>`.

As of beta.16, the HSM client surface (`src/hsm/`) is generated at
build time from a vendored OpenAPI spec via
[`progenitor`](https://docs.rs/progenitor) — see
[`src/hsm/mod.rs`](src/hsm/mod.rs)'s module docs for the codegen
pipeline, and [`CONTRIBUTING.md`](CONTRIBUTING.md) for how to
regenerate it against a newer upstream spec.

## Upgrading

- Migrating from `1.0.0-beta.15` → `1.0.0-beta.16`: see
  [`MIGRATION.md`](MIGRATION.md) for the full breaking-change guide
  (method surface, field types, return types, and a handful of silent
  URL/behavior fixes), plus before/after code samples.
- Full release history: [`CHANGELOG.md`](CHANGELOG.md).

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for build requirements, the
spec-regeneration procedure, and local test instructions.

## License

See [`LICENSE`](LICENSE).

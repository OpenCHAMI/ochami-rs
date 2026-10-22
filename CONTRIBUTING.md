# Contributing

## Requirements

- **Rust** `>= 1.99.1` (the crate's declared `rust-version` / MSRV in
  `Cargo.toml`). Ordinary `cargo build`/`cargo test` work with any
  toolchain at or above this version.
- **Node.js** (any recent LTS) — only needed to regenerate the HSM
  OpenAPI spec via `npx swagger2openapi` (see below). Not required for
  a normal build/test cycle, since the converted spec
  (`src/hsm/ochami_api_docs.openapi3.json`) is committed.
- **jq** — only needed for the same spec-regeneration step
  (`scripts/patch-hsm-spec.jq` patches the converted spec before
  `progenitor` consumes it).

## Local build & test

```sh
cargo build
cargo test
cargo doc --no-deps
```

`cargo build` runs `build.rs`, which reads the committed
`src/hsm/ochami_api_docs.openapi3.json` and generates the HSM HTTP
client (`$OUT_DIR/hsm_generated.rs`) via `progenitor` on every build.
This adds roughly 15-25s to a clean/cold build; incremental rebuilds
are unaffected unless the spec JSON or `build.rs` itself changes.

Tests use [`wiremock`](https://docs.rs/wiremock) to stand up a local
mock HTTP server per test file — no network access or real OCHAMI
deployment is required to run the test suite.

## Regenerating the HSM OpenAPI spec

`src/hsm/ochami_api_docs.yaml` is a vendored copy of HSM's upstream
Swagger 2.0 spec (see the file's header comment for provenance and
retrieval date). To pull in an updated upstream spec:

1. Replace `src/hsm/ochami_api_docs.yaml` with the new upstream
   version, and update its provenance header comment (retrieval date,
   any relevant upstream commit/tag reference).
2. Run:

   ```sh
   make convert-spec
   ```

   This is defined in the repository `Makefile` as:

   ```make
   convert-spec:
       npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml -o /tmp/hsm-raw.json
       jq -f scripts/patch-hsm-spec.jq /tmp/hsm-raw.json > src/hsm/ochami_api_docs.openapi3.json
       rm /tmp/hsm-raw.json
   ```

   It upgrades the Swagger 2.0 doc to OpenAPI 3.0 via
   `swagger2openapi`, then applies the jq patches in
   `scripts/patch-hsm-spec.jq` (each patch documents, in a comment
   above it, the spec defect it works around — e.g. an overly-narrow
   `ResourceName` regex, or `/locks/*` endpoints missing
   `operationId`). The result overwrites the committed
   `src/hsm/ochami_api_docs.openapi3.json`.
3. Review the JSON diff (`git diff src/hsm/ochami_api_docs.openapi3.json`)
   for unexpected schema/path changes.
4. Run `cargo build` and fix any new compile errors in `src/hsm/`
   (renamed/removed generated types or methods propagate as regular
   Rust compile errors).
5. Run `cargo test` and update wiremock test fixtures if response
   shapes changed.
6. Check whether any `scripts/patch-hsm-spec.jq` patch is now
   unnecessary (i.e. upstream fixed the underlying issue) and drop it
   if so.
7. Bump the crate version and add a `CHANGELOG.md` / `MIGRATION.md`
   entry if the regeneration introduced breaking changes.

**Firewalled/offline environments**: `make convert-spec` requires
network access (`npx` fetches `swagger2openapi` from the npm registry
on first run). If you're offline, you only need this step when
actually regenerating the spec from a new upstream source — normal
development against the already-committed
`src/hsm/ochami_api_docs.openapi3.json` needs no network access at
build or test time.

## Dev dependencies

- [`wiremock`](https://crates.io/crates/wiremock) — HTTP mocking for
  the wrapper-method test suite (`[dev-dependencies]` only).
- `progenitor`, `openapiv3`, `serde_json`, `prettyplease`, `syn` —
  `[build-dependencies]`, used by `build.rs` for HSM client codegen.

## Release process

1. Bump `version` in `Cargo.toml`.
2. Update `CHANGELOG.md` with a new `[x.y.z] - YYYY-MM-DD` entry.
3. If the release includes breaking changes, update `MIGRATION.md`.
4. `cargo test` and `cargo doc --no-deps` must both be clean.
5. `cargo publish --dry-run` — verify the `.crate` tarball includes
   `src/hsm/ochami_api_docs.openapi3.json` (required at consumer build
   time by `build.rs`).
6. `cargo publish`.
7. Tag the release commit (`vX.Y.Z`) and push tags.

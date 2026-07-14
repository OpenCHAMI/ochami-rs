# ochami-rs examples

Runnable end-to-end examples that exercise the crate against a real
OpenCHAMI deployment. Each example reads its connection settings from
environment variables so the same binary can point at any deployment:

| Var | Meaning | Example |
|---|---|---|
| `OCHAMI_URL` | Base URL of the OpenCHAMI ingress | `https://ochami.example.com` |
| `OCHAMI_TOKEN` | Bearer token accepted by the OCHAMI backends | `eyJhbGciOiJI...` |
| `OCHAMI_CA` | Path to a PEM file with the CA certificate the deployment presents | `/etc/pki/ca-trust/source/anchors/openchami.pem` |

Run any example with:

```bash
OCHAMI_URL=https://ochami.example.com \
OCHAMI_TOKEN="$(cat token.txt)" \
OCHAMI_CA=/path/to/ca.pem \
cargo run --example fetch_components
```

## Available examples

- **[`fetch_components.rs`](fetch_components.rs)** — list every HSM
  component of type `Node`, printing xname and current power state.
  Demonstrates: `Ochami::new`, `hsm_component_get_all_nodes`,
  structured error handling on `Error::OchamiError { status, .. }`.
- **[`power_status.rs`](power_status.rs)** — query PCS for the power
  state of one or more xnames (pass them as trailing CLI args).
  Demonstrates: `pcs_power_status_post`, per-xname result iteration.
- **[`boot_params_upsert.rs`](boot_params_upsert.rs)** — read the
  current boot parameters for an xname (via `bss_bootparameters_get`),
  print them, and update the kernel image with a placeholder value
  (via `bss_bootparameters_patch`). Demonstrates the BSS
  read-modify-write pattern.

None of these examples run in CI (they require a live OCHAMI backend
and valid credentials); they are here as an executable form of the
crate's usage documentation.

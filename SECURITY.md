# Security Policy

## Supported Versions

The latest `1.0.0-beta.*` release published on [crates.io](https://crates.io/crates/ochami-rs)
is the only supported version. Older beta releases receive no security updates —
upgrade to the latest.

## Reporting a Vulnerability

Please report security vulnerabilities privately by emailing:

**masber@hotmail.com**

Include:
- A short description of the issue.
- Steps to reproduce (a proof-of-concept when practical).
- Affected versions.
- Any suggested mitigation.

You should receive an acknowledgement within 5 business days. If you do not,
please follow up.

## Scope

- **In scope:** bugs in `ochami-rs` itself, and vulnerabilities in the
  dependency tree that `ochami-rs` exposes to consumers.
- **Out of scope:** bugs in the OpenCHAMI SMD/BSS/PCS server (report those to
  the [OpenCHAMI project](https://github.com/OpenCHAMI)), or consumer misuse of
  the client.

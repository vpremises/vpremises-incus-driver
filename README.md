# vpremises-incus-driver

Render reviewed placement declarations into OpenTofu configuration for Incus.

## What you can do

- Validate resolved VM or container requirements.
- Generate provider configuration for review before deployment.

## Current scope

Rendering does not run OpenTofu or create Incus resources. The operator supplies the deployment credentials and approval.

Package distribution is not activated by this documentation. Use the checked-in source and the declared dependency versions; published availability must be verified separately.

## Getting started

Install Rust 1.97 or newer and make the declared dependencies available. Use the configured private registry when a dependency is not distributed publicly. Run from this repository:

```sh
cargo test --locked
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

### Apply-time user validation

The renderer accepts only non-root ASCII account names matching
`[a-z_][a-z0-9_-]{0,31}`. Whitespace, control characters, YAML syntax and
uppercase names are rejected. Cloud-init is emitted as a structured JSON
(YAML-compatible) document with a locked password and `nologin` shell.
Before applying the generated OpenTofu plan, the operator must check that the
chosen image does not map this account to UID 0 and review the project, image,
network and storage placement. Rendering alone does not prove a safe deployment.

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

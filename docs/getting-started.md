# Using vpremises-incus-driver

Render reviewed placement declarations into OpenTofu configuration for Incus.

## Before you start

Rendering does not run OpenTofu or create Incus resources. The operator supplies the deployment credentials and approval.

## First steps

Run from the repository root:

```sh
cargo test --locked
```

## How to assess the result

- Validate resolved VM or container requirements.
- Generate provider configuration for review before deployment.

A passing source-level check establishes only what that check observes. Keep missing configuration, unavailable services and unverified deployment paths visible.

## Continue reading

[Repository overview](../README.md)

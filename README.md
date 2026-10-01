# vPremises Incus Driver

This repository validates a resolved vPremises environment and renders
OpenTofu JSON for the official `lxc/incus` provider. The generated declaration
contains one restricted project, one resource-limited profile, and one
non-ephemeral VM or system container. It never accepts a provider credential
or exposes the Incus API.

```bash
cargo run -- render tests/fixtures/resolved.json /tmp/main.tf.json
```

The cloud-init payload creates only a locked service account. It deliberately
does not upgrade packages or embed an SSH key. Recurring convergence belongs
to the idempotent Ansible roles in `vpremises-infrastructure`.

OpenTofu is not installed or executed by the renderer. An operator first
reviews the generated JSON, initializes the provider lock file, runs a plan,
and only then performs an approved apply. `scripts/plan` requires an existing
read-only dependency lock and never calls `apply`.

Incus projects isolate profiles and storage metadata. The generated project is
restricted, disallows destructive project removal, and attaches only the
declared private network and root storage pool.


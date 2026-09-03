# #622 Provider/profile hot-loading production wiring

## Goal

Make validated provider-instance and inference-profile changes available to
subsequent inference requests without restarting the consuming process.

## Design

Add one `ProviderReloadOwner` in `adl::provider::reload`. It reuses the
Runtime kernel's existing generic configuration watcher rather than introducing
new watch, debounce, hashing, or cancellation mechanics. The watched sidecar is
a provider-only ADL document: it may contain provider definitions and profiles,
but no executable steps, agents, credentials, authority grants, or secrets.

On each changed file, the owner parses the complete candidate, validates the
provider-only boundary, expands profiles through the existing profile registry,
and calls the existing last-known-good activation logic. A successful candidate
is published as one immutable `Arc` snapshot with a stable redacted digest. A
rejected candidate preserves the entire prior snapshot and emits only a bounded
reason code.

The production execution runner accepts the reload owner as its provider source.
Each step obtains one snapshot before provider construction and retains it for
the complete inference call. A later step may observe a newly accepted snapshot;
an in-flight call cannot be rewritten.

The sidecar may refer to credential environment-variable names or governed
credential references. It may not contain credential values. Credentials,
signing keys, authority objects, database pools, and model weights remain
restart-gated or separately governed.

## Outputs

- `adl/src/provider/reload.rs`
- production execution-runner integration
- focused integration and negative tests
- `docs/providers/provider-profile-hot-loading.md`

## Boundary

This issue wires already-designed mechanisms. It does not redesign providers,
add MLX, package OCI images, introduce automatic tuning, or create a provider
control plane.

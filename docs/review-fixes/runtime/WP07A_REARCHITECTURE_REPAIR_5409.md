# WP-07A Runtime Rearchitecture Repair (#5409, #5494)

## Boundary

PR #5420 established useful component, supervision-policy, channel, and
credential-renewal contracts, but its retained proof overstated completion.
The production daemon supervises one real `long_lived_agent_tick` cycle; it
does not spawn sixteen independent component tasks. This repair makes that
execution model explicit and removes the static all-ready projection.

The component catalog remains the policy and observation contract. The live
runtime API normalizes observed component health and fails readiness closed
when a required component or typed channel is missing or unhealthy. Cloud
bridge and observability remain explicitly degradable under their existing
supervision policies.

The Runtime v3 host-weather service remains separately implemented in
`adl-runtime/src/weather.rs`; this repair does not duplicate it or add another
weather service.

## Implemented Proof Surface

- `adl-runtime/src/topology.rs`
  - reports the real daemon-supervised-cycle execution model;
  - no longer claims static component readiness or independent component tasks;
  - runs 100 real supervised cycles over a Tokio channel, injects one failure,
    proves restart/recovery, and replays the retained lifecycle journal.
- `adl/src/csm_runtime_api.rs`
  - projects observed health for all sixteen catalog components;
  - derives required-component readiness from supervision policy;
  - checks each required typed-channel observation rather than trusting only a
    top-level status string.
- `adl-runtime/src/runtime_api_auth.rs`
  - retains one previous bearer generation for a bounded five-minute overlap;
  - rejects the previous generation after overlap expiry;
  - clears both generations on terminal revocation;
  - retains redacted rotation and revocation events without credential material.

## Validation

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-runtime \
  cargo test --manifest-path adl-runtime/Cargo.toml
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-adl \
  cargo test --manifest-path adl/Cargo.toml csm_runtime_api
git diff --check
```

Observed locally:

- `adl-runtime`: 122 unit tests and 1 independence test passed.
- integrated CSM runtime API: 44 focused tests passed; the added focused
  required-component/channel assertion passed after that full focused run.
- assembled behavioral soak: 100 supervised cycles, 101 task executions and
  channel deliveries, one injected restart, zero invalid replay lines.

This local proof does not claim a live external provider, cloud, API Gateway,
GPU, or Runtime v3 integration run. It does not override the separately
governed #4906 coherence gate.

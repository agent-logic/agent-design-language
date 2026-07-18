# WP-07A Runtime Rearchitecture Repair (#5409, #5494)

## Boundary

PR #5420 established useful component, supervision-policy, channel, and
credential-renewal contracts, but its retained proof overstated completion.
The production daemon supervises one real `long_lived_agent_tick` cycle; it
does not spawn sixteen independent component tasks. This repair makes that
execution model explicit and removes the static all-ready projection.

The fifteen-component CSM catalog remains the policy and observation contract.
The live runtime API normalizes observed component health and fails readiness
closed when a required component or policy-required typed channel is missing
or unhealthy. Cloud bridge and observability components remain explicitly
degradable under their existing supervision policies, while their Audit and
Evidence channels remain required by channel policy.

The Runtime v3 host-weather service remains separately implemented in
`adl-runtime/src/weather.rs`; this repair does not duplicate it or add another
weather service.

## Implemented Proof Surface

- `adl-runtime/src/topology.rs`
  - reports the real daemon-supervised-cycle execution model;
  - no longer claims static component readiness or independent component tasks;
  - excludes Runtime v3-owned weather from the CSM component assembly.
- `adl/src/long_lived_agent.rs`
  - gives production and tests one shared `run_daemon_cycle` path;
  - runs 100 real ticks through the production typed-channel fabric, injects
    one workflow failure, and proves recovery on the same runtime context.
- `adl/src/csm_runtime_api.rs`
  - projects observed health for all fifteen CSM catalog components;
  - derives required-component readiness from supervision policy;
  - checks each required typed-channel observation rather than trusting only a
    top-level status string.
- `adl-runtime/src/runtime_api_auth.rs`
  - retains one previous bearer generation for a bounded five-minute overlap;
  - automatically recovers an expired non-revoked generation without overlap;
  - serializes creation, rotation, renewal, and revocation with the existing
    `fs2` lock so terminal revocation cannot be overwritten concurrently;
  - rejects the previous generation after overlap expiry;
  - clears both generations on terminal revocation;
  - retains redacted rotation and revocation events without credential material.

## Validation

```text
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-runtime \
  cargo test --manifest-path adl-runtime/Cargo.toml
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-adl \
  cargo test --manifest-path adl/Cargo.toml csm_runtime_api
CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/wp-5494-adl \
  cargo test --manifest-path adl/Cargo.toml \
  long_lived_agent::tests::production_daemon_cycle_soak_runs_real_ticks_channels_and_recovery \
  -- --exact
git diff --check
```

Observed locally:

- `adl-runtime`: 123 unit tests and 1 independence test passed, including 10
  focused credential-lifecycle tests.
- integrated CSM runtime API: 44 focused tests passed.
- production daemon-cycle soak: 100 completed real ticks, all seven typed
  channel observations ready, one injected workflow failure, and recovery on
  the same runtime context; test execution completed in 27.78 seconds.

This local proof does not claim a live external provider, cloud, API Gateway,
GPU, or Runtime v3 integration run. It does not override the separately
governed #4906 coherence gate.

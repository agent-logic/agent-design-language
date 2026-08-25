# Issue 471 design: authoritative Runtime v3 kernel contracts

Status: approved for implementation

## Objective

Make Runtime v3 topology, ports, lifecycle, supervision, determinism, backpressure,
and health machine-authoritative. The kernel must reject assemblies whose declared
contracts differ from the channels and component behavior it actually runs.

## Decisions

1. Version the service contract schema to `adl.runtime.service_contract.v2`.
2. Replace diagnostic Rust type names with stable protocol identifiers and add
   explicit channel capacity and overflow policy to every declared port.
3. Build channels through a kernel-owned port registry. Components obtain only
   declared typed endpoints from `ComponentContext`; missing, duplicate, wrong
   direction, wrong protocol, and undeclared ports fail before task spawn.
4. Enforce determinism at the component boundary: deterministic components receive
   deterministic kernel inputs only; nondeterministic access requires an explicit
   declared capability and is recorded in lifecycle state.
5. Start independent components concurrently by topological layer. On shutdown,
   use contract-bound lifecycle roles to stop ingress first, drain workloads,
   flush checkpoints and telemetry, then close egress within a bounded deadline.
6. Supervision uses a time-window restart budget, applies policy to readiness
   failures, supports one-for-one and one-for-all scopes, and propagates degraded
   capability state to dependents. Factories explicitly declare required-core
   membership; required-core degradation is terminal.
7. Runtime health is an explicit aggregate of component readiness, liveness,
   restart window, capability state, queue pressure, and shutdown phase.
8. Channel metrics use poison-free atomics. No telemetry failure may poison or
   terminate the data path.

## Safety boundaries

- No compatibility inference from closure captures is authoritative.
- No unbounded queue, restart loop, startup wait, drain, or shutdown phase.
- No silent component death: every terminal/degraded state is observable.
- Existing Runtime public APIs remain compatible unless the versioned contract
  explicitly rejects an invalid assembly.
- This issue does not modify WP-27, distributed consensus, cloud infrastructure,
  model/provider selection, or Runtime v4 planning.

## Implementation slices

1. Contract v2 and kernel port registry.
2. Determinism and lifecycle enforcement.
3. Layered startup and staged shutdown.
4. Restart-window and supervision-scope policy.
5. Health projection and poison-free channel metrics.

## Proof

- Contract-negative tests for missing, duplicate, wrong-direction, wrong-protocol,
  undeclared, unbounded, and ambiguous port wiring.
- Determinism denial tests and lifecycle transition tests.
- Parallel-layer startup and reverse-layer staged shutdown ordering tests.
- Restart-window reset, readiness-failure policy, one-for-all, degradation
  propagation, required-core failure, and bounded-shutdown tests.
- Channel contention/telemetry safety tests and Runtime health projection tests.
- Existing kernel suites, strict Clippy, formatting, and diff hygiene.

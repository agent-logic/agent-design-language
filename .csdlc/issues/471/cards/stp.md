# Structured Task Prompt

Template: 1.0.0

Issue: 471

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Runtime v3 kernel architecture remediation only; no WP-27, cloud, provider, consensus, UI redesign, or Runtime v4 work.

## Deliverables

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/channel.rs
- adl-runtime-kernel/src/cognition.rs
- adl-runtime-kernel/src/component.rs
- adl-runtime-kernel/src/contract.rs
- adl-runtime-kernel/src/governance.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/proof.rs
- adl-runtime-kernel/src/reasoning.rs
- adl-runtime-kernel/src/supervisor.rs
- adl-runtime-kernel/src/telemetry.rs
- adl-runtime-kernel/src/time.rs
- adl-runtime-kernel/src/topology.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/contracts.rs
- adl-runtime-kernel/tests/kernel.rs
- adl-runtime-kernel/tests/production_acip_wss.rs
- adl-runtime-kernel/tests/reasoning.rs
- .csdlc/prepared/issues/471/design.md
- .csdlc/prepared/issues/471/diagram.mmd
- .csdlc/prepared/issues/471/validate-runtime-kernel.sh

## Acceptance

1. AC-1: Kernel-owned typed port registry makes declared wiring authoritative and rejects missing, duplicate, wrong-direction, wrong-protocol, and undeclared ports before spawn.
2. AC-2: Protocol identifiers, capacity, and overflow policy are stable versioned contract fields rather than Rust diagnostic names or hidden channel settings.
3. AC-3: Determinism declarations are enforced against explicit nondeterministic capabilities and lifecycle evidence.
4. AC-4: Independent components start concurrently by topological layer and failure is bounded and deterministic.
5. AC-5: Shutdown stops ingress first and drains dependents before dependencies, then flushes checkpoints and telemetry within bounded deadlines.
6. AC-6: Restart budgets are time-windowed and readiness failures follow the same declared failure policy as runtime failures.
7. AC-7: One-for-one and one-for-all supervision scopes are implemented using topology relationships.
8. AC-8: Degradation propagates capability loss to dependents and required-core degradation is terminal.
9. AC-9: Runtime health reports component readiness/liveness/restarts/capability/queue/shutdown state.
10. AC-10: Channel metrics are poison-free and cannot terminate the data path.
11. AC-11: Focused tests, existing kernel suites, strict Clippy, format, diff hygiene, and independent exact-head review pass.

## Dependencies

- none

## Inputs

- GitHub issue #471
- operator-provided Runtime architecture review
- adl-runtime-kernel current source and tests
- Runtime v3 public API and operations surfaces

## Non Goals

- WP-27 issue #315
- Runtime v4
- distributed consensus
- AWS or GCP
- model/provider changes
- Observatory redesign

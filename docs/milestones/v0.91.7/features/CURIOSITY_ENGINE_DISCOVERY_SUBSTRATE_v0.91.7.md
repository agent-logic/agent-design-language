# Curiosity Engine / Discovery Substrate

## Metadata

- Feature Name: Curiosity Engine / Discovery Substrate
- Milestone Target: `v0.91.7`
- Status: bounded runtime core implemented
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, artifact
- Proof Modes: review, replay, tests

## Purpose

Define the governed curiosity substrate required before `v0.92` can consume
curiosity as an active cognitive feature.

## Scope

In scope:

- curiosity artifacts and event records;
- detection hooks and surprise/novelty signals;
- hypothesis and experiment planning;
- discovery budgets and governance;
- Freedom Gate integration;
- ObsMem and reasoning-graph update expectations;
- first governed discovery-cycle proof.

Out of scope:

- broad autonomous exploration;
- autonomous external action;
- WP-07A CSM supervisor/component hosting;
- public claims that curiosity is fully solved.

## Runtime Status

As of `v0.91.7`, WP-10 issue `#4692` implements a bounded Runtime v2
Curiosity Engine core in `adl/src/runtime_v2/curiosity_engine.rs`.

The runtime core emits a deterministic curiosity packet with:

- explicit novelty/surprise signals;
- budgeted discovery proposals;
- Freedom Gate, CAV, operator-review, and Constructability gates;
- reasoning-graph, ObsMem, trace, and constructability handoff references;
- replay guarantees and non-claims.

The packet is available through:

```sh
adl runtime-v2 curiosity-engine --out artifacts/v0917/curiosity-engine.json
```

This is the host-agnostic curiosity core. WP-07A issues `#5124` and `#5125`
remain the CSM runtime-component hosting path for supervision, typed channels,
lifecycle integration, and Constructability component placement.

## Required Decisions

- Which events create curiosity artifacts?
- Which budgets and gates constrain curiosity actions?
- Which discovery cycle proves useful behavior before `v0.92`?
- Which findings update ObsMem, reasoning graphs, or issue plans?

## Dependencies

- Constructability Gate feature doc.
- Reasoning graph / skill-standard implementation.
- Security implementation readiness.

## Validation And Review

- Review discovery-cycle artifacts and budget enforcement.
- Require the focused Runtime v2 proof command before `v0.92` consumes
  Curiosity:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_curiosity_engine -- --nocapture
cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_curiosity_engine -- --nocapture
adl/target/debug/adl runtime-v2 curiosity-engine --out .adl/local-artifacts/wp10-curiosity/curiosity-engine.json
```

- Block any curiosity claim that lacks governance with evidence and operator
  approval.

## v0.92 Consumption

`v0.92` may consume Curiosity as a bounded governed-discovery core after
`#4692` merges. It may not claim CSM runtime-component hosting until the
WP-07A component path consumes this core and proves supervision/channel
integration.

## Non-Goals

- No unbounded exploration.
- No personhood or inner-state claims.
- No runtime completion claim from this doc.

# v0.93 Runtime v4 CSM Plugin System

## Status

Runtime v4 must be completed in v0.93, as explicitly directed by the operator on 2026-09-16. RV-01 through RV-08 are mandatory release scope. The prior design is not automatically approved: RV-01 must resolve its review findings before implementation admission.

## Purpose

Make CSM modules independently described and governed through one plugin contract while preserving the Runtime supervisor, identity, typed ports and policy. WASM is an execution adapter, not the CSM semantic model.

## Source Inputs

The local Runtime v4 design and Claude/Gemini reviews were inspected under #1047. Their scope and dispositions are retained in the source inventory and decision register. Tracked foundations are `docs/architecture/RUNTIME_V3_SERVICE_CONTRACT_ARCHITECTURE.md`, `docs/architecture/RUNTIME_V3_OPERATIONAL_COMPONENTS.md` and `adl-runtime-kernel/src/component.rs`. These are pre-extraction paths; RD-02 maps their destination.

## Design

One supervisor owns plugin state. Native, process and WASM adapters implement the same declared lifecycle and protocol; native Rust components are rebuilt with their Runtime generation, not loaded through an assumed stable Rust dynamic ABI. Typed CSM schemas remain authoritative over WIT projections.

The following are proposed corrections, not claims of implemented behavior:

- Manifest fields express requested capabilities and maxima. Topology binds routes and failure policy; operator policy supplies ceilings. Effective grants are the intersection of request, topology and policy and cannot be enlarged by a plugin.
- Every capability handle, route, output and checkpoint write carries a monotonically fenced generation epoch. Stale generations are rejected at host boundaries.
- A durable activation journal records prepared, quiescing, switched and accepted/rolled-back transitions. Boot recovery selects exactly one valid active generation; ambiguity fails closed. Rollback requires an intact prior checkpoint or explicitly reviewed reverse migration.
- Quiesce rejects new admission and drains accepted work within a bounded deadline. The contract records either explicit failed delivery with evidence or at-least-once redelivery with host idempotency keys; it must not promise exactly-once side effects. Partially emitted outputs and undrained queues need explicit replay/disposition rules.
- Each handler and host call receives deadline/cancellation context. Revocation blocks new calls immediately and applies a declared in-flight cancellation policy. A policy change recomputes effective grants; it cannot silently leave old authority active.
- Recovery classes distinguish stateless, rebuild-on-start, durable-checkpoint and explicitly lossy restart. The Runtime rejects lossy plugins where topology requires durable continuity and reports loss visibly.
- Versioned state and configuration migrations stage without changing active state, prove at least one successful migration, and restore the prior generation after partial failure. Two-phase configuration reload validates before atomic commit. Removal rejects live dependents or drains them in declared dependency order; refusal preserves the prior active graph. RV-03 owns these behaviors and RV-08 repeats their installed recovery cases.
- Process supervision covers authentication, bounded queues, orphan reaping and termination. WASM denies ambient imports and enforces memory, time, concurrent calls and outstanding host-call limits.

## Review dispositions and scope gate

Both reviews found undefined in-flight delivery and cancellation. Claude additionally flagged missing fencing and dual policy authority; Gemini's optimistic verdict conflicts with its own severe findings. No provider verdict is treated as design approval. RV-01 must independently review these proposed corrections.

RV-01 through RV-03 establish and qualify the native generation lifecycle before non-native adapters proceed. RV-04 through RV-08 must then complete process and WASM adapters, cross-adapter equivalence, provider integration and installed recovery qualification in v0.93. An intermediate native-only acceptance cannot satisfy the milestone release gate. No cluster-wide activation, automatic registry download, ambient credentials or unreviewed state migration is admitted.

## Acceptance Criteria

A live installed consumer must prove fenced generation changes, crash recovery, bounded delivery and cancellation, preserved state identity and rollback. Schema examples and adapter mocks alone cannot close implementation rows. Cross-adapter parity covers failure, authority, message ordering and state, not only successful outputs.

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #1047. Status: first-pass planning; implementation and release acceptance not claimed.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-runtime. Candidate results: RV-01, RV-02, RV-03, RV-04, RV-05, RV-06, RV-07, RV-08. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **RV-01:** An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes.
- **RV-02:** Existing Runtime behavior and identity survive native adapter conversion with no second scheduler.
- **RV-03:** Staging, bounded quiesce, versioned state/config migration, two-phase reconfiguration, dependency-safe removal, activation and rollback recover to one fenced generation after every interruption; prove a successful migrator and preserve the prior generation when migration/reconfiguration/removal is refused.
- **RV-04:** One real non-core plugin uses authenticated bounded transport, deadlines and process supervision.
- **RV-05:** The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits.
- **RV-06:** One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters.
- **RV-07:** Provider instances consume host-owned credentials and versioned configuration through the common inference capability.
- **RV-08:** Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback.

## Execution Flow

- RV-01 follows RD-11.
- RV-02 follows RV-01.
- RV-03 follows RV-02.
- RV-04 follows RV-03.
- RV-05 follows RV-04.
- RV-06 follows RV-05.
- RV-07 follows RV-06.
- RV-08 follows RV-07.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

RV-01 uses independent design/contract review over explicit success, refusal and recovery scenarios. Resolve manifest self-grants and undefined crash recovery in that contract before admitting RV-02; installed implementation evidence is not a prerequisite for this design decision.

RV-02 through RV-08 run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:
- RV-02: Behavior parity regression.
- RV-03: Stale output/checkpoint writer; Crash at switch; Lost or duplicated accepted work; Migration fails after partial progress; Invalid reconfiguration; Removal while dependents remain.
- RV-04: Orphan process; Hung handler; Unbounded queue.
- RV-05: Unapproved import; Timeout bypass.
- RV-06: Adapter-specific authority; State identity drift.
- RV-07: Credential in manifest; Implicit provider fallback.
- RV-08: Bad signature; Incompatible checkpoint; Rollback cannot restore.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

RV-01, RV-02, RV-03, RV-04, RV-05, RV-06, RV-07, RV-08; owner repositories: agent-logic-runtime. The current execution graph supersedes older sequencing or placement in retained source text.

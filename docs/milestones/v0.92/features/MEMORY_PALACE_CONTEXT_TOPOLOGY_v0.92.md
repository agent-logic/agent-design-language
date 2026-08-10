# v0.92 Feature: Memory Palace Context Topology

## Metadata

- Feature Name: Memory Palace Context Topology
- Milestone Target: `v0.92`
- Status: implementation required in WP-11
- Owner: ADL maintainers
- Doc Role: primary
- Supporting Docs:
  - `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
  - `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`
- Feature Types: architecture, artifact, policy
- Proof Modes: review, schema, tests

## Template Rules

This is the WP-11 implementation contract. Runtime behavior, schema, negative
tests, and retained proof are required before the feature is complete.

## Status

Implemented by WP-11 as an additive Runtime v3 kernel boundary. The contract
remains incomplete until exact-head local and native proof, independent review,
and publication are recorded; planning text alone cannot satisfy WP-11.

## Purpose

Define the first reviewable Memory Palace slice as a navigable context topology
for long-running agents. The goal is to reduce context-loss risk by separating
durable memory, active working set, retrieval hints, and reviewable context
packets.

## Source Inputs

- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Context

Memory Palace is the planned answer to long-running context loss. It must build
on ObsMem and memory grounding rather than replacing them or treating chat
history as authoritative state.

## Coverage / Ownership

Primary owner doc: this document.

Covered surfaces:

- Memory Palace boundary and topology terms
- relation to ObsMem, active working set, and context cache
- first proof expectations for long-running context continuity

Related / supporting docs:

- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`

## Overview

Memory Palace should provide a navigable map of durable context anchors,
working-set references, and retrieval paths that can be reviewed and resumed.

## Scope

This feature should establish:

- the boundary between ObsMem, Memory Palace, active working set, and context
  cache
- a minimal palace topology for named rooms, anchors, references, and traversal
  hints
- redaction-safe context packet expectations for review and handoff
- continuity hooks so a long-running agent can resume from durable context
  without treating chat transcript state as authoritative
- proof expectations for retrieval, summarization, provenance, and stale
  context detection

## Design

### Runtime v3 contract

`adl-runtime-kernel::memory_palace` consumes the durable WP-09 identity record,
the WP-10 continuity record, one exact Runtime v3 trace reference, a redaction
policy digest, a fixed observation time, and normalized ObsMem-shaped records.
It recomputes both predecessor record digests and rejects identity-root,
continuity-head, trace, citation, temporal, or redaction substitutions before
materializing context.

The canonical output binds those authorities into `authority_sha256`, sorts
rooms, records, and citations deterministically, and produces a JCS-hashed
context packet. `max_working_set_items` is bounded to 1–64. Additional valid
records become digest-bearing overflow entries and are never silently loaded.
Only public summaries and the exact literal `[REDACTED]` may enter the working
set; raw/private memory, host paths, parent traversal, secret-like payloads,
and unbound trace citations fail closed.

### Core Concepts

- Palace topology: named spaces, anchors, paths, and summaries.
- Working set: the bounded context currently loaded for execution.
- Context cache: refreshable local material derived from durable sources.
- Review packet: redaction-safe evidence that shows why a context item was
  loaded or ignored.

### Architecture

- Inputs: ObsMem references, trace artifacts, identity/continuity records, and
  explicit operator context notes.
- Outputs: topology records, context packets, stale-context warnings, and
  review notes.
- Interfaces: versioned topology and working-set packets, validator, Runtime
  memory/continuity services, and v0.92 issue-wave records.
- Invariants: raw private state must not leak; generated summaries must name
  their provenance; stale context must be detectable.

### Data / Artifacts

- Memory Palace topology packet.
- Context working-set packet.
- Stale-context validation report.

## Execution Flow

1. Select allowed durable memory and trace references.
2. Build or update palace anchors and traversal hints.
3. Materialize a bounded working set for the current task.
4. Emit reviewable provenance and stale-context checks.

## Determinism and Constraints

- Retrieval and context-packet construction must be reproducible from declared
  inputs.
- Context packets must stay redaction-safe and bounded.
- Memory Palace must not become an unreviewed hidden memory channel.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| ObsMem | read | Durable memory references and evidence-ranked retrieval input. |
| Trace | read | Provenance for context packet construction. |
| Identity / Continuity | observe | Links long-running context to the active identity chain. |
| Authoring | write | Future issue cards and review packets should cite the topology route. |

## Validation

- Demo: run a bounded resume/context proof over real topology and working-set
  behavior.
- Deterministic / Replay: rebuild the same context packet from the same
  declared references.
- Schema / Artifact Validation: validate versioned topology and working-set
  packets.
- Tests: cover stale context, redaction, missing reference, provenance
  mismatch, and unauthorized private-state access.
- Review / Proof Surface: v0.92 review packets should cite this feature doc
  when Memory Palace scope is included or deferred.
- Exact Runtime v3 target: `cargo nextest run --manifest-path
  adl-runtime-kernel/Cargo.toml --test memory_palace --no-tests=fail
  --status-level all`.
- Native portability: the same target emits one repository-contained canonical
  semantic packet on macOS and Linux; the issue-local validator recomputes the
  producer, manifest, log, output, workflow, run, and exact-head bindings and
  requires byte-identical semantic output.

## Non-goals

- later distributed or unbounded Memory Palace expansion
- raw private-state exposure
- replacing ObsMem, trace, or memory-grounding contracts
- approving v0.92 activation before implementation proof exists

## v0.92 Consumption

`v0.92` consumes this document as WP-11's required first working Memory Palace
slice. Birthday work may reference Memory Palace only after that issue lands
real behavior and exact-revision proof.

## Acceptance Criteria

- The v0.92 feature index links this document.
- The v0.92 WBS can route Memory Palace work without reconstructing it from
  local notes.
- The boundary among ObsMem, memory grounding, working set, context cache, and
  Memory Palace is explicit.
- WP-11 lands the first working deterministic slice and its negative proof.

## Risks

- Risk: Memory Palace becomes a narrative metaphor instead of an artifact
  boundary. Mitigation: require topology, context packet, and validator
  surfaces in implementation issues.
- Risk: summaries hide provenance. Mitigation: every context packet should name
  its source references.

## Future Work

Later issues may extend the first slice across distributed polis boundaries or
larger context topologies after the v0.92 contract is proven.

## Notes

This document requires useful implementation and proof before completion.

# ADR 0074: Runtime Generation Source Ownership

## Status

Proposed documentation of the implemented DEC-01 topology. Tracked by issue #745.

## Context

Runtime v2 compatibility and Runtime v3 execution coexist in the repository. A cross-generation reference can be mistaken for authority transfer, and reorganizing files can accidentally change the supported compatibility boundary.

## Decision

Preserve exactly three source-owner roots: adl/src/runtime_v2 for Runtime v2, adl-runtime for the Runtime v3 guardian/outer runtime, and adl-runtime-kernel for the Runtime v3 kernel. Documentation and evidence are shared explanatory surfaces, not additional runtime authorities.

Classify reverse references with the DEC-01 topology vocabulary. Retain the explicit reasoning_runtime_bridge into adl_runtime::reasoning_runtime. Captured compatibility baselines must remain captured records rather than becoming live dependencies on another generation's source layout.

Migration and rollback under DEC-01 are topology/consumer dry-run contracts. They preserve each generation's owner and stop for unclassified consumers. They neither delete v2 nor change the selected runtime default. Runtime v4 remains excluded; a requirement for it triggers explicit replanning.

## Consequences

Source reorganization and compatibility work can be reviewed against a closed owner/disposition manifest. Shared documentation cannot introduce authority. A passing topology check does not by itself prove a live migration, process cutover, or rollback of running workloads.

## Alternatives Considered

Treat every reverse reference as forbidden: rejected because a declared compatibility bridge exists. Let both generations own shared source: rejected because ownership becomes ambiguous. Use DEC-01 to start v4: excluded by the milestone decision.

## Supersession Relationships

Refines coexistence and source ownership alongside [ADR 0054](../../adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md) and [ADR 0057](../../adr/0057-reversible-adl-v2-default-and-rollback.md). It does not replace their separately scoped production-selection decisions or conflate the ADL v2 default with Runtime v2 source ownership.

## Source Evidence

- [DEC-01 topology](../../runtime/runtime-v2-v3-authority-topology.md)
- [Machine-readable topology](../../milestones/v0.92.1/evidence/runtime-decoupling/runtime-authority-topology.json)
- [Topology validator](../../milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh)
- [DEC-01 execution record](../../../.csdlc/issues/513/cards/sor.md)
- [Milestone decisions](../../milestones/v0.92.1/DECISIONS_v0.92.1.md)

## Validation Notes

The retained execution record reports compatibility tests, migration/rollback dry runs, and negative topology probes. This packet inspected those records and did not repeat runtime validation. No live consumer migration is claimed.

## Approval Boundary

This draft records DEC-01's existing bounded topology. New source authority, a runtime-default change, or v4 requires its own explicit decision.

# ADR 0072: C-SDLC v3 Native Authority

## Status

Proposed documentation of an existing operational decision. Tracked by issue #745. This draft does not grant cutover approval.

## Context

ADR 0056 records the accepted v2 generation boundary for v0.91.8. The operator-reviewed V3-F/#505 transition was merged by PR #591 on 2026-09-07. Continuing to present the historical v2 selector as current authority would conflict with the post-cutover operating contract.

## Decision

Record native C-SDLC v3 as the operational lifecycle authority only while its canonical selector, authenticated receipt, terminal reconciliation, and required Git objects validate against canonical origin/main. Missing or stale proof suspends authority. It does not activate v2 or resurrect v1 wrappers.

Use typed native owners for issue preparation, binding, card operations, independent exact-revision review, publication, finish, and cleanup. Keep lifecycle state authoritative and rendered cards/projections derived and verifiable. Preserve generation/digest guards, explicit repository and worktree identity, and authenticated remote readback. Operational ownership follows bound Git topology rather than claims or leases.

The v2 generation is retained for explicitly authorized rollback or bounded transition remediation. Preserve historical pre-cutover records as evidence; do not rewrite them to imply that v3 was operational before V3-F. Native authority checks and issue-local review truth are separate requirements.

## Consequences

There is one operational generation and no implicit fallback after a tooling failure. Migration must preserve retained invariants, audit provenance, and recovery semantics. A retained remote intent with uncertain outcome needs authenticated reconciliation; changing the operation identity is not a duplicate-prevention strategy. Rollback requires an explicit operator decision.

## Alternatives Considered

Keep v2 and v3 interchangeable: rejected by the post-cutover contract because it creates competing authority. Fall back to raw transport after any error: rejected because transport success does not establish lifecycle truth. Treat the merged cutover PR alone as sufficient forever: rejected because current selector, receipt, reconciliation, and Git checks remain mandatory.

## Supersession Relationships

On documentation acceptance, this record succeeds [ADR 0056](../../adr/0056-c-sdlc-v2-sole-lifecycle-authority.md) only for operational generation selection. Its historical v1-sunset and typed-lifecycle principles remain evidence. This proposal does not modify ADR 0056's status.

## Source Evidence

- [Root operating contract](../../../AGENTS.md)
- [C-SDLC v3 contract](../../csdlc-v3/CONTRACT.md)
- [Canonical selector](../../../csdlc-v3/operator/authority-selector.json)
- [Native receipt](../../../csdlc-v3/operator/native-authority-receipt.json)
- [Authority implementation](../../../csdlc-v3/src/authority.rs)
- [Local lifecycle implementation](../../../csdlc-v3/src/commands/local/mod.rs)
- [Remote lifecycle implementation](../../../csdlc-v3/src/commands/remote/mod.rs)

## Validation Notes

This draft was compared with current source and contract text. It does not rerun cutover, migration, cloud, or lifecycle acceptance. The source-bound packet manifest identifies the inspected bytes. Issue #745 was created through explicitly authorized typed v2 recovery; defect #744 owns repair of the retained native-v3 intent.

## Approval Boundary

The underlying V3-F approval remains the existing #505/#591 authority. Acceptance of this ADR requires normal documentation review and publication; it cannot supply missing operational proof.

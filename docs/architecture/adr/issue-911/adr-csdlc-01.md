# ADR-CSDLC-01: One authoritative semantic issue record

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: SIM-04 (#870).
Participating owners: SIM-01 (#867), SIM-02 (#868), SIM-03 (#869), SIM-04 (#870), SIM-05 (#871), CSDLC-DECOMPOSE (#862), CSDLC-REMOTE (#907), CSDLC-MERGE (#849).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

What owns lifecycle truth when commands, cards, local transactions and remote receipts describe the same issue?

## Context

The simplification plan identifies independently interpreted state and presentation-coupled digests as sources of contradiction. Current native v3 authority already depends on authenticated selector/receipt proof. The proposal is a simplification within that generation, not a new authority cutover.

## Decision

Route operator intent through one authoritative semantic issue record and transaction owner. Commands derive repository, worktree and evidence context where the declared contract permits, while checking the actual identities and authority before effects. Rendered cards are derived projections of semantic state, not another writer of lifecycle facts. Formatting drift is distinguishable from an amendment that invalidates proof or review.

Diagnostics must remain observational: no hidden transaction recovery, registration, projection replacement or other mutation during status/validation. Supported corrections and interruptions have finite explicit recovery paths. Local state and remote effects reconcile through durable operation identity, preserving idempotence after uncertain responses; changing an operation key is not a way to assume an uncertain effect failed.

Preserve exact-head independent review, topology, digest/concurrency guards, terminal readback and cleanup denial. Machine payloads remain on stdout and human observability on stderr according to the existing contract. One semantic owner does not mean C-SDLC owns Runtime services or that removing commands proves simplification.

## Alternatives Considered

Keeping card Markdown and transaction state as interchangeable authorities preserves ambiguity. Asking operators to reconstruct every digest and context field repeats facts the command can verify. Hidden repair in diagnostics is convenient but violates the observed-state contract. One semantic owner with explicit recovery reduces contradictory interpretations without weakening guards.

## Consequences

Commands can explain one current state and legal next action; amendments invalidate the evidence they actually affect. Transition requires explicit conversion of old records and projections. Pure-model correctness is insufficient: public installed commands must exercise this owner in actual operational journeys.

## Reversibility

Develop against isolated copies and preserve the existing writer until the separately governed transition is admitted. After activation, state rollback follows ADR-CSDLC-02; do not make old and new writers interchangeable. Historical projections remain evidence rather than an alternate source of truth.

## Validation Notes

Use installed healthy and interruption journeys, not only model tests. Prove zero diagnostic mutations with byte/registration snapshots; finite amendment recovery; one successful commit for competing requests; no duplicate remote effects; and rejection of stale review, wrong topology and corrupt evidence. Preserve stdout/stderr separation and redaction.

## Supersession Relationships

Refines proposed ADR0072 and the current native v3 contract. ADR0056 retains historical v2 scope; this proposal neither reactivates v2 nor grants transition authority.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md](../../../milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/architecture/adr/0072-csdlc-v3-native-authority.md](../0072-csdlc-v3-native-authority.md)
- [docs/csdlc-v3/CURRENT_AUTHORITY.md](../../../csdlc-v3/CURRENT_AUTHORITY.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.

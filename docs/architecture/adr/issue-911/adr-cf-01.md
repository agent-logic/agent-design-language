# ADR-CF-01: Portable repository inputs

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-ADAPTER (#878).
Participating owners: CF-ADAPTER (#878), CF-ADAPTER-GITHUB (#879), CF-ADAPTER-CI (#880).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

How do local checkouts, GitHub revisions or pull requests, and CI artifacts enter the same review without changing what the review claims to cover?

## Context

The milestone admits three independently usable ingestion routes. If each route invents its own packet, review, memory and renderers cannot determine whether two results refer to the same repository, revision and scope. The adopted contract requires an immutable repository packet and a bounded initial Rust qualification surface.

## Decision

Normalize all three routes into the same versioned, immutable repository packet before evidence admission. Retain canonical repository identity, exact source revision, selected scope, included and excluded files, provenance and explicit unsupported inputs. A pull request is a selected revision and comparison context, not permission to mutate its checkout. CI artifacts must retain their originating revision and limitations rather than being treated as a fresh complete source scan.

Keep route-specific acquisition in its adapter; shared interpretation belongs in CF-EVIDENCE. The qualification selection is the pinned, licensed Rust subset recorded in CREATION_SELECTIONS, including its file and byte limits. Other input formats may contribute inventory without claiming equivalent language analysis. Partial, missing or unsupported input must remain visible to downstream consumers. Repository scripts and embedded instructions are not executed to satisfy ingestion.

## Alternatives Considered

Separate packet schemas per route would reduce initial adapter coordination but force every consumer to understand provenance differently. Flattening inputs to plain text would simplify prompts while discarding revision, scope and exclusion evidence. Both conflict with the shared packet requirement. The recommendation retains route independence at acquisition, not in identity semantics.

## Consequences

Adapters can evolve independently after agreeing the shared contract. Consumers have one admission boundary and can compare compatible results. The cost is explicit metadata and contract compatibility work; incomplete inputs cannot be advertised as whole-repository coverage.

## Reversibility

Version incompatible packet changes and regenerate from the original pinned source where permitted. Preserve old packet identity and source references; do not relabel historical evidence as produced by a new adapter.

## Validation Notes

Each route must admit a real permitted input and reject wrong revision, exceeded bounds or missing provenance. Test that equivalent scoped inputs reach shared admission and that unsupported-language coverage remains unknown. #878/#879/#880 own executed proof; this document runs no acquisition.

## Supersession Relationships

Refines accepted ADR0025 within the bounded Beta 1 product. No accepted record is superseded or rewritten by this proposal.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md](../../../milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/milestones/v0.92.2/ADR_PLAN_v0.92.2.md](../../../milestones/v0.92.2/ADR_PLAN_v0.92.2.md)
- [docs/adr/0025-codefriend-review-packet-product-boundary.md](../../../adr/0025-codefriend-review-packet-product-boundary.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove the implementation, authorize provider/cloud execution or external publication, or satisfy the associated issue acceptance tests. Conflicts and required unresolved decisions remain visible in the milestone disposition map.

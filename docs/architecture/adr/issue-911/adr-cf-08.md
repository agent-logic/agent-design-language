# ADR-CF-08: Approval and renderer parity

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-UX (#895).
Participating owners: CF-UX (#895), CF-RENDER-MD (#896), CF-RENDER-HTML (#897), CF-RENDER-PDF (#898).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

What exactly does a human approve, and how do different report formats preserve that approval?

## Context

The milestone separates approval from Markdown, HTML and PDF rendering. Its adopted publication contract binds run/finding sets, artifact manifests, rendering versions, claims and target. Accepted ADR0025 keeps publication responsibility with humans.

## Decision

CF-UX records approval for the exact selected run/finding set, artifact-manifest digests, rendering versions, claims/non-claims, approval scope and intended target. A change to scope, findings, renderer or target invalidates the applicable approval. Withheld or invalidated publication remains a visible state; a prior approval cannot be applied to a materially different candidate.

CF-RENDER-MD consumes approval, synthesis and both action plans. HTML and PDF follow the Markdown result for semantic parity. Formats may differ in layout, but must preserve finding identity, source evidence, severity rationale, limitations, disagreements and action-plan content. Rendering a file is not an external publish action. This record does not permit Medium upload, manuscript submission or repository mutation.

## Alternatives Considered

Approving a title or a mutable output directory leaves later changes outside the review. Allowing each renderer to summarize independently can change severity or erase limitations. Requiring byte-identical formats is impossible and unnecessary. Binding the artifact set while validating semantic parity preserves meaning and review scope.

## Consequences

Approval becomes auditable and safely invalidated by material changes. Renderers must carry stable identities and limitations rather than optimize for visual brevity. Corrections may require renewed approval; a successful renderer cannot override withholding.

## Reversibility

Withdraw approval without deleting the historical decision record. Re-render changed content as a new candidate and obtain approval for its exact scope. Preserve old manifest references so a reviewer can determine which version was approved and which was withheld.

## Validation Notes

Execute all three exporters with the same accepted input fixture and compare required semantic fields and omissions. Change findings, scope, renderer and target separately and prove approval invalidation. Verify no renderer exports private evidence or treats withheld publication as approved.

## Supersession Relationships

Refines accepted ADR0025. No existing accepted publication authority is replaced; the candidate makes its exact-artifact binding concrete.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md](../../../milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/adr/0025-codefriend-review-packet-product-boundary.md](../../../adr/0025-codefriend-review-packet-product-boundary.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.

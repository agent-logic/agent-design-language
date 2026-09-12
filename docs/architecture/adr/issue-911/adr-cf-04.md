# ADR-CF-04: Architecture cognition and explanation

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-COG (#882).
Participating owners: CF-COG (#882), CF-COG-IMPACT (#883), CF-COG-RATIONALE (#884), CF-COG-DRIFT (#886).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

How can architecture analysis explain structure, impact, rationale and drift without turning incomplete graphs into unsupported certainty?

## Context

Four atomic tasks separately own the structure reporter, change-impact reporter, rationale reporter and drift reporter. Their contracts require evidence-linked output and explicit unknowns, including missing rationale, unknown edges and incompatible baselines.

## Decision

Build explanations from the admitted repository graph and its declared coverage. CF-COG reports dependency, layering, coupling and connascence findings with traceable evidence. CF-COG-IMPACT traces a changed symbol or module across known boundaries and exposes unknown edges; an opaque score is not an acceptable substitute. CF-COG-RATIONALE relates independently deployable boundaries to available ADR evidence and reports missing or conflicting rationale explicitly.

CF-COG-DRIFT compares compatible baseline and current graphs through CF-MEMORY semantics. A partial graph cannot prove a problem resolved, and an incompatible baseline yields an explicit refusal or not-comparable outcome. Keep observed structure separate from inferred intent and risk. Unsupported language analysis remains unknown even when generic inventory can read the files. Do not invent a decision rationale from the architecture that happened to be implemented.

## Alternatives Considered

A single architecture score is easy to rank but hides missing edges and competing explanations. Generating rationale from code alone makes inference look like an accepted design decision. Four unrelated graphs would simplify local implementation while breaking cross-report consistency. A shared admitted graph with separate accountable reporters retains explainability.

## Consequences

Users can inspect why a finding or blast radius was reported and where it stops. Graph coverage and extraction errors become part of the report contract. The approach requires explicit edge provenance and compatibility checks, and may produce fewer confident claims than a heuristic summary.

## Reversibility

Revise an analysis or extraction rule under a recorded contract version and rerun on the pinned source. Preserve earlier reports and classify whether comparisons remain compatible. A new inference must not retroactively become the rationale of an old ADR.

## Validation Notes

Execute known-graph, known-change and structural-delta scenarios through the production reporters. Include false-positive samples, unknown edges, missing/conflicting rationale, opaque-score rejection, incompatible baseline and partial-coverage negatives. Source traceability is required in each explanation.

## Supersession Relationships

Refines accepted ADR0025 within the bounded Beta 1 product. No accepted record is superseded or rewritten by this proposal.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ADR_PLAN_v0.92.2.md](../../../milestones/v0.92.2/ADR_PLAN_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/adr/0025-codefriend-review-packet-product-boundary.md](../../../adr/0025-codefriend-review-packet-product-boundary.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove the implementation, authorize provider/cloud execution or external publication, or satisfy the associated issue acceptance tests. Conflicts and required unresolved decisions remain visible in the milestone disposition map.

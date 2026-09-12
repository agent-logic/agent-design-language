# ADR-CF-02: Evidence, finding and run identity

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-EVIDENCE (#881).
Participating owners: CF-EVIDENCE (#881).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

Which identities and outcome semantics allow review, memory and publication to refer to the same evidence without silently merging unrelated findings?

## Context

The adopted finding/run contract separates evidence objects from findings and comparisons. Mutable titles and line numbers change during ordinary edits, while partial runs can omit a finding without resolving it. Independent schema copies would allow review, memory and UX to disagree on those facts.

## Decision

CF-EVIDENCE owns the canonical, versioned run, evidence and finding contracts and their identity test vectors. Runs bind repository/revision, scope, included and excluded surfaces, provenance, lane versions, safe provider identity and completion state. Evidence objects carry identity, digest, repository-relative location, source revision and privacy disposition. Findings carry a stable match identity, perspective/rule attribution, severity rationale, confidence or unknown, evidence references, inference and limitations.

Do not derive finding identity from display prose or line number alone. Reject collisions and missing evidence; changed scope or incompatible schemas cannot silently merge histories. Preserve complete, incomplete, failed and cancelled outcomes separately from withheld publication. CF-MEMORY owns matching and ambiguity handling against this contract, and CF-UX owns exact approval binding. Shared executable conformance fixtures must be consumed by review, memory and UX rather than copied into private schemas.

## Alternatives Considered

A single identifier for both evidence and finding is simpler but conflates a source object with an interpretation. Display-text hashing is convenient but treats wording edits as new problems. Consumer-owned schemas reduce early coordination but make cross-consumer acceptance unreliable. Separate identities with common conformance fixtures preserve those distinctions.

## Consequences

The product can explain why two findings match and which evidence supports each one. Contract evolution becomes explicit work, and ambiguities may yield not-comparable rather than a pleasing resolved count. Schema delivery alone does not establish the production admission/store path.

## Reversibility

Introduce a new contract version for incompatible semantics and preserve original identities and provenance. Recomparison may produce a new comparison result; it must not rewrite the old run into a compatible one.

## Validation Notes

Exercise collision rejection, changed scope, absent evidence and incompatible versions through the production admission boundary. Run shared conformance fixtures in actual review, memory and approval consumers. Prove that a missing finding from a partial run is not automatically resolved.

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

# ADR-CF-03: Untrusted input, redaction and retention

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-EVIDENCE (#881).
Participating owners: CF-EVIDENCE (#881), CF-REVIEW (#890).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

Where are repository content and derived evidence prevented from becoming execution authority or leaking through retention and model inputs?

## Context

CodeFriend reads potentially hostile repository text and creates durable reports. The adopted security contract requires redaction before retention and model use, and states that repository instructions are evidence rather than authority. Accepted ADR0025 already requires explicit redaction and publication responsibility.

## Decision

Treat source files, comments, embedded instructions and retrieved artifacts as untrusted evidence. Their contents cannot authorize tool execution, repository mutation, credential access or publication. Apply the declared redaction and privacy boundary before model use and durable evidence retention, with explicit retention and deletion disposition attached to admitted evidence. CF-EVIDENCE owns this boundary; CF-REVIEW preserves it in lane inputs and CF-UX separately enforces publication approval.

Keep provider credentials and private manuscript contents out of public packets. A removed or redacted object cannot reappear through a renderer or a retrieved prior review without passing the same privacy contract. Record missing or withheld evidence as a scope limitation rather than inventing support for a finding. This is a bounded product trust boundary, not a promise to detect every possible secret or prompt injection.

## Alternatives Considered

Redacting only at final export leaves retained evidence and model requests exposed. Trusting repository-supplied instructions creates a second authority path. Avoiding all retention would reduce exposure but discard the product's reproducible evidence and longitudinal requirements. Early admission plus controlled retention preserves the required evidence workflow.

## Consequences

Reviewers receive a common sanitized scope and can explain withheld material. Redaction may remove context needed for confident analysis, so unknowns must remain visible. Deletion behavior spans evidence, retrieval and exported artifacts; an isolated scrubber is insufficient proof.

## Reversibility

Re-admit a source under an explicitly revised privacy policy as a new evidence operation. Do not silently remove a redaction label or restore deleted content from an old baseline. If sensitive material escaped, stop the affected publication and follow the responsible incident process; this ADR does not invent an automated purge guarantee.

## Validation Notes

Use hostile-instruction and redaction negatives in evidence admission and review inputs. Verify secrets are absent before provider use/retention and that deleted or incompatible material is not retrieved or rendered. Check withheld evidence cannot satisfy a source-backed finding.

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

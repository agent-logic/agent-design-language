# ADR-CF-06: Independent review, synthesis and action plans

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-REVIEW (#890).
Participating owners: CF-REVIEW (#890), CF-SYNTHESIS (#892), CF-REMEDIATE (#893), CF-TESTPLAN (#894).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

How can multiple review perspectives and downstream plans remain independent, attributable and advisory?

## Context

The adopted contract requires four perspectives to receive the same scoped redacted evidence and commit before seeing peer findings. The task split assigns lane execution, synthesis, remediation planning and test planning to separate consumers; accepted ADR0025 does not grant autonomous code authority.

## Decision

Run four isolated perspective roles against the same admitted evidence and their own instructions. A lane commits its result before receiving peer findings. Synthesis is the first consumer of all lane outputs and preserves source attribution, disagreements, severity rationale and scope limitations. Independence is an information-flow property, not a requirement to purchase four different model vendors. Failed or partial lanes do not satisfy a completed four-perspective review.

CF-SYNTHESIS owns combined findings; CF-REMEDIATE and CF-TESTPLAN each turn that synthesis into their separately usable plans. Preserve the evidence and unresolved disagreement behind an action recommendation. Plans propose remediation and tests but do not mutate source or silently accept findings. Publication remains governed by CF-UX and the exact-artifact approval record.

## Alternatives Considered

Letting later reviewers see earlier findings encourages agreement without independent evidence. Requiring a different vendor per lane confuses procurement with information isolation. A synthesis that discards disagreement yields a simpler report but hides decision uncertainty. Merging all planning into the lane runner removes the task-specific consumer boundaries.

## Consequences

Reports retain the origin of claims and make disagreement reviewable. A failed lane reduces completion rather than disappearing into a successful summary. Separate consumers add coordination and shared-contract obligations, but their results can be reviewed and tested independently.

## Reversibility

Rerun an affected lane or synthesis against pinned inputs under a new result identity; preserve the previous outputs and changed scope. Refresh downstream plans and applicable publication approval when their inputs change. Do not edit a committed lane to make historical independence appear stronger.

## Validation Notes

Inspect every lane input for absence of peer output. Exercise contradictory findings, a missing/failed lane and incomplete scope, then verify synthesis preserves them. Execute both action-plan consumers with usable outputs and no source mutation; test that partial lanes cannot count as complete review.

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

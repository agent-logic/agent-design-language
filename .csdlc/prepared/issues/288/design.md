# #288 Design: v0.92 final ADR serialization and review handoff

## Boundary

Issue #288 owns only the final shared ADR serialization for the WP-18C ADR
tail. It consumes terminal issue-local evidence from #283, #284, #285, #286,
and #287, then updates the shared ADR index, ADR plan, machine-readable review
evidence manifest, and an internal review handoff packet.

It must not accept any ADR, run provider credentials, execute Unity or cloud
proof, change implementation acceptance criteria, or close parent #207.

## Source evidence

- #283 / ADR 0065 terminal cache and `.csdlc/evidence/283/evidence-manifest.json`
  classify ADR 0065 as reconciled with replacement terminal authority and leave
  acceptance to #288/human review.
- #284 / ADR 0066 terminal cache and `.csdlc/evidence/284/evidence-manifest.json`
  retain terminal and partial Guardian evidence while explicitly preserving #142
  completion and ADR acceptance as residual gaps.
- #285 / ADR 0068 terminal cache and `.csdlc/evidence/285/evidence-manifest.json`
  retain WP-19 handoff evidence but keep WP-18 birthday proof non-terminal.
- #286 / ADR 0069 terminal cache and `.csdlc/evidence/286/adr0069-evidence-reconciliation.md`
  records issue #84 as open and blocks ADR 0069 promotion.
- #287 / ADR 0071 terminal cache and `.csdlc/evidence/287/evidence-manifest.json`
  records #341/WP-18B as open with no derived terminal cache and blocks ADR 0071
  promotion.

## Serialization policy

- ADR 0065 may move from Deferred to Proposed because #283 records exact
  replacement terminal authority and non-empty machine-readable validation
  evidence.
- ADR 0066 remains Deferred because #284 retains residual #142/operational
  completion gaps.
- ADR 0068 remains Deferred because #285 retains non-terminal WP-18 birthday
  proof.
- ADR 0069 remains Deferred because #286 records the WP-18A Unity/Runtime
  consumer gate as open.
- ADR 0071 remains Deferred because #287 records #341/WP-18B as open with no
  terminal cache.
- No ADR becomes Accepted in #288.

## Deliverables

- Update `docs/architecture/adr/V092_ADR_INDEX_143.md`.
- Update `docs/milestones/v0.92/ADR_PLAN_v0.92.md`.
- Update `docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json`
  only for the final ADR serialization/review handoff extension.
- Add a bounded internal review handoff packet under `docs/milestones/v0.92/review/`.
- Add issue-owned machine-readable evidence and a focused validator under
  `.csdlc/evidence/288/`.

## Validation

The focused validator must prove that:

- the shared ADR index and ADR plan agree on ADR 0065 Proposed and ADR
  0066/0068/0069/0071 Deferred;
- no touched ADR status is Accepted;
- the review evidence manifest contains exact #283-#287 terminal references and
  residual-gap classifications;
- the handoff packet names architecture, security, documentation, and evidence
  review lanes without claiming those lane findings are already human-approved;
- every terminal child cache exists and the recorded merge commits are
  ancestral to the current branch.


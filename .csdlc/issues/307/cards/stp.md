# Structured Task Prompt

Template: 1.0.0

Issue: 307

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later coordinate only the approved release-tail graph, child evidence reconciliation, exact sprint review, and umbrella closeout.

## Deliverables

- .csdlc/prepared/issues/307/design.md
- .csdlc/prepared/issues/307/diagram.mmd
- .csdlc/prepared/issues/307/validate_preparation_bundle.py
- .csdlc/prepared/issues/307/validate_child_sequence.py
- .csdlc/issues/307
- .csdlc/evidence/307
- docs/milestones/v0.92/review/sprint_307

## Acceptance

1. AC-1: #343 is terminal, canonical, ancestral, and clean before #308 executes.
2. AC-2: #309 remains active between terminal #308 and #310, and #310 consumes its reviewed post-deletion head and inventory.
3. AC-3: The child graph is exactly #308 through #319, acyclic, and assigns every included outcome to exactly one independently owned issue while allowing explicitly safe parallel lanes.
4. AC-4: Successor execution depends on the predecessor merge/readiness contract, not individual closeout bookkeeping; final #307 closeout reconciles terminal, canonical, ancestral, and cleanup truth for the complete child universe.
5. AC-5: Review findings are resolved or explicitly routed, and release documentation/claims agree with landed exact revisions and residual risks.
6. AC-6: WP-30's separately authorized tag/release ceremony is verified by live readback without duplicate or partial-state mutation.
7. AC-7: One exact-head sprint review passes, #307 closes truthfully, and v0.93 receives the accepted handoff without implicit activation.
8. AC-8: #268 is recorded as closed successfully and no longer blocks Sprint 6.
9. AC-9: #471 is recorded as a WP-27/#315 remediation subissue rather than an independent release-tail lane.

## Dependencies

- Terminal/canonical/ancestral/clean #343
- Merge/readiness completion for predecessor-gated child execution
- Terminal/canonical/cleanup reconciliation for final #307 closeout only

## Inputs

- agent-logic/agent-design-language#307
- agent-logic/agent-design-language#343
- agent-logic/agent-design-language#308
- agent-logic/agent-design-language#309
- agent-logic/agent-design-language#310
- agent-logic/agent-design-language#311
- agent-logic/agent-design-language#312
- agent-logic/agent-design-language#313
- agent-logic/agent-design-language#314
- agent-logic/agent-design-language#315
- agent-logic/agent-design-language#471
- agent-logic/agent-design-language#316
- agent-logic/agent-design-language#317
- agent-logic/agent-design-language#318
- agent-logic/agent-design-language#319

## Non Goals

- Child implementation, repair, review execution, or publication
- AWS, provider, tag, release, deployment, or v0.93 activation during preparation
- Treating #309 as satisfied, optional, or deferred
- Treating issue closeout as a successor dependency
- Treating #471 as a release-tail sibling outside WP-27 remediation

# Structured Task Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare, validate, review, approve, bind, commit, and push issue-local lifecycle artifacts only; do not implement any WP-14A deliverable.

## Deliverables

- six current native typed C-SDLC cards
- issue-specific preparation design and dependency diagram
- declarative complete predecessor gate and deterministic receipt/ancestry checker
- COTS/reuse and bounded budget decisions
- PVF-classified validation plan
- reviewed preparation-only protected-path claim

## Acceptance

1. AC-1: SIP, STP, SPP, VPP, SRP, and SOR render through current typed C-SDLC v2 native generation and validate
2. AC-2: Design and diagram enumerate scope, evidence flow, COTS/reuse, budget, non-claims, promotion boundary, and downstream release-tail order
3. AC-3: Every declared direct, child, nested, and routed predecessor is listed with merged, typed closed_out, receipt, and origin/main ancestry requirements
4. AC-4: The active claim protects only #5384 issue, request, and lock paths and cannot authorize product or shared-document edits
5. AC-5: Validation lanes are PVF-classified with proof role, determinism, resource profile, budgets, acceptance mapping, and required/deferred status encoded by proof_role and defer_reason
6. AC-6: A bounded preparation subagent reviews the complete diff and all actionable findings are fixed before typed design approval and bind
7. AC-7: The preparation branch is committed and pushed with no PR or publication and root main remains untouched

## Dependencies

- WP-13 #5346 and #5347
- acceptance gates #5358 and #5361
- WP-14 children #5352, #4758-#4763, #5007, #4739, #4741, #5332, and #5107
- WP-10A #5497 and children #5498-#5502
- Runtime v3 nested inputs #5591, #5592, #5589, #5590, and #5526
- independently owned acceptance inventory inputs #5540, #5541, #5558, and #5548
- all dependencies require merged state, typed closed_out projection, shared-Git receipt, and observed SHA ancestry on current origin/main before implementation promotion

## Inputs

- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
- docs/templates/prompts/current.json
- .csdlc issues and shared-Git csdlc-v2 closeout receipts
- live GitHub issue and PR truth through an approved connector

## Non Goals

- WP-14A implementation, acceptance execution, deployment, or handoff execution
- product, Runtime, C-SDLC implementation, test, workflow, or shared milestone-document changes
- predecessor repair, waiver, closeout, merge, or claim takeover
- PR creation, typed publication, merge, or issue closeout
- AWS, Runtime v2, raw gh, provider execution, or credential access
- identity, consciousness, birthday, production-provider, or v0.92 readiness claims

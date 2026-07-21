# Structured Intent Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare a complete, reviewed, fail-closed execution packet for WP-14A without beginning implementation or acceptance work.

## Required Outcome

All six current native typed cards, issue-specific design and diagram, COTS, budget, PVF, protected-path, and dependency gates are durable and reviewed; implementation remains impossible under the preparation claim.

## Scope

- typed C-SDLC v2 issue projection for #5384
- issue-local preparation requests, design, diagram, dependency manifest, and gate checker
- bounded exact preparation review and typed design approval
- preparation-only claim binding with no product paths

## Authority

- Issue #5384 and its routing comments define WP-14A scope
- Checked-in v0.91.8 WBS, issue wave, and platform acceptance feature define dependency topology
- Typed C-SDLC v2 projections and shared-Git receipts define lifecycle truth
- Current origin/main ancestry defines integration truth
- This operator instruction approves preparation and binding only, not implementation, publication, merge, deployment, or predecessor waiver

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations only
- Keep root main untouched and perform all writes in the current bound issue worktree represented by .
- Do not implement product, Runtime, C-SDLC, documentation, test, workflow, deployment, or handoff scope
- Do not create or publish a PR and do not advance publication or closeout
- Do not use AWS, Runtime v2, raw gh, credentials, or fake approvals
- Do not promote until every declared predecessor is merged, typed closed_out, receipted, and ancestral to refreshed origin/main

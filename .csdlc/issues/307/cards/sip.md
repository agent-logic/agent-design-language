# Structured Intent Prompt

Template: 1.0.0

Issue: 307

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate the v0.92 quality and release-tail child graph from terminal #343 through WP-30, with truthful dependency, review, merge, async closeout, release, and handoff evidence.

## Required Outcome

An operator-approved acyclic child sequence completes with merge/readiness truth for successor execution, terminal/canonical/cleanup reconciliation only for final #307 closeout, release claims bound to landed evidence, and one exact sprint review before #307 closes and hands off without implicitly activating v0.93.

## Scope

- Entry-gate reconciliation from terminal #343
- Operator-approved child graph from #308 through #319
- Per-child review, merge, ancestry, handoff, async terminal, and cleanup truth
- Sprint-level release-evidence consistency and final exact review
- Truthful #268 successful closure status
- #471 routing as a WP-27/#315 remediation subissue

## Authority

- #307 coordinates and reviews; every child retains its own implementation and lifecycle authority
- #309 remains active v0.92 WP-21 authority between #308 and #310
- No child, AWS, provider, tag, release, deployment, or v0.93 activation mutation
- GitHub closure or green checks alone never substitute for typed terminal authority at final closeout
- #268 is closed successfully and does not block Sprint 6 or milestone closeout
- #471 remains child remediation under #315/WP-27 and is not an independent release-tail lane

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and remain unbound until #343 is terminal
- Preserve each child issue's independent ownership and validation contract
- Do not bypass #309 or reinterpret #310's post-deletion baseline
- Require reviewed/green/merged predecessor truth before advancing dependent child work; require canonical caches and cleanup truth before final #307 closeout

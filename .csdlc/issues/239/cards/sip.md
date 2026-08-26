# Structured Intent Prompt

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make derived terminal validation accept governed publication metadata-only head progression without accepting substantive drift.

## Required Outcome

A repository-grounded reconciliation shared with existing metadata-only review policy plus a PR #238-shaped regression and successful cached #5835 validation after merge.

## Scope

- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/tests/gate_finish.rs
- .csdlc/evidence/239/

## Authority

- Canonical issue generation, digest, publication, PR, repository, and terminal identity remain mandatory
- Only the existing typed metadata-only path policy may reconcile publication revision to terminal head
- Sprint #5854 closeout waits for this issue to land and revalidate cached #5835

## Assumptions

- none

## Operator Constraints

- Do not rewrite merged #5835 cards or terminal cache by hand
- No optional CI jobs
- At most one large runner
- Do not merge before fresh independent exact-head approval

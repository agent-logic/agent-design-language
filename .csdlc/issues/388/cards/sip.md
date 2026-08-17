# Structured Intent Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide narrow implemented-phase typed repairs for lifecycle card truth fields blocking #114 publication.

## Required Outcome

After review-assignment recovery, an implemented issue can repair stale SPP summary, VPP validation summary/failure policy, and SOR follow-up replacement or empty-vector removal through CAS-guarded typed operations without weakening review/publication/finish gates.

## Scope

- csdlc-v2/src/cards.rs semantic operation definitions
- csdlc-v2/src/store.rs operation authorization, guards, mutations, and audit records
- focused csdlc-v2 regression tests for #114-like recovery
- .csdlc/prepared/issues/388/validate_preparation_bundle.py
- .csdlc/evidence/388

## Authority

- #388 owns only typed lifecycle-tool repair operations
- #114 remains the consumer and is not product-mutated by #388
- Repairs require exact issue, expected generation/digest, actor, reason, and cleared downstream truth
- Review, publication, terminal, branch, worktree, and source authority are not changed by these operations

## Assumptions

- none

## Operator Constraints

- Bind beneath /Volumes/FastWork/adl-worktrees before source edits
- Do not hand-edit #114 cards as part of #388
- Do not relax review/publication/finish guards
- No raw GitHub lifecycle writes
- Keep #114/#115/#116/#117 boundaries intact

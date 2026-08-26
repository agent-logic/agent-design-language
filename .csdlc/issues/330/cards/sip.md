# Structured Intent Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the production recovery-to-cleanup boundary exposed by #300's bridge-fed integration proof.

## Required Outcome

A real #297 recovery-derived cleanup can complete without poisoning retained recovery validation, and cleanup final-receipt races fail closed with byte-exact zero mutation.

## Scope

- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/projection_recovery_integration.rs
- focused csdlc-v2/tests regressions for #330
- .csdlc/issues/330
- .csdlc/evidence/330

## Authority

- #330 owns only the production invariant repair named by #300 RED evidence
- #300 remains frozen/unpublished until #330 is terminal and ancestral
- #297 is terminal dependency authority, not a surface for mutation
- Typed C-SDLC v2 remains sole lifecycle authority

## Assumptions

- none

## Operator Constraints

- Never edit tracked primary main for implementation
- Bind beneath /Volumes/FastWork/adl-worktrees before source edits
- Preserve root unrelated staging and #300 blocked-proof checkpoint
- Do not weaken #299 cleanup authority or zero-mutation acceptance criteria

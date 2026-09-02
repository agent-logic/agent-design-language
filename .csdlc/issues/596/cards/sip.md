# Structured Intent Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the concrete Sprint 5/6 cutover-review blockers while keeping C-SDLC v2 as live authority until #505.

## Required Outcome

The remediation PR has typed local lifecycle state, uses typed C-SDLC v2 operations only as the still-live lifecycle authority, preserves a zero-net csdlc-v2 source/test diff, captures v2 defects as v3 replacement blockers, and remains explicitly non-authoritative for v3 cutover.

## Scope

- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/full-replacement-denominator.json
- .csdlc/issues/596
- .csdlc/prepared/issues/596
- .csdlc/evidence/604/full-cycle-defects-tail.md

## Authority

- C-SDLC v2 remains live authority until explicit #505 cutover
- PR #597 may close #596 only
- #505 and #534 references are Part-Of only
- No merge, finish, cleanup, or v3 authority cutover is authorized by this issue
- Raw gh is not used for covered lifecycle writes

## Assumptions

- none

## Operator Constraints

- Do not work on primary main
- Do not use /private/tmp for generated task artifacts
- Use typed C-SDLC v2 operations for covered lifecycle state
- Capture defects found while testing real issues for later resolution

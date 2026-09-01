# Structured Intent Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the concrete Sprint 5/6 cutover-review blockers while keeping C-SDLC v2 as live authority until #505.

## Required Outcome

The remediation PR has typed local lifecycle state, uses typed GitHub operations, fixes reviewed v2/v3 defects with behavior-backed proof, and remains explicitly non-authoritative for v3 cutover.

## Scope

- csdlc-v2/src/github.rs
- csdlc-v2/src/bin/csdlc-github-pr.rs
- csdlc-v2/operator
- csdlc-v3
- .github/workflows/ci.yaml
- adl/tools
- docs/csdlc-v3
- .csdlc/prepared/issues/596
- .csdlc/evidence/sprints-5-6-cutover-fixes

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

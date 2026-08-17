# Structured Intent Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Unblock #114 by adding a bounded typed v2 repair route for stale implemented-phase card truth before publication.

## Required Outcome

Implemented, reviewed, and published tooling fix that lets #114 truthfully repair STP/SPP/SOR card fields without hand edits or weakened publication guards.

## Scope

- csdlc-v2 typed editor/store authorization for implemented-phase pre-publication card truth repair
- focused regression coverage for #114-shaped review-recovery sequence
- no direct #114 card hand edits

## Authority

- Typed v2 owner binaries remain lifecycle authority
- Review/publication/finish guards remain fail-closed
- No product Runtime or Observatory behavior changes

## Assumptions

- none

## Operator Constraints

- Preserve #114/#116/#117/#110 worktrees and staging
- Use FastWork bound worktree before source edits
- No raw GitHub lifecycle writes

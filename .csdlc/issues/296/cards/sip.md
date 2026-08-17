# Structured Intent Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide a typed, fail-closed authored-design refresh after implemented review recovery.

## Required Outcome

An implemented issue with current review-recovery provenance can atomically refresh repository-owned design and diagram bindings, invalidate stale approval, preserve lifecycle history and topology, and require a new canonical design review before review or publication continues.

## Scope

- csdlc-v2 typed semantic edit contract
- implemented review-recovery authorization
- authored design and diagram binding refresh
- focused lifecycle, CAS, path, atomicity, and history tests

## Authority

- Only implemented records with current recover_review provenance are eligible
- No direct card, record, or authored-artifact mutation by the operator
- No phase, topology, execution-evidence, or historical audit rewrite
- Issue #294 is blocked and is not mutated
- Issue #291 store work remains excluded

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries only
- Bind under /Volumes/FastWork/adl-worktrees
- Preserve root #292 staging and known locks
- Coordinate exclusive csdlc-v2 file ownership with #291 and #294
- Use canonical fresh-session UUID design and exact-head review evidence
- Publish ready with Closes #296 and stop before merge

# Structured Intent Prompt

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore strict Clippy on current main by removing the obsolete unreachable v1 tooling helper introduced by PR #320.

## Required Outcome

The affected ADL crate passes focused CLI tests and strict all-target Clippy while preserving the completed v1 tooling sunset and leaving #259 untouched.

## Scope

- adl/src/cli/mod.rs
- adl/tests/issue_327_removed_tooling.rs
- .csdlc/prepared/issues/327/validate_preparation_bundle.py
- .csdlc/prepared/issues/327/validate_changed_paths.py
- .csdlc/issues/327
- .csdlc/evidence/327

## Authority

- #327 owns only the dead helper correction and issue-local proof
- #259 owns Runtime serving eligibility and is not modified
- Independent C-SDLC v2 binaries remain sole lifecycle authority

## Assumptions

- none

## Operator Constraints

- Never patch #259 or tracked primary main
- Bind beneath /Volumes/FastWork/adl-worktrees before source edits
- Preserve all unrelated root staging
- No optional or paid CI

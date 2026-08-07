# Structured Intent Prompt

Template: 1.0.0

Issue: 5906

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Allow historical finish to reconcile multiple merged closing PRs only when GitHub proves one unique latest merge and the request pins it exactly.

## Required Outcome

Deterministic unique-latest merged PR precedence with fail-closed timestamp and identity checks, followed by terminal reconciliation of issues 5818 and 5861.

## Scope

- csdlc-v2/src/github.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate_finish.rs
- .csdlc/issues/5906
- .csdlc/prepared/issues/5906

## Authority

- Issue 5906 owns only historical merged-candidate precedence
- Routine review, publication, and finish gates remain unchanged
- Exact live GitHub identity remains terminal authority

## Assumptions

- none

## Operator Constraints

- Never edit tracked work on main
- Use typed C-SDLC v2 Rust binaries
- No AWS
- Run focused tests and strict Clippy
- Obtain exact-head subagent review before PR

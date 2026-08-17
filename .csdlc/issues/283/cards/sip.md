# Structured Intent Prompt

Template: 1.0.0

Issue: 283

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile terminal evidence for ADR 0065 without promoting or accepting the ADR.

## Required Outcome

An issue-local evidence packet states whether ADR 0065 has terminal, landed, exact-revision-bound ACIP schema catalog and governed projection proof, with residual gaps recorded for #207 and #288.

## Scope

- .csdlc/issues/283
- .csdlc/prepared/issues/283
- .csdlc/evidence/283

## Authority

- #283 consumes terminal evidence only; it does not own ACIP implementation or proof repair
- #5832 remains historical/superseded evidence unless current terminal authority explicitly restores it
- #209 / PR #215 may serve as replacement proof only if live merge, derived terminal cache, non-empty artifacts, and validation manifests agree
- #288 owns shared ADR index, plan, manifest, and review packet serialization
- No ADR may be moved to Accepted by #283

## Assumptions

- none

## Operator Constraints

- Do not edit shared ADR docs, ADR index, milestone plan, or product code
- Use typed C-SDLC v2 for lifecycle state
- Use raw GitHub only for read-only observation if typed GitHub state is insufficient
- Preserve root main cleanliness and bind execution to a FastWork worktree before implementation edits

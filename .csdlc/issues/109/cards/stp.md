# Structured Task Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Document and prove the existing SRP handoff to a fresh external review session.

## Deliverables

- csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
- docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md
- .csdlc/prepared/issues/109/validate-fresh-session-srp.sh

## Acceptance

1. AC-1: standard SRP remains sole review authority
2. AC-2: exact-head SRP goes to a fresh non-inheriting session
3. AC-3: reviewer is read-only and reports findings first with file/line evidence
4. AC-4: actionable findings are resolved and substantive changes trigger fresh review
5. AC-5: review depth matches docs, code, or authority-critical scope
6. AC-6: no new orchestration or lifecycle machinery
7. AC-7: no redundant broad validation

## Dependencies

- existing standard SRP
- existing csdlc-review record route

## Inputs

- csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
- docs/tooling

## Non Goals

- new review protocol types
- persistent reviewers
- model-specific routing
- CI redesign

# Structured Task Prompt

Template: 1.0.0

Issue: 369

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the typed bound/implemented false-design-review recovery route and focused regressions.

## Deliverables

- .csdlc/evidence/369
- .csdlc/issues/369
- .csdlc/prepared/issues/369
- .csdlc/prepared/issues/369/design.md
- .csdlc/prepared/issues/369/diagram.mmd
- .csdlc/prepared/issues/369/run_exact_focused_matrix.py
- .csdlc/prepared/issues/369/validate_exact_scope.py
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Acceptance

1. AC-1: Exact request fields include phase generation digest previous reviewer/revision false reviewer actor reason and disposition.
2. AC-2: Bound and implemented recovery preserve topology/audit, append correction, advance CAS, and set design review pending only.
3. AC-3: Wrong phase/CAS/reviewer/revision, empty values, repeated correction, or later review/publication/terminal authority fail closed.
4. AC-4: Existing initialized recovery and approval flows remain compatible; no generic state editing is introduced.
5. AC-5: Exact #275 false-review shape recovers and doctor/validation can require a genuinely fresh approval.
6. AC-6: Focused tests, strict Clippy, exact review, hosted CI, finish/cache/ancestry pass before #275 resumes.

## Dependencies

- Blocks #275
- Blocks #205 integration

## Inputs

- agent-logic/agent-design-language#369
- #275 bound gen17 false current design approval
- existing initialized decomposition design-review recovery

## Non Goals

- Any #275 product edit
- Replacement approval
- History rewrite
- Generic audit mutation

# Structured Task Prompt

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair the false-ready diagnosis without changing product scope or lifecycle authority.

## Deliverables

- csdlc-v2/src/doctor.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/prepared/issues/17/design.md
- .csdlc/prepared/issues/17/diagram.mmd

## Acceptance

1. Repository identity drift is diagnosed deterministically.
2. Owned paths that cannot route a declared new Rust module are diagnosed.
3. Missing validators are rejected unless explicitly declared and deferred with fail-closed semantics.
4. Validation requires a nonzero issue-specific denominator.
5. Focused regression fixtures reproduce issue 5795's false-ready shape.

## Dependencies

- WP-03 merge b5bcfdfc13a6f454a715cbb9aa64e24bce3b7ba6

## Inputs

- csdlc-v2/src/doctor.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/git.rs
- .csdlc/issues/5795

## Non Goals

- Implementing issue 5795
- Running broad or optional validation
- Changing publication or closeout behavior

# Structured Task Prompt

Template: 1.0.0

Issue: 75

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add one typed publication linkage mode and carry it through publication, observation, evidence, and finish.

## Deliverables

- csdlc-v2/src/publication.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate_finish.rs

## Acceptance

1. AC-1: closing mode preserves current exact closing-keyword behavior
2. AC-2: part_of accepts only an exact non-closing Part of reference
3. AC-3: split-authority part_of references are repository-qualified
4. AC-4: mixed, ambiguous, or mismatched linkage fails closed
5. AC-5: intent, remote observation, and publication evidence retain linkage_mode
6. AC-6: finish cannot close or terminalize an issue from part_of evidence
7. AC-7: omitted mode remains compatible as closing

## Dependencies

- Current C-SDLC v2 publication and finish contracts

## Inputs

- agent-logic/agent-design-language#75
- csdlc-v2/src/publication.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate_finish.rs

## Non Goals

- Change GitHub issue migration
- Permit unqualified split-authority references
- Change ordinary closing PR behavior
- Add a second publication binary

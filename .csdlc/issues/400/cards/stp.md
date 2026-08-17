# Structured Task Prompt

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

C-SDLC v2 tooling only: csdlc-edit/card/store behavior and focused tests for implemented-phase recovery truth repair.

## Deliverables

- typed implemented-phase recovery support for SPP step status truth
- typed implemented-phase recovery support for STP dependency truth
- csdlc-v2/tests/gate5.rs focused regression coverage
- focused positive and negative tests
- validation and exact review evidence

## Acceptance

1. AC1: A typed recovery request can repair implemented-phase SPP plan-step statuses including completed and in-progress truth when generation/digest evidence matches.
2. AC2: A typed recovery request can repair implemented-phase STP dependencies when review evidence identifies omitted dependencies.
3. AC3: The recovery path rejects stale CAS, unsupported phases, review/publication/terminal-incompatible states, duplicate or malformed steps/dependencies, and unrelated card fields.
4. AC4: Audit history records old/new SPP/STP truth without deleting prior review or implementation evidence.
5. AC5: Existing review, publication, and terminal authority guardrails remain intact and covered by focused tests.

## Dependencies

- #117 reproduction evidence
- #292 implemented card identity repair precedent
- #294 initialized design recovery precedent
- #296 authored-design refresh precedent

## Inputs

- .csdlc/prepared/issues/400/design.md
- .csdlc/prepared/issues/400/diagram.mmd
- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/card_identity.rs
- csdlc-v2/tests/gate5.rs

## Non Goals

- raw Markdown card editing
- weakening csdlc-review or csdlc-publish
- broad lifecycle reset
- #117 product-scope changes
- generic card rewrite machinery

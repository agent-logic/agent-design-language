# Structured Task Prompt

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement one guarded csdlc-v2 SIP Goal recovery operation and tests.

## Deliverables

- Semantic operation and card application support for recovered SIP Goal repair.
- Implemented-phase recovery authorization and audit/projection support.
- Focused csdlc-v2 regression tests.

## Acceptance

1. AC-1: A recovered implemented issue can repair SIP goal through a typed operation.
2. AC-2: The operation rejects stale generation/digest and unrecovered reviewed/published/terminal state.
3. AC-3: Existing implemented-phase SIP guards remain fail-closed for unrelated SIP mutations.
4. AC-4: Focused regression proves #286-style SIP goal recovery without permitting arbitrary implemented SIP edits.

## Dependencies

- #400/#404 implemented-phase recovery patterns are landed references.
- #286 supplies the motivating reproduction but is not mutated by #407.

## Inputs

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .git/csdlc-v2/requests/286-sip-goal-r5-attempt.json

## Non Goals

- No direct #286 card repair.
- No general implemented-phase SIP set_field support.
- No publication or review guard weakening.

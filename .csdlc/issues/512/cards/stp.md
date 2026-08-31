# Structured Task Prompt

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one implemented HTML Observatory redesign consuming authentic Runtime projections.

## Deliverables

- OBS-A contract implementation
- Authentic Runtime projection consumption
- Browser and accessibility proof
- Redaction and recovery proof
- Review-ready OBS-B evidence packet
- .csdlc/prepared/issues/512/validate-obs-b-browser.sh
- .csdlc/prepared/issues/512/validate-obs-b-accessibility.sh
- .csdlc/prepared/issues/512/validate-obs-b-redaction.sh
- .csdlc/prepared/issues/512/validate-obs-b-recovery.sh

## Acceptance

1. AC-1: OBS-A contracts are implemented
2. AC-2: Runtime projections are source-grounded
3. AC-3: Accessibility and recovery cases pass
4. AC-4: No mock substitutes for the required Runtime route
5. AC-5: One-command pre-cutover canary passes but execution remains blocked until #511 and #536 are terminal

## Dependencies

- #511 reviewed and terminal
- #536 Sprint 8 coordination terminal

## Inputs

- agent-logic/agent-design-language#512
- agent-logic/agent-design-language#511
- agent-logic/agent-design-language#536
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- adl/tools/validate_layer8_authority_observatory_ui.sh

## Non Goals

- TLS 1.2 implementation owned by #251
- Public exposure owned by #122
- Unity integration owned by independent backlog #84
- Mock Runtime substitution

# Structured Task Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One versioned Runtime orientation resource, admission-time injection path, per-agent provenance record/projection, and Observatory display.

## Deliverables

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/agent_roster.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/agent_orientation.test.mjs
- .csdlc/prepared/issues/708/validate-orientation-plan.sh

## Acceptance

1. AC-1: Every newly admitted agent receives the active valid orientation before its first model request
2. AC-2: The stored digest covers the exact delivered orientation bytes and the stored version identifies that delivery
3. AC-3: Existing agents retain their delivered provenance while valid hot reload affects only later admissions
4. AC-4: Invalid resource or reload input fails closed and preserves the last valid active package
5. AC-5: Runtime projections and Observatory agent details expose the stored per-agent version and digest
6. AC-6: Orientation remains explicitly non-authoritative and cannot enlarge agent permissions
7. AC-7: Focused tests prove ordering, full or deterministic-projection content, provenance, reload, failure preservation, and rendering

## Dependencies

- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md exists as the canonical source input
- Current Runtime admission and agent projection seams remain available

## Inputs

- agent-logic/agent-design-language#708
- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/agent_roster.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/agent_orientation.test.mjs
- .csdlc/prepared/issues/708/design.md
- .csdlc/prepared/issues/708/diagram.mmd
- .csdlc/prepared/issues/708/validate-orientation-plan.sh

## Non Goals

- Rewriting the welcome-package source document
- Retrofitting or mutating already-running agents' initial context
- Granting authority through orientation text
- Building a general prompt-template or content-management framework

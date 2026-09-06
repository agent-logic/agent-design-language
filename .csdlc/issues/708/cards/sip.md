# Structured Intent Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deliver the exact versioned Polis welcome package to every newly admitted agent before its first model turn and expose per-agent delivery provenance.

## Required Outcome

A first-class Runtime orientation resource is validated, admission-injected, retained per agent by version and digest, and visible through Runtime and Observatory projections.

## Scope

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
- .csdlc/issues/708
- .csdlc/evidence/708

## Authority

- Issue authority is agent-logic/agent-design-language#708
- The welcome package is orientation only and grants no authority
- Runtime policy, admission, Layer 8, and operator authority remain controlling
- The source welcome-package document is not modified by this issue

## Assumptions

- none

## Operator Constraints

- Schedule and label as v0.92.1 for ASAP execution
- Do not modify docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
- Do not perform tracked work on main
- Keep the implementation simple and bounded

# Structured Intent Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make model-backed resident agents initiate governed agent-to-agent contact through a first-class Runtime action path.

## Required Outcome

Beacon Axioma can initiate a governed turn to Ember Axioma through the live-style Observatory/Shepherd path without relying on natural-language roleplay or direct test-only primitive calls.

## Scope

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- demos/html-observatory/app.js
- adl/src/csm_shepherd_agent.rs
- adl/src/csm_resident_agents.rs
- .csdlc/prepared/issues/675/**
- .csdlc/issues/675/**

## Authority

- Issue authority is agent-logic/agent-design-language#675
- This is a corrective follow-up to #662/#668; #662 remains closed and is not silently rewritten
- Runtime policy, Layer8 signing, admission, replay, cancellation, and recipient/provider failure remain authoritative
- C-SDLC v2 remains the live lifecycle authority until explicit V3-F/#505 cutover

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not write to /private/tmp
- Do not use AWS, paid runners, or live cloud/provider calls without explicit authorization
- Do not hide A2A behind prompt wording alone

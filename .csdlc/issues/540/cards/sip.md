# Structured Intent Prompt

Template: 1.0.0

Issue: 540

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove Runtime v3 can admit an explicit additional browser CORS origin for http://localhost:8000 without using port 8000 as an ADL-owned listener.

## Required Outcome

Focused Runtime kernel tests prove configured allow and default deny for http://localhost:8000 while https://localhost:8765 remains canonical and ADL software does not bind port 8000.

## Scope

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/control.rs
- .csdlc/issues/540
- .csdlc/prepared/issues/540

## Authority

- Issue authority is agent-logic/agent-design-language#540.
- The issue may only affect Runtime v3 configured-origin CORS proof and any minimal support needed for additional_allowed_origins.
- Port 8000 may be used only as an HTTP Origin header value; ADL software must not bind or serve on port 8000.

## Assumptions

- none

## Operator Constraints

- Do not use /private/tmp.
- Do not write tracked implementation on main.
- No AWS, big runner, paid jobs, or long-running external resources.
- Keep Observatory fixed on https://localhost:8765 and Runtime API fixed on https://localhost:20997.

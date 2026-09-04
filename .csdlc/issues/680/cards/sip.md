# Structured Intent Prompt

Template: 1.0.0

Issue: 680

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add first-class ADL provider support for Moonshot/Kimi K3.

## Required Outcome

Operators can discover, configure, and select Moonshot/Kimi K3 through normal ADL provider setup/profile paths with deterministic offline tests.

## Scope

- Moonshot/Kimi provider profile and setup surfaces
- Provider selection support needed to treat kimi/moonshot as first-class
- Offline deterministic provider/profile/setup tests
- Documentation or help text required for operator setup

## Authority

- No live paid/provider call without separate explicit operator authorization
- Do not commit credentials or token material
- Preserve existing kimi:k2.5 and OpenRouter Kimi compatibility
- Use typed C-SDLC v2 lifecycle for bootstrap, bind, review, publication, and finish

## Assumptions

- none

## Operator Constraints

- Get issue #680 to PR
- Keep unrelated root staging, including .csdlc/prepared/issues/678/, preserved

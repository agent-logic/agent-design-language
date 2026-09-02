# Structured Intent Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Support Vertex Gemini global endpoints and model-family thinking controls through provider config without endpoint overrides.

## Required Outcome

A reviewed provider implementation that supports native global Gemini routing, trusted global Vertex hosts, thinking config rendering, focused tests, and live provider proof.

## Scope

- adl/src/provider/http_family.rs
- adl/src/provider/http_family/tests.rs
- .csdlc/evidence/608

## Authority

- #608 owns provider transport/config behavior only
- #592 owns later Polis integration
- Credentials remain outside the repository and are never printed or committed

## Assumptions

- none

## Operator Constraints

- Use the approved company GCP key by path only
- Do not expose credential contents or generated access tokens
- Do not add provider dependencies
- Do not redesign the entire provider system in this issue

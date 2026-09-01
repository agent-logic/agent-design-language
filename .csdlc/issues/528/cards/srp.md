# Structured Review Prompt

Template: 1.0.0

Issue: 528

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Design and implementation review for #528 Vertex AI Gemini provider transport only; exclude GCP infrastructure, #509 qualification, and live paid provider proof unless separately authorized.

## Prompts

- Does #528 preserve one shared Gemini semantic codec while adding a distinct Vertex AI transport?
- Does the design avoid credential disclosure and embedded API keys?
- Are project/location/model/endpoint/timeouts/cancellation boundaries explicit and testable?
- Do deterministic tests cover UTS tool names and arguments, streaming/non-streaming normalization, error classification, and redaction?
- Are live Vertex calls correctly separated as optional externally authorized proof?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

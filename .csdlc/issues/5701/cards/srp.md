# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/tests/guardian_soak.rs

## Prompts

- Do the OpenAPI contracts cover every real production Runtime v3 and Observatory endpoint without documenting unavailable behavior?
- Does route-parity validation prevent undocumented real routes and documented phantom routes?
- Are WSS authentication, inbound frames, outbound frames, close/error behavior, and correlation identifiers documented accurately?
- Are constants such as port and base URL represented through config/server variables rather than hard-coded runtime behavior?
- Does the implementation avoid #5344 protected paths unless there is explicit typed transfer or release?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:a05c942d460ee906766d5e715b02a702425f9547:9d71624962896802462ba0ddb4ec75c7ef538d3924dd17c8f6137ef4ece2eb5c")

Reviewer: Some("subagent:019fafbd-db23-7681-a979-4cc0bc68585d")

Result: pass

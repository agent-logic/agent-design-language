# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/tests/openapi_contract.rs
.csdlc/issues/551/cards/sor.md
.csdlc/issues/551/cards/sor.values.json

## Prompts

- Does validation reject an advertised Observatory origin that the combined CORS and WSS policy would not accept?
- Do REST and WSS default to the existing v2 contract, explicitly project v1 and v3, and reject unsupported schema selectors?
- Does one validated reload atomically replace every Polis parameter and Runtime presentation consumer without restart?
- Do invalid reloads preserve the complete last-known-good snapshot with bounded redacted diagnostics?
- Does HTML explicitly request v3 and render only feed-owned identity values?
- Is Unity absent from the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- No live browser, external network, or deployed TLS exercise was performed; the repair is test-only and preserves the previously reviewed production behavior.

## Review Result

Revision: Some("git-blake3:1e72566c2b37d5bc8f402802ada7f116db7a6c91:635df04606bc3a8e25f67204cf5574b19e23fb0abfd6fb5529c8be9e6526b895")

Reviewer: Some("fresh-session:019ff6cf-7d5a-7d4c-8a4e-1b91e7109af7")

Result: pass

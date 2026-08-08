# Structured Review Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/63
.csdlc/prepared/issues/63

## Prompts

- Can any implemented-phase SIP field other than declared_scope be changed through this route?
- Can correction occur while review, publication, or readiness truth remains attached?
- Does the audit retain exact previous and replacement arrays plus actor and reason?
- Do real editor and validator tests prove rendered-card and stale-input behavior?
- Does post-publication recovery remain owned by csdlc-review rather than this operation?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The focused regressions intentionally avoid broad workspace tests; GitHub CI remains the integration proof for the bounded C-SDLC v2 change.

## Review Result

Revision: Some("git-blake3:16d93fcc92a0a4a7d788be590462dd4ba15eb034:4c6a2ddd251388d42f2f67c98c43ed1a1302204cb5db68c50f8df34f4034e652")

Reviewer: Some("subagent:63-exact-head-review")

Result: pass

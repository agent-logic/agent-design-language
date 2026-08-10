# Structured Review Prompt

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.

## Prompts

- Is Axum the sole HTTP/WSS stack and is TLS construction genuinely shared?
- Does Quinn retain its maintained TLS 1.3 mTLS contract while reusing policy rather than code duplication?
- Can any Runtime product path still issue a certificate, trust a served leaf directly, mutate host trust, or disable verification?
- Are public server TLS, private mTLS, and application authority unambiguously separated?
- Do focused negative tests prove wrong SAN, invalid validity, unknown CA, incomplete chain, and wrong client identity fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:5308ba89b81c333fa6b1b08f5d5ac9cee461622c:ff95ddecce3579bc88f89dcbe7fd12ea19ce40a86516676760dc2403b7899ce8")

Reviewer: Some("Lorentz independent subagent 019fe945-9c9e-7153-8b4d-66680d6244e3")

Result: pass

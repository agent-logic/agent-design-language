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

Revision: Some("git-blake3:e58b28ce8a3f937fcece1ef9db1c68d20b9e4a33:6e26f6f6ef89ad5d44eaa3c1d127b6fc282623c42ee1a9e9f10ecd9e27c5b0ff")

Reviewer: Some("Lorentz independent subagent 019fe945-9c9e-7153-8b4d-66680d6244e3")

Result: pass

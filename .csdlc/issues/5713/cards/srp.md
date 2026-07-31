# Structured Review Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
adl-runtime/src/local_tls.rs
adl-runtime/tests/local_tls.rs
docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md
.csdlc/evidence/5713
.csdlc/issues/5713

## Prompts

- Does configuration fail closed unless TLS mode is explicitly managed_external or local_self_signed?
- Can local bootstrap ever mutate externally managed certificate/key paths?
- Does ordinary restart reuse the same certificate identity without regeneration?
- Is replacement explicit and atomic with last-valid preservation on failure?
- Do tests prove SANs, server-auth, rustls acceptance, restrictive permissions, and concurrent exclusion?

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

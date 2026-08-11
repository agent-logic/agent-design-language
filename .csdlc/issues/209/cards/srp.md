# Structured Review Prompt

Template: 1.0.0

Issue: 209

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/209
.csdlc/prepared/issues/209/design.md
.csdlc/prepared/issues/209/produce-native-receipt.rb
.csdlc/prepared/issues/209/validate-native-receipts.rb
.csdlc/evidence/209
.github/workflows/wp14-production-acip-repair.yml
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/acip.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime-kernel/tests/production_acip_wss.rs
adl-runtime-kernel/tests/support/runtime_init.rs
adl-runtime/src/runtime_api_auth.rs
docs/api/runtime-v3/v1/openapi.json
docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md

## Prompts

- Does the test exercise the real Guardian/kernel operation path rather than a service-local ACK?
- Can any principal/session sequence choice deny or authorize unrelated traffic?
- Does bounded pressure preserve replay and operation state while returning the declared typed error?
- Are OpenAPI required/nullability rules identical to runtime admission?
- Are exact native receipts bound to every authority-bearing source and test path?

## Findings

[
  {
    "id": "P1-rejected-advance-domain-capacity",
    "severity": "p1",
    "summary": "Fixed: excessive first advances validate against high-water zero before domain insertion, and repeated rejected domains cannot consume capacity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d05fe7f841f057888c763feaa3a41f9bfe2fa684:25521fe67893fdc5b9bc20d7a9a68103b87604219dfb8f4e6f0f70feae885f28",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact Linux and macOS native receipt execution remains deferred until reviewed publication; merge remains blocked until retained packet validation and fresh post-native review pass.

## Review Result

Revision: Some("git-blake3:d05fe7f841f057888c763feaa3a41f9bfe2fa684:25521fe67893fdc5b9bc20d7a9a68103b87604219dfb8f4e6f0f70feae885f28")

Reviewer: Some("/root/sprint4_5857/review_209_exact_head")

Result: pass

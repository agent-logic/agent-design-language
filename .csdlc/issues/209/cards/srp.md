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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

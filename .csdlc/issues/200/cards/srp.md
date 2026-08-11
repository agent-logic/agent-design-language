# Structured Review Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/mod.rs
adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/authority_reconciliation.rs
adl-runtime/src/distributed/authority_reconciliation/tests.rs
adl-runtime/src/distributed/polis_runtime.rs
adl-runtime/tests/distributed_authority_reconciliation.rs
.csdlc/prepared/issues/200/produce-proof-receipt.rb
.csdlc/prepared/issues/200/validate-proof-receipt.rb
.csdlc/evidence/200/v4
.csdlc/issues/200

## Prompts

- Can any caller, raw token, adapter object, closure, receipt, boolean, legacy command, or local history create a plan, result, or permit?
- Does every durable phase reconcile exact old/new checkpoint outcomes without duplicating steps or losing completed authority?
- Are read and mutation permits impossible until state, result, checkpoint, marker, and published view all agree?
- Does deterministic time evidence pass unchanged from #201 without replica-local branching?
- Do bounds, canonical encoding, exclusive locks, symlink and opened-handle race defenses prevent partial mutation and unbounded allocation?
- Does the exact proof bind all thirty-six cases and avoid claiming #203/#204 concrete behavior?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The repaired Linux fixture path must still pass the replacement hosted Runtime coverage job before merge; local exact and full Runtime denominators are green.

## Review Result

Revision: Some("git-blake3:0b4887b1aa1a7f5e76f672929c45935311d5d28e:0936c8d3a8f831a6dd9661cf1cbfdfc69920296db8f6ec04177187b3dd087693")

Reviewer: Some("codex:/root/review_200_portable_final")

Result: pass

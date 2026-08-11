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
.csdlc/prepared/issues/200
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

- Replacement hosted Runtime coverage remains a required pre-merge gate for the repaired Linux fixture path.

## Review Result

Revision: Some("git-blake3:1e7ab4b1b561554ec270c5ea53da0547e703ee15:794fa71e2b028ff0d9e41febbd35a2af5a0ad4dec17b32685d1e3b149496e083")

Reviewer: Some("codex:/root/review_200_publish_hygiene_final")

Result: pass

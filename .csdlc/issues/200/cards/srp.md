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
.csdlc/evidence/200/v5
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

- Replacement exact-head hosted CI remains a required pre-merge gate after rebasing onto issue #208.

## Review Result

Revision: Some("git-blake3:4d59de75c462e90c97768eaa4962d206bffd7b0e:e24edb736c5e67be872b25c06e7947bcbaa25c64dea3580df9d3ac3e870fb950")

Reviewer: Some("codex:/root/review_200_post_208_final")

Result: pass

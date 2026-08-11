# Structured Review Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/200
.csdlc/prepared/issues/200
.csdlc/evidence/200
adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/authority_reconciliation.rs
adl-runtime/src/distributed/authority_reconciliation/tests.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/src/distributed/polis_runtime.rs
adl-runtime/tests/distributed_authority_reconciliation.rs

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

- Hosted CI and live GitHub merge state remain pending until exact-head publication.

## Review Result

Revision: Some("git-blake3:19a0c1e5e8bfc81cdfc0e4404bdd39aa320a265c:efcc31c37bbff3b4811e8ed1912bd14ff30e1d6ef5a4bf33fbb7a02d6730c404")

Reviewer: Some("codex:/root/review_200_impl_19a0")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/199
.csdlc/prepared/issues/199
.csdlc/evidence/199/v19
adl-runtime/src/distributed/lease.rs
adl-runtime/src/distributed/membership_coordinator.rs
adl-runtime/src/distributed/membership_coordinator/tests.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/src/distributed/transport/governed/learner_transport.rs
adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
adl-runtime/src/distributed/transport/governed/polis_runtime.rs
adl-runtime/tests/distributed_membership_transition.rs

## Prompts

- Does every operation require coarse AuthorityOperationKind::Membership plus the exact sealed issue-local artifact, with wrong coarse kind and wrong discriminator denied separately?
- Are exact old and target stable maps plus target membership bound before any learner or Raft membership effect?
- Are coordinator phase completion and parity publication private behind GovernedMembershipRuntime, using only current factory receipts and concrete local projections?
- Are local membership and authority projections staged until durable checkpoint/publication and repaired idempotently on exact published retry?
- Is non-voting enrollment durably journaled before external activation and safely resumed without premature local visibility?
- Does removal activate and re-observe the exact pending-exclusion receipt before retain=false transition and final parity publication?
- Does restart recovery distinguish proven no-effect membership errors from joint/target applied configurations and avoid duplicate ambiguous effects?
- Does the bounded durable result cache return exact results for current and older retained operations without re-entering effects?
- Does the real four-node proof execute removal, separate enrollment, fresh-node rejoin promotion, catch-up, joint/final commitment, parity publication, and crash recovery?
- Does retained v19 proof bind twelve behavior-specific cases, nine production assertions including the exhaustive real-node crash matrix, protected-source drift, immutable evidence, strict Clippy, and current-main ancestry?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI and live GitHub merge state remain pending until publication of the exact reviewed implementation.

## Review Result

Revision: Some("git-blake3:fa52b97ec95b55ccefcd9fc2660f5823de9eb9d9:6f58a93e3f46f4d54927709e40ef0a2047f45e3103f88c5019c8df31471e8a86")

Reviewer: Some("/root/review_issue_199_v19_fresh")

Result: pass

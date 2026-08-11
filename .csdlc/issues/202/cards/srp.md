# Structured Review Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/src/distributed/transport.rs
adl-runtime/src/distributed/transport/core.rs
adl-runtime/src/distributed/transport/root.rs
adl-runtime/src/distributed/transport/governed/learner_transport.rs
adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
adl-runtime/src/distributed/transport/governed/polis_runtime.rs
adl-runtime/tests/distributed_authorized_learner_transport.rs
adl-runtime/tests/distributed_runtime_transport.rs
.csdlc/issues/202
.csdlc/prepared/issues/202
.csdlc/evidence/202

## Prompts

- Does SecureLearnerNetworkFactory provide a real learner-owned production path with its own durable transport instance, peer pins, TransportAuthorityOwner, ProductionLearnerAuthority, and live nonextractable boot custody rather than a voter factory or cfg(test) scalar/key substitute?
- Does the real four-node proof complete a two-factory signed handshake over Quinn, force snapshot catch-up, apply a later append in the fourth PolisRaft, retain voters exactly 1,2,3, and prove learner-owned expiry waits through an actual Raft effect plus response?
- Does pending exclusion govern production admission, session currentness, voter-side learner-route installation, learner-owned ingress, retained-route draining, and same-stable-id recovery under a different node and Guardian identity with a strictly higher committed index?
- Does removal activation bind exact identity, voter-cut digest, target-membership digest, and a live deadline while exact retries use the durable cached result before re-decoding an expired authorization and still reject mismatched caller bindings?
- Are target-membership digest and deadline carried into durable admission, durable exclusion, learner session binding, RPC authorization digest, and restart reconstruction without a public bypass?
- Do ordinary and learner requests retain one shared authority lease from under-fence revalidation through stream creation, actual OpenRaft effect, and response, with transitions draining denied routes before later governed STREAM frames?
- Do the unchanged distributed_transport and distributed_discovery targets plus distributed_runtime_transport compile, and do all 42 private runner tests, 13 public tests, and both strict Clippy lanes pass?
- Does v7 evidence preserve exactly 36 semantic cases, 42 private runner tests, 13 public tests, and 31 named subassertions while protecting every governed source, integration target, and proof script?
- Does the v7 producer reject every dirty or untracked path outside its own output with no named temporary-directory exceptions, and does the validator require the proof's required main revision to equal current refs/remotes/origin/main and be ancestral to the protected source?
- Is the exact review revision clean, immutable, based on origin/main 1567469e395f9a6ea6c2e736366a8008f5ee1e06, bounded to #202, and free of unresolved P0-P3 findings before publication?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:4b55f2e4a40ce03e1612aaf4da7c129962f3369e:ab1400a4186baadad114ad90a481162360ea7b9c2555264c5117b4d2518850c8")

Reviewer: Some("codex:/root/review_202_v7_production_final")

Result: pass

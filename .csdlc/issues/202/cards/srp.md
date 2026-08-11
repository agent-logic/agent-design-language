# Structured Review Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed
adl-runtime/tests/distributed_authorized_learner_transport.rs
adl-runtime/tests/distributed_transport.rs
adl-runtime/tests/distributed_discovery.rs
adl-runtime/tests/distributed_runtime_transport.rs
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh
.csdlc/prepared/issues/202
.csdlc/evidence/202/v9

## Prompts

- Does SecureLearnerNetworkFactory provide a real learner-owned production path with its own durable transport instance, peer pins, TransportAuthorityOwner, ProductionLearnerAuthority, and live nonextractable boot custody rather than a voter factory or cfg(test) scalar/key substitute?
- Does the real four-node proof complete a two-factory signed handshake over Quinn, force snapshot catch-up, apply a later append in the fourth PolisRaft, retain voters exactly 1,2,3, and prove learner-owned expiry waits through an actual Raft effect plus response?
- Does pending exclusion govern production admission, session currentness, voter-side learner-route installation, learner-owned ingress, retained-route draining, and same-stable-id recovery under a different node and Guardian identity with a strictly higher committed index?
- Does removal activation bind exact identity, voter-cut digest, target-membership digest, and a live deadline while exact retries use the durable cached result before re-decoding an expired authorization and still reject mismatched caller bindings?
- Are target-membership digest and deadline carried into durable admission, durable exclusion, learner session binding, RPC authorization digest, and restart reconstruction without a public bypass?
- Do ordinary and learner requests retain one shared authority lease from under-fence revalidation through stream creation, actual OpenRaft effect, and response, with transitions draining denied routes before later governed STREAM frames?
- Do the unchanged distributed_transport and distributed_discovery targets plus distributed_runtime_transport compile, and do all 42 private runner tests, 13 public tests, and both strict Clippy lanes pass?
- Does v9 evidence preserve exactly 36 semantic cases, 42 private runner tests, 13 public tests, and 31 named subassertions while protecting every governed source, integration target, and proof script?
- Does the v9 producer reject every dirty or untracked path outside its own output with no named temporary-directory exceptions, and does the validator require the proof's required main revision to equal current refs/remotes/origin/main and be ancestral to the protected source?
- Is the exact review revision clean, immutable, based on origin/main 1567469e395f9a6ea6c2e736366a8008f5ee1e06, bounded to #202, and free of unresolved P0-P3 findings before publication?
- Does the coverage-impact mapping classify executable transport subtree files through runtime_v3_distributed_transport while preserving the dependency-free shim contract, and does the focused policy suite pass?
- After replace_authority_cut intentionally drains the stale peer route, does the exact runtime regression reinstall the rotated peer as a fresh governed route and preserve the issue-191 retry marker?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub required checks must rerun on the republished exact head; optional jobs remain intentionally disabled.

## Review Result

Revision: Some("git-blake3:77f0e52e36fc0f4a12b223d31754b96071402eda:0f9147103fb6d12ac09cecf89c26008158781a4e7b727d8a8ded91a617a16e5b")

Reviewer: Some("codex:/root/review_202_v8_ci_final")

Result: pass

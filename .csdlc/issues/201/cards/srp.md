# Structured Review Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/201
.csdlc/prepared/issues/201
.csdlc/evidence/201/v7
adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/authority_protocol_contract_tests.rs
adl-runtime/src/distributed/identity.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/src/distributed/polis_runtime.rs
adl-runtime/src/distributed/transport.rs
adl-runtime/tests/distributed_authority_protocol.rs
adl/tools/check_coverage_impact.sh
adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Does snapshot-replicated current authority contain only stable polis, epoch, membership, configuration, and voter truth, with no restart-scoped current boot map?
- Does runtime-external trusted boot custody require exact canonical full-vector equality on every new Prepare, rejecting stale, duplicate, reordered, zero, non-JCS, missing, or extra voter cuts before any state mutation?
- Does each prepared operation freeze its complete historical canonical boot vector plus a digest bound to its stable authority, and do finalization, publication, restore, and snapshot install reverify that historical custody rather than the runtime's later current cut?
- Can a boot-rotated reopen immediately build and install a snapshot before any new Prepare, while the current cut succeeds and the stale cut rejects byte-for-byte without state mutation?
- Do the exact 86 semantic cases, truthful full 230-test runtime lane, strict production Clippy, real three-voter OpenRaft path, and squash/shallow-safe proof all bind the same final source revision?
- Does lifecycle truth supersede the stale a629080 v7 result and remain pre-review and unpublished until independent design approval, final evidence, and fresh exact-head implementation review?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains pending until the exact reviewed repair is pushed and PR #229 is updated; merge is not authorized by this review record.

## Review Result

Revision: Some("git-blake3:591d3931a3802fbaeb0123e7b49452944f5564b3:43d7ee352a5a8f32977f43d0cccd9cd310a9e0d9261e6ae672e2a2aeefca581e")

Reviewer: Some("codex:/root/review_201_coverage_truth_final")

Result: pass

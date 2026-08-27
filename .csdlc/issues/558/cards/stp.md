# Structured Task Prompt

Template: 1.0.0

Issue: 558

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Test/harness-only stabilization for `real_four_node_learner_replication`, limited to timeout, polling, leader readiness, and diagnostics needed under coverage instrumentation.

## Deliverables

- Stabilized governed learner replication test harness in `adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs`
- Focused local validation evidence under `.csdlc/evidence/558`
- Typed review/publication/finish records for #558

## Acceptance

1. AC-1: The fix is limited to behavior-preserving test/harness stabilization for the existing governed learner replication proof.
2. AC-2: Learner authorization, membership, append routing, transport policy, and Runtime product semantics remain unchanged.
3. AC-3: Focused local validation proves `real_four_node_learner_replication` passes after the stabilization.
4. AC-4: The PR closes #558 and explicitly records that #499/#514 were shared-gate consumers.
5. AC-5: Independent OpenAI Responses API exact-head review passes before publication/finish.
6. AC-6: Required hosted checks are green before typed C-SDLC finish merge.

## Dependencies

- #513 is merged/closed by root at merge 5bc84a0f.
- #499 and #514 remain separate downstream work and are not modified here.

## Inputs

- GitHub issue #558
- GitHub issues #499 and #514 as blocked downstream shared-gate consumers
- Observed failure: node 4 missing `authorized-learner-replicated`, last_applied 11, current_leader None, timeout after 66.25s under coverage instrumentation
- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs

## Non Goals

- No product semantic changes to learner transport or authorization.
- No Runtime v4 work.
- No #483, Sprint 2, or Sprint 3 edits.
- No broad refactor of distributed runtime tests.

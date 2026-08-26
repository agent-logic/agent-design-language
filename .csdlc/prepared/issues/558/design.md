# Issue #558 design: governed learner replication coverage stability

## Intent

Stabilize the existing `real_four_node_learner_replication` proof so it remains deterministic under coverage instrumentation while preserving governed learner transport semantics.

## Observed failure

The adl-runtime coverage profile runs 679 tests and currently has one known blocker:

- `distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication`

The reproduced failure waits 66.25s and reports that node 4 did not observe `authorized-learner-replicated`; node 4 remained at `last_applied = 11` with `current_leader = None`.

## Boundary

This issue may change only the test/harness stabilization surface needed for instrumentation-aware determinism. Acceptable changes include bounded timeout/polling adjustments, explicit leader stabilization before learner append, or better diagnostic waits in `adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs`.

This issue must not weaken learner authorization, membership, append routing, transport policy, or product Runtime semantics. It must not modify #499/#514 scope beyond unblocking their shared coverage gate and must not touch #483, Sprint 2, Sprint 3, or Runtime v4 work.

## Validation

The focused proof is the exact learner replication test in adl-runtime. If local coverage instrumentation is too expensive for fast iteration, retain the ordinary focused proof plus diagnostic evidence explaining the coverage-specific stabilization, and defer final coverage confirmation to hosted required checks.

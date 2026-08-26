# Design: #560 runtime_v2 workspace coverage timeout stabilization

## Problem

Hosted workspace coverage run `33017588921` on PR #514 head
`401a6b533bce34c2d1d3b580b36939a3392f3b78` failed only because three
`runtime_v2::tests::unified_runtime_kernel` tests reached the `ci-coverage`
120s nextest slow-timeout:

- `runtime_v2_unified_runtime_kernel_rejects_event_order_and_correlation_drift`
- `runtime_v2_unified_runtime_kernel_events_are_correlated`
- `runtime_v2_unified_runtime_kernel_rejects_summary_and_participant_drift`

The same run completed `1224` tests with `1221` passed. Normal CI,
runtime-hosted coverage, fmt/clippy, and runtime tests were green.

Follow-up hosted run `33021333783` on PR #561 proved the original three tests
passed under the bounded `240s` allowance, but four sibling
`runtime_v2::tests::unified_runtime_kernel::*` tests reached the unchanged
`120s` ceiling. The same run also exposed that
`adl_gws_context_mirror::tests::milestone_truth_reads_current_repo_story`
accepted current milestones through `v0.92` but not the active `v0.92.1`
planning band.

## Bounded solution

Prefer a coverage-profile-only nextest override for the exact fully-qualified
`runtime_v2::tests::unified_runtime_kernel::*` module prefix:

- keep `profile.ci-coverage` default timeout unchanged for the rest of the workspace;
- increase only the seven runtime_v2 unified-kernel module tests to a bounded
  instrumentation-aware slow-timeout;
- add `v0.92.1` to the context-mirror test's explicit accepted current-milestone
  set;
- do not change Runtime v2 implementation semantics, assertions, event
  correlation logic, summary drift logic, or participant drift logic.

## Validation

Focused proof should prove the selector denominator selects exactly seven
runtime_v2 unified-kernel tests, run those tests through `cargo llvm-cov nextest`
using the `ci-coverage` profile, and run the context-mirror milestone truth
test. Hosted proof remains the required `adl-coverage` check.

## Non-goals

- No #483, Sprint 2, or Sprint 3 edits.
- No Runtime v4 work.
- No semantic relaxation of runtime_v2 unified-runtime-kernel assertions.
- No broad nextest timeout increase unless exact-test override proves impossible.

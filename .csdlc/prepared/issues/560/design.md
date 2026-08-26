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

## Bounded solution

Prefer a coverage-profile-only nextest override for those exact test names:

- keep `profile.ci-coverage` default timeout unchanged for the rest of the workspace;
- increase only the three observed runtime_v2 unified-kernel tests to a bounded
  instrumentation-aware slow-timeout;
- do not change Runtime v2 implementation semantics, assertions, event
  correlation logic, summary drift logic, or participant drift logic.

## Validation

Focused proof should run the three affected tests through `cargo llvm-cov
nextest` using the `ci-coverage` profile or the closest repo-supported focused
coverage equivalent. Hosted proof remains the required `adl-coverage` check.

## Non-goals

- No #483, Sprint 2, or Sprint 3 edits.
- No Runtime v4 work.
- No semantic relaxation of runtime_v2 unified-runtime-kernel assertions.
- No broad nextest timeout increase unless exact-test override proves impossible.

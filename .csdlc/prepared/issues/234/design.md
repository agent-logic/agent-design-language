# Issue #234 Design: Systemic CI Cost Control

## Decision

Use one automatic pull-request entrypoint: `.github/workflows/ci.yaml`.
That workflow classifies the changed surface before any expensive job is
eligible to start. Required heavy validation continues to use the configured
16-core runner. Standalone native proofs, retained proofs, soaks, demos,
provider canaries, and release-only coverage remain available only through
explicit dispatch or an intentionally declared reusable-workflow call.

## Dispatch Contract

1. Only `ci.yaml` may subscribe directly to `pull_request`.
2. The path-policy job runs on a standard runner and emits the selected lanes,
   skipped lanes, reasons, and effective validation profile.
3. Required heavy jobs use `vars.ADL_HEAVY_RUNNER`, whose approved value is the
   16-core GitHub-hosted runner. This issue does not downgrade required work.
4. Optional workflows never acquire a runner for ordinary PRs.
5. CI concurrency is keyed by target repository/workflow plus source
   repository/branch and target base, so duplicate PR objects for the same
   effective surface share one fleet. A newer commit shares that group and
   `cancel-in-progress: true` cancels the older in-progress branch run without
   conflating distinct base surfaces.
6. Unknown changed paths select the conservative required baseline only; they
   do not fan out to every optional proof workflow.

## Required-Check Readiness Contract

`csdlc-github-pr` and `csdlc-finish` treat the request's declared required
check names as the complete check gate. GitHub `unstable` mergeability is
permitted only when every declared required check is present and successful
and any required review is approved. Canceled, unknown, or failed checks that
are not declared required do not block readiness. `blocked`, `behind`,
`dirty`, draft, `unknown`, missing or failed required checks, base drift, head
drift, and missing required review continue to fail closed.

## Coverage And Soak Contract

- PR coverage is selected by the same path policy and remains bounded to the
  affected crate or workspace surface.
- Full-workspace, nightly, release, and ratchet coverage stay explicit and do
  not run merely because a PR touched a shared manifest or library root.
- Long Runtime and Guardian soaks are explicit out-of-band proof lanes with
  bounded timeouts and diagnostic receipts. They are not hidden inside normal
  tests or PR coverage.
- Required focused coverage may use the 16-core runner; optional coverage may
  not start automatically.

## Publication Procedure

Publication must reuse or fail closed on an existing open PR for the same head
branch and base. It must not create parallel PR objects that cause duplicate
workflow events. A superseding revision uses the same source-surface
concurrency group, and `cancel-in-progress: true` cancels the older run.

## Proof

A deterministic repository contract scans every workflow and fails when:

- a workflow other than `ci.yaml` subscribes to `pull_request`;
- a required heavy job bypasses path-policy selection or the configured heavy
  runner;
- CI concurrency is not keyed by source repository, branch, and target base, or
  `cancel-in-progress: true` is absent;
- a representative focused Runtime/Observatory change selects unrelated
  native proofs, soaks, demos, providers, or full coverage;
- a standalone optional workflow loses explicit-dispatch availability.

No hosted optional workflow is run to validate this policy. Local structural
and routing contracts are the proving surface before one reviewed publication.

## Non-Goals

- Changing GitHub organization runner configuration or billing settings.
- Moving required heavy validation to standard runners.
- Weakening required checks or release proof.
- Changing Runtime, Guardian, Observatory, provider, or demo behavior.

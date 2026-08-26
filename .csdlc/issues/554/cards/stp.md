# Structured Task Prompt

Template: 1.0.0

Issue: 554

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Shared CI baseline repair only: stale v0.92 README invariant and Runtime-v2 coverage reliability.

## Deliverables

- Focused source/docs fix
- Focused validation evidence
- Reviewed #554 PR with Closes #554

## Acceptance

1. AC-1: The Memory Palace v0.92 docs invariant test passes without broad release-truth rewrites.
2. AC-2: Runtime-v2 unified-runtime-kernel tests no longer time out in the required coverage posture.
3. AC-3: The #554 PR is reviewed, required checks are green, and it merges before #549 is rerun.

## Dependencies

- Observed #549 run 32998653026 failure evidence

## Inputs

- .git/csdlc-v2/logs/issue-514-pr-549-job-98275292059.zip
- docs/milestones/v0.92/README.md
- adl/tests/memory_palace_tests.rs
- adl/src/runtime_v2

## Non Goals

- Do not alter #514 provider-profile functionality.
- Do not touch #483.
- Do not bypass required coverage.

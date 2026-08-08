# Structured Task Prompt

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Fix issue #13 only: explicit coverage-producer selection, aggregation, and focused regression proof.

## Deliverables

- Explicit Runtime, fast-workspace, and full-workspace producer selectors.
- Job-level producer guards and deterministic aggregate result checks.
- Focused selected/unselected routing tests.
- Retained CI timing evidence from the reviewable PR run.

## Acceptance

1. A Runtime-local change does not execute either full-workspace shard.
2. Producer routing occurs at job level before checkout and tool installation.
3. The required adl-coverage check succeeds for intentionally skipped workspace coverage.
4. Runtime coverage runs when selected.
5. Full workspace coverage runs when selected.
6. Focused contracts cover selected and unselected producer and aggregator combinations.
7. CI evidence shows unselected producers consume only GitHub skipped-job accounting.

## Dependencies

- Canonical CI workflow
- validation manager
- path policy
- GitHub job result semantics

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/13
- https://github.com/agent-logic/agent-design-language/actions/runs/31143017130
- .github/workflows/ci.yaml
- adl/tools/ci_path_policy.sh
- adl/config/validation_lane_selector.v0.91.6.json

## Non Goals

- Coverage threshold changes
- Weakening selected coverage
- Runtime TLS changes
- AWS or runner provisioning

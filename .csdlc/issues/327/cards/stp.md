# Structured Task Prompt

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One strict-Clippy baseline defect caused by the unreachable real_tooling helper; no broad CLI or lifecycle redesign.

## Deliverables

- One bounded source correction
- adl/tests/issue_327_removed_tooling.rs
- .csdlc/prepared/issues/327/validate_preparation_bundle.py
- .csdlc/prepared/issues/327/validate_changed_paths.py
- Focused CLI and strict-Clippy proof
- Fresh exact-head review
- Ready PR with required hosted checks

## Acceptance

1. AC-1: Strict Clippy for adl/Cargo.toml all targets passes under -D warnings.
2. AC-2: Focused CLI tests pass and removed v1 tooling commands remain fail closed.
3. AC-3: No #259 behavior, source, PR, issue, or lifecycle state changes.
4. AC-4: Fresh exact-head independent review reports no actionable findings.
5. AC-5: Required hosted checks pass before typed terminal finish.

## Dependencies

- Merged PR #320 on current origin/main
- Required strict-Clippy failure observed on PR #326

## Inputs

- agent-logic/agent-design-language#327
- adl/src/cli/mod.rs
- .github/workflows/ci.yaml

## Non Goals

- Restoring v1 tooling
- Broad CLI refactor
- Any #259 mutation
- Optional or paid CI
- Unrelated lifecycle cleanup

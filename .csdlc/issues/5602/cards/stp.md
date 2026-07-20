# Structured Task Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add supported no-report profile collection to partition commands and strengthen focused contracts only.

## Deliverables

- Partition commands with --no-report
- Focused command-shape and failure-semantics regression coverage
- Local validation using /Volumes/FastWork

## Acceptance

1. AC-1: Every partition still executes its full selected test set
2. AC-2: Partition commands collect profiles without rendering reports
3. AC-3: Explicit combined ADL and Runtime reports remain authoritative
4. AC-4: Existing thresholds and fail-closed behavior remain unchanged
5. AC-5: Focused contracts pass with build artifacts on /Volumes/FastWork

## Dependencies

- PR #5599 run 29773292101 job 88477335280 retained failure evidence

## Inputs

- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- GitHub run 29773292101 job 88477335280

## Non Goals

- Coverage threshold changes
- Test-scope reduction
- Runtime product changes
- AWS execution

# Structured Task Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add a fail-closed Linux process backend to CSMctl and validate it without mutating real host services.

## Deliverables

- Linux CSMctl backend
- deterministic launcher tests
- Linux runbook instructions
- review evidence

## Acceptance

1. AC-1: Darwin selects the existing launchd backend
2. AC-2: Linux start/status/restart/stop works with isolated fixtures
3. AC-3: unsupported platforms fail closed
4. AC-4: stale or foreign PID files cannot signal unrelated processes
5. AC-5: Linux operation is documented
6. AC-6: exact-head Gemini review has no unresolved actionable findings

## Dependencies

- #424

## Inputs

- CSMctl
- start_CSM.sh
- docs/tooling/START_CSM_RUNBOOK.md
- .csdlc/prepared/issues/426/design.md

## Non Goals

- Runtime redesign
- system-wide service installation
- AWS qualification execution
- issue #269

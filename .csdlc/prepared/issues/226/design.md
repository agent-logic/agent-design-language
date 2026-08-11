# Issue #226 Design: Focused Runtime Validation Routing

## Decision

Extend the existing validation-lane manifest so the two tracked HTML Observatory validation tools map to the existing HTML Observatory runtime surface lane and issue design diagrams map to the existing docs diff lane.

## Invariants

- Runtime source changes continue to select the focused Runtime kernel lane.
- Lifecycle metadata and design diagrams do not create an unmapped-surface escalation.
- Observatory validators select their existing bounded browser/static contract lane.
- Slow proof and authoritative full coverage remain opt-in only when their own declared selectors require them.
- Unknown source or tooling paths still fail closed as unmapped.

## Scope

- `adl/config/validation_lane_selector.v0.91.6.json`
- `adl/tools/test_select_validation_lanes.sh`
- `adl/tools/test_ci_path_policy.sh`

## Non-goals

- Running or changing soak tests.
- Diagnosing the Guardian soak response issue.
- Changing Runtime product behavior.
- Weakening unknown-path fail-closed behavior.

# Issue #226 Design: Focused Runtime Validation Routing

## Decision

Extend the validation-lane manifest so the two tracked HTML Observatory validation tools map to a direct shell/JavaScript syntax lane and issue design diagrams map to the existing docs diff lane.

## Invariants

- Runtime source changes continue to select the focused Runtime kernel lane.
- Lifecycle metadata and design diagrams do not create an unmapped-surface escalation.
- Observatory tooling selects a tiny direct syntax lane; product/demo changes continue to select the existing integrated Observatory contract lane independently.
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

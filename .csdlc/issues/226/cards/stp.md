# Structured Task Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Map the three observed unmapped paths to their existing proving lanes and add focused routing regressions; do not execute or redesign slow tests.

## Deliverables

- Manifest selectors for HTML Observatory validators and design diagrams
- Focused selector regression proving no unmapped escalation
- Focused path-policy regression proving no slow or authoritative coverage escalation

## Acceptance

1. AC-1: The exact #111/#113 changed-path set no longer reports unmapped_change_surface
2. AC-2: The two HTML Observatory validation tools map to a direct bounded shell and JavaScript syntax lane
3. AC-3: design/*.mmd maps to docs diff hygiene
4. AC-4: Focused PR routing does not select slow proof or authoritative full coverage
5. AC-5: An unknown source or tooling path still fails closed
6. AC-6: Exact-head independent review has no unresolved findings

## Dependencies

- Existing validation manager and CI path-policy contracts on current main

## Inputs

- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/validation_manager.py
- adl/tools/ci_path_policy.sh
- adl/tools/test_select_validation_lanes.sh
- adl/tools/test_ci_path_policy.sh

## Non Goals

- Running or changing soak tests
- Diagnosing the Guardian soak response failure
- Changing coverage thresholds
- Changing Runtime product code
- Suppressing unknown-path escalation

# Structured Intent Prompt

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent unselected workspace coverage producers from consuming hosted execution while preserving deterministic required coverage checks.

## Required Outcome

Runtime-local changes run Runtime coverage without workspace producers, full coverage still runs when selected, and the required aggregate succeeds only when producer results match explicit path-policy selectors.

## Scope

- .github/workflows/ci.yaml
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- .csdlc/prepared/issues/13
- .csdlc/issues/13

## Authority

- The canonical path-policy job remains the sole producer-selection authority.
- Coverage thresholds and coverage commands are unchanged.
- Unselected producers must be skipped before runner work begins.

## Assumptions

- none

## Operator Constraints

- Never write tracked issue changes on main.
- Use focused deterministic validation.
- Do not use AWS.
- PR body must include Closes #13.

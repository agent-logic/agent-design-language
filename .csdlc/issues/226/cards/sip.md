# Structured Intent Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Keep focused Runtime and Observatory changes on bounded proving lanes without launching unrelated slow, soak, or authoritative full-coverage work.

## Required Outcome

All declared Runtime, Observatory validator, lifecycle metadata, and design-diagram paths map to existing bounded validation lanes while genuinely unknown surfaces continue to fail closed.

## Scope

- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/test_select_validation_lanes.sh
- adl/tools/test_ci_path_policy.sh
- .csdlc/issues/226
- .csdlc/prepared/issues/226

## Authority

- Issue and code authority are agent-logic/agent-design-language#226
- This slice changes validation routing only and does not change Runtime product behavior
- Unknown paths retain fail-closed escalation authority

## Assumptions

- none

## Operator Constraints

- Do not run optional, hosted, soak, long-running, or out-of-band jobs
- Use only focused selector and path-policy contract tests
- Keep the repair isolated from issue #111 product code

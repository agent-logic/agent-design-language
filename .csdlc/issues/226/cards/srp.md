# Structured Review Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/config/validation_lane_selector.v0.91.6.json
adl/tools/test_select_validation_lanes.sh
adl/tools/test_ci_path_policy.sh
.csdlc/evidence/226
.csdlc/prepared/issues/226/design.md

## Prompts

- Are the new selectors narrowly bounded to existing proof ownership?
- Does any unknown path become silently covered?
- Can this change launch optional slow or coverage jobs?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Optional hosted, coverage, soak, slow, and long jobs were intentionally not run because this is a focused routing-only repair.

## Review Result

Revision: Some("git-blake3:da76e0d7c8b96cc0ad54a047d7290fcb8b0d640a:cda536cbbe6cde136bc4fe2c1b01bd8c32267cb4f12969a92f1ff90970885de0")

Reviewer: Some("subagent:019ff210-ff6d-76e0-af5b-bd6bd6cb162c")

Result: pass

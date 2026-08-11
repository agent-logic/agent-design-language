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

- Optional hosted, coverage, soak, slow, and long jobs were intentionally not run.

## Review Result

Revision: Some("git-blake3:4fdb11b6b7e8c6a03891399e26b52887c776fa8f:40cdaded16e6952f3e43fb251cb05e04d49a9ca23c7610381ff3759be2d3c21e")

Reviewer: Some("codex-exec:issue-226-independent-review-7")

Result: pass

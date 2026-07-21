# Structured Review Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh

## Prompts

- Do all six cards and the design remain preparation-only and avoid acceptance/deployment overclaim?
- Are #5540 and #5541 consumed only as closed evidence inputs?
- Are #5548 and #5558 retained as independently owned open blockers?
- Are protected paths strictly issue-local with no shared milestone-document ownership?
- Are future proof lanes exact-revision-bound, deterministic where claimed, and fail-closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted required checks remain the final integration proof for the workflow contract.

## Review Result

Revision: Some("git-blake3:2a6c3199264d0b4a5fcee98acfaf7b566b44c095:a88b44534399911371bbe28819bb4f2dd27fa3285bb658e70db23527587069d2")

Reviewer: Some("provider:deepseek:deepseek-chat")

Result: pass

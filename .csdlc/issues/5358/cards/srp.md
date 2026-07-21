# Structured Review Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_path_policy.sh

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

- The review is bounded to the already identified always-run tooling-contract prerequisite repair; repository-wide CI behavior remains governed by required hosted checks.

## Review Result

Revision: Some("git-blake3:b2d16654c6737e92155bc2abb3f05e0b87699e3a:a3193789149d8abd3ae1faccbd71981b085a2643d6f07c64c7323a42cb174f26")

Reviewer: Some("provider:deepseek:deepseek-chat")

Result: pass

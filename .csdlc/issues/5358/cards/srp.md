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

- The existing repository-wide cargo-llvm-cov version policy is unchanged; install-action remains commit-pinned, and this repair introduces no new action or download source.
- The provider-asserted reviewer consumed the exact scoped patch and supplied source evidence rather than direct repository filesystem access; local executable contract proof remains authoritative.

## Review Result

Revision: Some("git-blake3:b142aa99870ecad5c304635851d6b1f516dd463d:3543c041833fb150520be97e35c347e4ca62c85858e076773f0fc1dd1d3ae32c")

Reviewer: Some("provider:deepseek:deepseek-chat")

Result: pass

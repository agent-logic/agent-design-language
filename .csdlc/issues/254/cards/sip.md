# Structured Intent Prompt

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Eliminate repeated Azure-backed hosted workspace coverage compilation while preserving required adl-coverage semantics.

## Required Outcome

The PR full-coverage hosted path compiles the instrumented workspace at most once and the required aggregate check only verifies, merges, and enforces coverage from producer summaries.

## Scope

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/validate_ci_workflow_policy.rb

## Authority

- CI topology and contract-test behavior only.
- No runtime production behavior, optional validation dispatch, cloud provisioning, or main-worktree edits.
- GitHub lifecycle writes must use typed v2 routes; raw gh is not allowed.

## Assumptions

- none

## Operator Constraints

- do not use gh
- do not use /private/tmp
- use the issue FastWork worktree
- do not start optional/cloud/paid/native/slow/soak jobs
- do not write on main

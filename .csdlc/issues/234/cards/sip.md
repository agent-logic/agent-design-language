# Structured Intent Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Keep required validation on the 16-core runner while preventing optional, unrelated, duplicate, and soak jobs from automatically consuming runners or budget.

## Required Outcome

One automatic PR workflow classifies changes before runner allocation, required heavy lanes use the configured 16-core runner, optional proof stays explicit, duplicate heads coalesce, and deterministic contracts prevent regression.

## Scope

- .github/workflows
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- csdlc-v2 publication duplicate-PR guard surfaces if current behavior requires repair
- focused CI and coverage procedure documentation
- .csdlc/prepared/issues/234
- .csdlc/issues/234
- .csdlc/evidence/234

## Authority

- Issue #234 owns CI dispatch, routing, cost-control contracts, and publication duplicate-head prevention only
- Required heavy validation remains on vars.ADL_HEAVY_RUNNER with the approved 16-core value
- No GitHub organization runner, billing, or hosted-runner configuration changes are authorized
- No Runtime, Guardian, Observatory, provider, demo, or product behavior changes are authorized

## Assumptions

- none

## Operator Constraints

- Never write tracked implementation changes on main
- Use the bound FastWork issue worktree
- Do not use /private/tmp
- Do not run optional GitHub workflows while developing this policy
- Validate locally, obtain bounded review, and publish one reviewed revision
- Do not downgrade required heavy jobs to standard runners

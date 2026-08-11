# Structured Task Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Audit all CI and coverage workflows, centralize automatic PR dispatch, isolate optional and soak proof, coalesce duplicate heads, and add deterministic policy contracts without changing product behavior.

## Deliverables

- Complete automatic-trigger and runner-allocation inventory
- Single automatic PR entrypoint with head-SHA concurrency
- Manual-only standalone proof, soak, demo, provider, and release coverage workflows
- Deterministic workflow and path-routing regression contract
- Concise procedure defining required, optional, deferred, and canceled lanes
- Reviewed ready PR with no optional hosted validation cycle

## Acceptance

1. AC-1: Only ci.yaml subscribes directly to pull_request; every standalone proof workflow requires explicit dispatch or an intentional reusable-workflow call.
2. AC-2: Every required heavy job remains path-policy gated and uses vars.ADL_HEAVY_RUNNER with the 16-core approved default.
3. AC-3: Optional, unrelated, soak, demo, provider, retained-proof, nightly, and release-only coverage lanes skip before acquiring a runner on ordinary PRs.
4. AC-4: CI concurrency is keyed by target repository and workflow plus source repository, source branch, and target base, so duplicate PR objects for one effective surface share one fleet and a newer commit cancels the older branch run.
5. AC-5: Unknown paths fail closed to a conservative required baseline without selecting the optional fleet.
6. AC-6: Focused Runtime and Observatory changes select only their declared required checks and issue-owned coverage, with long soaks excluded.
7. AC-7: A deterministic repository contract scans all workflows and fails on unauthorized PR triggers, heavy-runner bypass, concurrency regression, or optional fanout.
8. AC-8: The procedure records machine-readable selected, skipped, deferred, and canceled reasons without starting skipped workflows.
9. AC-9: Local focused proof and bounded exact-head review pass before one publication push; no optional GitHub workflow is used as development feedback.

## Dependencies

- agent-logic/agent-design-language#234
- current ci.yaml path-policy outputs
- current configured ADL_HEAVY_RUNNER 16-core selector
- current standalone native proof and coverage workflows

## Inputs

- .github/workflows/*.yml
- .github/workflows/*.yaml
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- GitHub Actions run inventory for duplicated #223/#228 branch heads
- issue #234 body

## Non Goals

- Changing GitHub runner-group or billing configuration
- Using standard runners for required heavy validation
- Weakening required checks
- Changing product behavior
- Running broad or optional hosted validation during implementation

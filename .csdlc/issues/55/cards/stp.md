# Structured Task Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Change the heavyweight coverage aggregator runner expression and its focused regression contract only.

## Deliverables

- Heavyweight adl_coverage_hosted selected-runner route
- Focused regression assertion preventing ubuntu-latest fallback
- Workflow syntax and larger-runner contract proof
- Exact-head review and publication evidence

## Acceptance

1. AC-1: adl_coverage_hosted uses vars.ADL_HEAVY_RUNNER with adl-ubuntu-24.04-16core fallback
2. AC-2: Focused CI contracts fail if the heavyweight aggregator regresses to ubuntu-latest
3. AC-3: Existing producer routing, Spot opt-in, artifact provenance, Codecov boundaries, and stable adl-coverage semantics remain unchanged
4. AC-4: Workflow syntax and existing larger-runner preflight contracts pass
5. AC-5: Typed state validates and the exact clean implementation head receives independent review before publication

## Dependencies

- Existing ADL_HEAVY_RUNNER repository variable contract
- Current .github/workflows/ci.yaml coverage job topology
- Existing CI runtime and path-policy contract tests

## Inputs

- AGENTS.md
- .github/workflows/ci.yaml
- adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- csdlc-v2/tests/gate_runner_preflight.rs

## Non Goals

- Changing coverage thresholds or test selection
- Changing coverage producer routing
- Changing the stable adl-coverage status aggregator
- Changing artifact or Codecov semantics
- Adding or using AWS runners
- Modifying unrelated PR branches

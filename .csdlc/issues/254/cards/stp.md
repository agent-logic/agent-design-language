# Structured Task Prompt

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair the required hosted coverage topology so it stops recompiling the workspace in the aggregate job.

## Deliverables

- single full workspace hosted coverage producer artifact
- light aggregate coverage job on ubuntu-latest
- provenance and summary verification before merge/gates
- contract tests forbidding aggregate Rust coverage reruns
- ready PR closing #254

## Acceptance

1. AC-1: The full workspace hosted path compiles instrumented workspace coverage at most once per PR run.
2. AC-2: adl-coverage-hosted does not install Rust coverage tools, download profraw shards, or invoke run_authoritative_coverage_lane.sh.
3. AC-3: The aggregate check still fails closed when required runtime/workspace producer summaries or provenance are missing.
4. AC-4: Azure heavy runner selection remains limited to Rust-producing jobs; the aggregate job runs on ubuntu-latest.
5. AC-5: Focused CI contract tests and workflow policy validation pass locally.

## Dependencies

- existing adl-coverage workflow topology
- existing coverage producer provenance files
- existing merge_coverage_summaries.py and coverage-impact gate

## Inputs

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/validate_ci_workflow_policy.rb

## Non Goals

- changing #199 production code
- dispatching optional or paid validation
- changing coverage thresholds
- introducing new cloud infrastructure
- using raw gh or GitHub connector lifecycle writes

# Issue #55 Design: Route Heavy Coverage Aggregation to the 16-Core Runner

## Problem

The `adl-coverage-hosted` producer-aggregation job performs heavyweight Rust
coverage profile aggregation and coverage-impact enforcement on
`ubuntu-latest`. The workspace/runtime coverage producers already use the
repository's selected heavy-runner contract, leaving a slow serial tail on a
smaller runner.

## Decision

Change only the heavyweight `adl_coverage_hosted` job's `runs-on` expression to
the existing `${{ vars.ADL_HEAVY_RUNNER || 'adl-ubuntu-24.04-16core' }}`
contract. Extend the focused CI contract checks so this job cannot silently
regress to `ubuntu-latest`.

The stable `adl-coverage` result aggregator remains lightweight and unchanged.
Coverage producers, artifacts, Codecov publication, Spot opt-in behavior, and
coverage thresholds remain unchanged.

## Invariants

- `adl_coverage_hosted` uses the selected heavy runner.
- `adl_coverage` retains its lightweight stable-status role.
- Existing producer routing and artifact provenance remain unchanged.
- No AWS execution route is introduced.
- A focused contract test fails if the heavyweight aggregator returns to
  `ubuntu-latest`.

## Validation

1. Parse `.github/workflows/ci.yaml` as YAML.
2. Run focused CI runtime/path-policy contract tests.
3. Run the larger-runner preflight/contract proof.
4. Validate typed issue truth, then obtain exact-head review.

## Rollback

Revert the runner-expression and its tightly coupled regression assertion. No
data or state migration is involved.

# WP-02B Post-Migration Build Acceleration Experiment Design

## Decision Boundary

WP-02B owns one bounded selection and command-structure experiment for the
restricted GitHub-hosted 16-core Ubuntu runner chosen by the operator. The
standard runner remains rollback context and is not a new dispatch target. It starts only
after WP-02 migration verification, WP-02A CI reliability, organization-owner
budget approval, alerts, and selected-repository runner access are proven.

The tracked issue-local validator at
`.csdlc/prepared/issues/5853/validate-experiment.rb` is the evidence-shape
authority. The operator-local source plan is historical planning input, not a
portable runtime dependency.

## Experiment Contract

1. Freeze one exact commit, workflow revision, toolchain, lockfiles, commands,
   permissions, cache design, proof inputs, workloads, and required-check
   topology.
2. Retain one cold baseline, three cache-hit warm baselines, and three
   cache-hit test-only canaries on the selected 16-core runner.
3. Retain queue, setup, cache, compile/link, execution, artifact, total-time,
   critical-path, reliability, retry/cancellation, and cost data without
   dropping unexplained outliers.
4. Adopt only when all retained runs pass, test-only p95 is at most 120 seconds,
   median workload reduction is at least 35 percent, and required-check and
   validation semantics remain unchanged.
5. Route `adl-rust-tests` to the selected runner, remove the dispatch-only
   experiment harness, and use the implementation PR as the production canary.
6. Preserve `ubuntu-latest` as a one-line rollback without using it as a new
   experimental control.

## Security And Negative Boundary

- The runner group is selected-repository only with maximum concurrency one.
- Paid execution requires an owner-approved maximum cost and alerts.
- Untrusted fork code receives no privileged runner or secret access.
- Required-check names, branch protection, validation breadth, and proof
  semantics do not change.
- Missing gates, absent cache-hit evidence, incomplete declared samples,
  parity failure, cost breach, or failed cleanup invalidates adoption.
- AWS, self-hosting, 32-core runners, coverage topology, custom images, and
  ARM64 are separate decisions.

## Rollback

Rollback changes only the centralized runner selection back to
`ubuntu-latest`. It must not require product code, test expectation, lifecycle
record, required-check, or branch-protection changes.

## Completion Evidence

Completion requires retained runner and benchmark receipts for all seven
declared runs, `eligibility.json`, `frozen-manifest.json`, `decision.json`, and
`final-state.json`, all accepted by the tracked validator, plus workflow
contract checks, a green production canary, diff hygiene, and exact-head review.

`frozen-manifest.json` declares the bounded denominator and numeric adoption
thresholds. The validator recomputes p50, p95, reliability, workload reduction,
and cost from retained receipts; it rejects denominator drift, inconsistent
statistics, missing security gates, or adoption below either performance gate.
## Owned Paths

- `.github/workflows/ci.yaml`
- `adl/tools/test_ci_runtime_contracts.sh`
- `.csdlc/evidence/5853`
- `.csdlc/prepared/issues/5853/validate-experiment.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-ci-runner-experiment-v1",
    "paths": [
      ".github/workflows/ci.yaml",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "issues": [
      5801,
      5853
    ],
    "order": [
      5801,
      5853
    ]
  }
]
```

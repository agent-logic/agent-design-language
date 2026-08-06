# WP-02A CI Topology

## Authority Flow

1. `.github/workflows/ci.yaml` invokes `adl/tools/ci_path_policy.sh` with the
   pull-request base and head revisions.
2. The policy emits a normalized `change_class`, `pvf_lane`, and
   `release_gate_role` plus the existing lane booleans, coverage authority,
   validation profile, and fail-closed state.
3. The workflow fans selected proof into focused C-SDLC v2, ADL v2, Runtime
   v3, tooling-contract, Rust, demo, slow-proof, and coverage jobs.
4. Stable `adl-ci` and `adl-coverage` aggregators accept only `success` and
   `skipped`; cancelled, failed, or missing lane results fail the aggregator.
5. PR-fast coverage is non-authoritative. Full coverage uses isolated runtime
   and workspace producers, run-attempt-bound artifacts, and one hosted
   aggregation gate.

## Normalized Classes

| Change class | Minimum PVF role |
|---|---|
| `current_docs_review` | diff hygiene |
| `lifecycle_metadata` | typed metadata or diff-hygiene proof |
| `workflow_tooling` | focused contract proof |
| `ordinary_product_source` | focused Rust source proof |
| `runtime_critical_source` | Runtime v3 or stronger source proof |
| `unknown` | fail-closed authoritative proof |
| `mixed` | strongest selected constituent proof |

The normalized summary is additive. Existing required-check names, routing
booleans, coverage thresholds, and source-proof authority remain unchanged.

## Cancellation And Final State

Workflow concurrency uses one group per workflow and pull request or ref with
`cancel-in-progress: true`. Cancellation prevents stale work from consuming
resources; it is not success authority. Both stable aggregators use an explicit
allowlist of `success|skipped`, and focused contract tests parse that allowlist
and reject `cancelled`, `failure`, and an empty result.

## Invariants

- Unknown or unavailable diffs require authoritative proof.
- Mixed changes never select less proof than their strongest constituent.
- Required check names remain `adl-ci` and `adl-coverage`.
- Full coverage is aggregated once for one exact run/head lineage.
- Destination migration settings, AWS, runner size, secrets, and branch
  protection are outside WP-02A.
- The WP-02A stop condition treats destination CI state as stable when its
  disabled, pre-cutover configuration is inventoried and unchanged. It does
  not authorize enabling destination Actions before Sprint 1 completes.

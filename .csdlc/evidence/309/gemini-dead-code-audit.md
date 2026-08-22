# Gemini advisory audit — non-authoritative input

This is retained advisory model output, not a deletion manifest or lifecycle
authority. The deterministic Git census, accountable disposition manifest,
source-grounded historical deletion manifests, focused validation, and exact
rollback receipt govern #309.

Operator reconciliation:

- Accepted for characterization: export-only liveness is insufficient;
  `adl_skill_v1`, the speculative-decoding prototype, the retired UTS benchmark
  cluster, local-Gemma evaluator, and retired GWS demo implementations were
  independently source-checked and placed in Band B.
- Rejected: deleting `policy_authority`, `cognitive_transition_schema`, AWS
  validation, Runtime v2/#414, active provider/ACC/capability surfaces, current
  demos, or the supported Gödel CLI. Those recommendations conflict with
  current governance, issue, runtime, CLI, or milestone authority.
- Deferred: every other suggested candidate remains retained unless a later
  issue supplies its own complete consumer and replacement proof.

### Executive finding
The repository reduction program is progressing, with 90,042 lines already removed prior to the current baseline. The current 49-file audit reveals that the existing reachability census has false-negative liveness rules, incorrectly flagging core execution paths and active CLI commands as having low incoming edges. However, several isolated evaluation prototypes and benchmark modules are genuinely dead and can be safely deleted.

### Historical denominator reconciliation
The historical #309 trend denominator was 355,675 physical Rust lines. The pinned execution baseline (`e926e3bca0ab1981d77b4658d2feb4059bdf33a6`) contains 265,633 lines. The difference of 90,042 lines represents code that was already deleted or moved outside the measurement boundary prior to this baseline. This reduction likely occurred during earlier phases (e.g., Phase 1/2 no-owner and replacement-coupled deletions) that removed superseded v1 implementations, retired tests, or legacy tooling. These 90,042 lines count toward the historical reduction trend but are no longer present in the active baseline.

### DELETE_NOW table
| Candidate | Evidence | Cheapest Decisive Check |
| :--- | :--- | :--- |
| `adl/src/adl_skill_v1.rs` | Source explicitly states it is a "pre-v0.92 skill standard surface". Manifest shows only 1 incoming edge from `lib.rs`. No active runtime or CLI consumers. | `git grep adl_skill_v1` |
| `adl/src/cognitive_transition_schema.rs` | Source defines a schema for WP-02 (`wp02_cognitive_transition_manifest_valid_fixture`). Manifest shows only 1 incoming edge from `lib.rs`. | `git grep cognitive_transition_manifest` |
| `adl/src/policy_authority.rs` | Source defines a prototype for WP-11 (`wp11_policy_context_fixture`). Manifest shows only 1 incoming edge from `lib.rs`. | `git grep evaluate_policy_authority_v1` |
| `adl/src/uts_acc_multi_model_benchmark/evaluation.rs` | Part of a pure benchmark prototype. No active workflows or CLI commands consume this module. | `git grep uts_acc_multi_model_benchmark` |
| `adl/src/uts_acc_multi_model_benchmark/execution.rs` | Part of a pure benchmark prototype. No active workflows or CLI commands consume this module. | `git grep uts_acc_multi_model_benchmark` |
| `adl/src/uts_acc_multi_model_benchmark/parsing.rs` | Part of a pure benchmark prototype. No active workflows or CLI commands consume this module. | `git grep uts_acc_multi_model_benchmark` |
| `adl/src/uts_acc_multi_model_benchmark/runtime.rs` | Part of a pure benchmark prototype. No active workflows or CLI commands consume this module. | `git grep uts_acc_multi_model_benchmark` |
| `adl/src/uts_acc_multi_model_benchmark/task_fixtures.rs` | Part of a pure benchmark prototype. No active workflows or CLI commands consume this module. | `git grep uts_acc_multi_model_benchmark` |

### NEEDS_CHARACTERIZATION table
| Candidate | Evidence | Cheapest Decisive Check |
| :--- | :--- | :--- |
| `adl/src/cli/artifact_cmd.rs` | Provides `validate-control-path` subcommand. Manifest shows no workflow consumers, but it may be used interactively. | `git grep "adl artifact"` |
| `adl/src/cli/identity_cmd.rs` | Provides `adl identity` command. Manifest shows no workflow consumers, but it may be used interactively. | `git grep "adl identity"` |
| `adl/src/cli/tests/godel.rs` | Tests the `godel` CLI command. The command's active usage status is unclear. | `cargo test --test godel` |
| `adl/src/godel/affect_slice.rs` | Part of the Gödel experiment loop. May be a retired evaluation prototype, but has a CLI command. | `git grep "adl godel"` |
| `adl/src/godel/canonical_evidence.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/cross_workflow.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/evaluation.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/hypothesis.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/mutation.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/obsmem_index.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/prioritization.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/promotion.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/stage_loop.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/surface_status.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |
| `adl/src/godel/workflow_template.rs` | Part of the Gödel experiment loop. | `git grep "adl godel"` |

### RETAIN_ACTIVE highlights
- **Core Execution Engine**: `adl/src/execute/runner.rs`, `adl/src/execute/state/runtime_control.rs`, `adl/src/execute/state/steering.rs`, and `adl/src/execute/support.rs` are fundamental to the runtime execution scheduling and state management.
- **Core CLI Commands**: `adl/src/cli/run.rs` and `adl/src/cli/demo_cmd.rs` provide the primary `adl run` and `adl demo` entrypoints.
- **ObsMem Integration**: `adl/src/obsmem_demo.rs` is explicitly called by `adl/src/cli/run.rs` (line 228), proving it is active despite the census missing the edge. `adl/src/obsmem_contract/client.rs` and `error.rs` are core contracts.
- **Learning Export**: `adl/src/learning_export/bundle_v1.rs`, `dataset.rs`, and `trace_bundle_v2.rs` are active CLI features for exporting learning data.
- **Providers**: `adl/src/provider/deepgram.rs` is a supported speech provider with active tests.
- **Run Artifacts**: `adl/src/cli/run_artifacts/cognitive.rs`, `runtime.rs`, `summary.rs`, and `adl/src/cli/run_artifacts_types.rs` are actively used to generate run state artifacts.

### Additional candidate clusters
Beyond the 49 supplied candidates, the following clusters in the baseline manifest are highly likely to be dead or superseded:
1. **`adl/src/uts_acc_compiler.rs` and `adl/src/uts_acc_compiler/*`**: If the `uts_acc_multi_model_benchmark` is deleted, this compiler may have no remaining consumers.
2. **`adl/src/demo/stock_league/*`**: An old versioned demo implementation.
3. **`adl/src/demo/v086_review_surface.rs`**: A superseded versioned demo from v0.86.
4. **`adl/src/wp08_acip_sns_proof.rs`**: An old evaluation prototype.
5. **`adl/src/aws_remote_validation.rs` and `adl/src/bin/adl_aws_remote_validation.rs`**: AWS infrastructure code that violates the "no cloud by default" policy and should be removed unless explicitly retained by the operator.

### Corrected reachability algorithm
The current census relies on `pub mod` declarations and simple text matching, which creates false negatives (e.g., missing the call to `obsmem_demo::maybe_emit_obsmem_demo_artifacts` in `cli/run.rs` because it uses a multi-item import `use ::adl::{..., obsmem_demo, ...};`).

A deterministic, authority-rooted reachability algorithm suitable for a fail-closed validator must:
1. **Define Authoritative Roots**: Start from `main.rs`, `bin/*.rs`, exported library functions (`pub` in `lib.rs`), active test harnesses, and supported workflow scripts.
2. **AST-Based Resolution**: Use a Rust AST parser (e.g., `syn` or `rust-analyzer` internals) to extract all function calls, struct instantiations, and trait implementations.
3. **Precise Path Resolution**: Resolve paths to their defining modules by correctly handling `use` aliases, multi-item imports (`use a::{b, c}`), and glob imports (`use a::*`).
4. **Graph Traversal**: Traverse the graph of resolved semantic references to mark all reachable modules and items.
5. **Fail-Closed Deletion**: Any file not containing at least one semantically reachable item from an authoritative root is classified as dead.

### Reversible wave proposal
- **Wave 1 (Isolated Prototypes)**: Delete `adl_skill_v1.rs`, `cognitive_transition_schema.rs`, and `policy_authority.rs`. (Estimated: 3 files, ~2,500 lines).
- **Wave 2 (Benchmarks)**: Delete the `uts_acc_multi_model_benchmark` cluster. (Estimated: 5 files, ~2,000 lines).
- **Wave 3 (Characterization - Gödel)**: Characterize the `godel` experiment loop and its CLI commands. If confirmed superseded, delete the cluster. (Estimated: 12 files, ~7,000 lines).
- **Wave 4 (Characterization - Legacy Demos & Cloud)**: Characterize and delete old demos (`stock_league`, `v086_review_surface`) and AWS remote validation tools. (Estimated: 8 files, ~8,000 lines).

### Protected surfaces and stop conditions
The following surfaces must **not** be deleted:
- Active Runtime v2 sources and #414 resident-Shepherd dehydration/rehydration continuity behavior.
- Supported `adl` commands (`run`, `demo`, `csm`, `csmctl`, etc.), exit codes, and output formats.
- Security and state-authenticity negative tests.
- Deterministic compiler and canonicalization tests.
- Persistence, recovery, rollback, and corruption tests.
- The permission-safe process-status boundary.
- Any capability that has a real runtime, CLI, test-contract, artifact, workflow, documentation, or declared external consumer.

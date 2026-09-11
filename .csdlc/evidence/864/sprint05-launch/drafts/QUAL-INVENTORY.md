# [v0.92.2][Runtime qualification][QUAL-INVENTORY] Measure exact-revision Cargo-registered validation inventories

## One complete result

Run a reproducible inventory tool against immutable refactor baseline `a71d699d52831b32bb68ed9c7c7e837925949de4` and merge `e986de6d06aacd385de93dd033def77a718c1581`, producing actual Cargo-registered validation denominators with explicit execution status. This task delivers the executed measurement, not an inventory plan or a new source-size reduction claim.

## Owned implementation and proof paths

Own bounded extensions to `adl/tools/validation_inventory.py` and `adl/tools/test_validation_inventory.sh`, or a narrowly named revision-inventory companion under `adl/tools/` when the existing tool's source-attribute semantics are insufficient. Inspect `adl/Cargo.toml` and relevant component manifests at each Git revision; do not assume a root Cargo workspace. Preserve `docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py` and its retained output as historical source accounting. New exact-revision results belong to this issue's evidence directory, not old evidence paths.

## Executed acceptance

1. Resolve both full commits, capture tree identities and isolate read-only source snapshots/worktrees at those exact revisions. Run Cargo metadata/target discovery and test enumeration with declared package/features/platform/toolchain; retain commands, exit status and nonzero registered-target/case counts where available. Include disabled/ignored/generated/doc-test cases explicitly or disclose why they cannot be enumerated.
2. Distinguish source test declarations, Cargo-registered test targets, enumerated executable cases, cases actually run and outcomes. Historical declarations 50→54 do not establish an executed denominator or a reduction. A failed build/list command records unavailable counts and cannot close a required measurement as success.
3. Compare the same declared scope/features across both revisions. Emit additions/removals/moves, exclusions and uncertainty without treating a facade shrink or lexical relocation as behavior reduction. The actual report consumer QUAL-EVIDENCE must be able to verify revision and measurement provenance.
4. Negative fixtures reject wrong revision, mismatched manifest/features, duplicated or omitted target rows, source-count substitutions and lexical-move-as-reduction claims. Run the real tool on both revisions, not only mocked metadata.
5. Obtain independent exact-result review and retain registered inventories plus execution classifications. This issue need not run every enumerated test, but it must execute the inventory process and clearly mark tests not run; it cannot claim runtime qualification.

## Dependencies and preserved scope

Execution prerequisites: WP-01, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: both_exact_revisions_measured, cargo_registered_inventory, independent_review.
- PVF: both_exact_revisions_measured, cargo_registered_inventory, source_count_not_execution, lexical_relocation_not_reduction.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.

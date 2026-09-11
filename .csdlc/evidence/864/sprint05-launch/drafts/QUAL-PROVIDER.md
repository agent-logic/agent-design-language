# [v0.92.2][Runtime qualification][QUAL-PROVIDER] Execute provider loss, interruption and recovery qualification

## One complete result

Induce real provider subprocess loss, timeout and interruption through the production provider harness and demonstrate bounded recovery with a healthy configured provider. This task qualifies actual effects and recovery; caller-set failure flags and named reference traces are not execution proof.

## Owned implementation and proof paths

Own bounded qualification harness/test changes around `adl-runtime-kernel/tests/governed_operations.rs` and its modules, the actual provider adapters they invoke, and `adl-runtime/src/qualification/mod.rs` when used by the production proof path. Re-resolve the current registered test names before running: the historical `parity_c_live_governance::real_provider_process_loss_timeout_and_recovery_are_classified` name is a source-recovery starting point, not a verified current test. `adl-runtime-kernel/src/conversation_sessions_tests.rs::resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress` is current ingress regression context. Never count a zero-match exact filter as proof. Keep #852's dispatch event implementation separately owned.

## Executed acceptance

1. RT-PROVIDER/#855 must deliver its registered-provider lifecycle before this qualification. Select a scoped owned test subprocess/session through that production route, observe actual start, kill/loss, deadline timeout and cancellation/interruption, and retain process/request identity, timestamps, exit state, Runtime outcome and correlation.
2. Restore service through a healthy configured production provider and execute successful subsequent work. Prove bounded retry/recovery, no duplicate accepted work and no stale provider identity reused. A ready flag without a real healthy result is insufficient.
3. Confirm real failure triggers reached provider invocation. Negative fixtures reject supplied failure flags, static reference-trace metadata, absent execution logs, wrong process/request identity and timeout classifications without elapsed/deadline evidence.
4. Exercise loss, timeout and interruption separately; do not infer one from another. Preserve statuses, cancellation, truncation and redaction at adapter boundaries. Where WSS failure-event evidence is consumed, require the reviewed #852 output rather than reimplementing or claiming it here; the exact graph remains RT-PROVIDER as this task's execution prerequisite, with full cross-producer joining in QUAL-EVIDENCE.
5. Use only task-owned processes and the explicitly approved environment; never kill shared resident/provider services. Independently review exact source/run/profile and all scenario evidence. Local mock transport proof is labeled separately and cannot replace required production provider execution.

## Dependencies and preserved scope

Execution prerequisites: RT-PROVIDER, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: actual_process_loss, actual_timeout_interrupt, healthy_recovery, actual_execution_receipts, independent_review.
- PVF: actual_process_loss, actual_timeout_interrupt, healthy_recovery, caller_failure_flags_not_proof, reference_trace_not_execution, provider_failure_recovery.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.

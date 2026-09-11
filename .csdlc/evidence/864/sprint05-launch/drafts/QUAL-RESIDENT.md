# [v0.92.2][Runtime qualification][QUAL-RESIDENT] Execute six-resident workload and signed-restore qualification

## One complete result

Execute six distinct resident tick/ACC/UTS workloads through production paths, then production signed dehydration/restore and resumed work with exact population, lineage and observable-effect evidence. The result is a completed qualification run, not a packet authoring task or six repeated copies of one synthetic action.

## Owned implementation and proof paths

Reuse bounded production harnesses `adl/tools/run_issue268_six_resident_uts_cycle.py`, `adl/tools/run_issue268_continuity_uts_qualification.py`, `adl/tools/validate_issue268_six_resident_uts_plan.py`, the plan `adl/tools/issue268_six_resident_uts_plan.json`, and corresponding existing test scripts. Ground workload behavior in `adl/src/long_lived_agent.rs`, its modules/tests and the current signed continuity owner resolved by the harness. Shared Runtime source changes are limited to a demonstrated harness-blocking defect and must be routed rather than absorbing QUAL-RUNTIME/#852's event repair. Historical runner/cloud setup scripts are inputs, not permission to run them against live infrastructure.

## Executed acceptance

1. Resolve the reviewed #852 failure-event repair (QUAL-RUNTIME) and RT-PROVIDER/#855 first. Execute six actual distinct roles from the bounded plan: shepherd_controller, planner, tool_executor, runtime_observer, recovery_custodian and reviewer_escalation. Bind role/tool authority/model/configuration identities and exact producer revision; placeholder artifact/configuration hashes are rejected before execution.
2. Retain each resident's tick, ACC/UTS operation, request/correlation identity, generated/provider result where applicable and observable workload effect. Measure exact admitted population and prove six distinct workloads. Caller labels or six receipt rows alone do not demonstrate six executions.
3. Exercise the production signed dehydration boundary, validate the retained population/lineage, restore through production verification and execute resumed work. Completed cases must not replay; only the exact pending case resumes. Match pre/post identity, signature, workload effects and population counts.
4. Execute tamper, omission and substitution negatives on isolated copies: changed signature/payload, removed resident, swapped lineage/provider/config or stale snapshot must be rejected before restored work. Capture actual denial and absence of inappropriate effects.
5. Distinguish deterministic local mock-provider regression from live model qualification. Complete the approved qualification profile's real production provider work; a mock trace alone cannot close the operational claim. Pin model/artifact/configuration, environment and resource limits before calls.
6. Independently review exact candidate, run artifacts, full scenario denominator, signatures and effect evidence. Sanitize public references while preserving auditable authorized access to sensitive proof; privately retained historical Run72 detail is not silently substituted for this current run.

## Dependencies and preserved scope

Execution prerequisites: QUAL-RUNTIME, RT-PROVIDER, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, actual_execution_receipts, independent_review.
- PVF: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, tamper_rejected, omission_rejected, substitution_rejected, mock_vs_live_explicit, resident_execution.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.

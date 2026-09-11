# [v0.92.2][Runtime qualification][QUAL-EVIDENCE] Validate complete criterion-bound Runtime qualification evidence

## One complete result

Deliver and execute a fail-closed five-row evidence consumer that admits current criterion-specific producer/execution records only after exact criterion, provenance, scenario and independent-review checks. This is a working validator consuming real completed producer results, not a manually authored aggregate packet or a substitute for missing qualification.

## Owned implementation and proof paths

Select `adl/tools/validate_v0922_runtime_qualification.py` and a focused negative-test companion under `adl/tools/` as new paths unless an equivalent existing current consumer is found and explicitly selected before implementation. Inputs are the completed #852 failure-event proof, QUAL-RESIDENT, QUAL-PROVIDER and QUAL-INVENTORY outputs. Inspect retained `docs/milestones/v0.92.1/evidence/release/tail-01/required-lane-denominator.json`, source issue text preserved under `.csdlc/evidence/864/task-scope-revision/`, and the existing #851 branch/worktree only after ownership reconciliation. Do not rewrite historical packets or hand-edit any lifecycle cards.

## Executed acceptance

1. Admit exactly `RUST-01-ac-4`, `DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2` and `DRT-C-ac-3`. Bind each exact criterion text/digest to its canonical source revision, producer revision, execution profile/log/artifact digests, required scenario set and independently reviewed result. Preserve explicit many-to-one producer mappings without permitting cross-criterion evidence substitution.
2. Execute the consumer against actual completed QUAL-INVENTORY two-revision measurements, QUAL-RESIDENT population/workload/signed-restore proofs, QUAL-PROVIDER failure/recovery proofs and #852 dispatch/WSS/correlation/redaction proofs. Missing producer execution leaves the relevant row not-proven and blocks aggregate completion; authoring a mapping is insufficient.
3. Preserve original 19 findings separately from the five criterion rows. Preserve the five cloud-control gaps and two execution-proof gaps as distinct historical categories. Reconcile references for historical consumers #522 and #833 without reopening/closing them or asserting all their original findings newly proved by five rows.
4. Negative fixtures individually remove/alter criterion text/digest, source/producer revision, scenario, execution log, signature/provenance and independent review; substitute another criterion's evidence, self-authored approval, stale/partial evidence or synthetic metadata. Every invalid case must fail admission with actionable reasons, not merely fail JSON parsing.
5. Verify producer evidence and actual outcomes rather than trusting a success boolean. Independent review binds exact candidate and artifacts; absence or unresolved findings cannot become accepted. Report per-row pass/fail/not-proven plus complete/excluded/missing denominators and residual risks.
6. Expose this validator as the real consumer used by the current qualification evidence workflow, and retain a positive run on complete real inputs plus all negatives. Public sanitized manifests may reference protected artifacts under authorized verification, but inaccessible proof cannot be represented as independently verified. No release approval is conferred.

## Dependencies and preserved scope

Execution prerequisites: QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: five_criterion_mappings_complete, producer_revision_and_criterion_digest_bound, independent_review_bound, exact_criterion_binding, independent_review, criterion_text_digest_source_and_producer_revision_execution_logs, historical_19_findings_5_cloud_control_2_execution_gaps_preserved, all_scenarios_present, self_authored_and_missing_review_evidence_rejected.
- PVF: five_criterion_mappings_complete, producer_revision_and_criterion_digest_bound, independent_review_bound, synthetic_metadata_rejected, cross_criterion_substitution_rejected, stale_or_partial_evidence_rejected, evidence_integrity_negatives, criterion_text_digest_source_and_producer_revision_execution_logs, historical_19_findings_5_cloud_control_2_execution_gaps_preserved, all_scenarios_present, self_authored_and_missing_review_evidence_rejected.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.

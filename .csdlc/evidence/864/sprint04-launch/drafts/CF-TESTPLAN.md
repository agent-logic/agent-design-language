# [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings

## One complete result

The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source.

Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files.
2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review.
3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings.
4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`.

pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

# [v0.92.2][CF-SYNTHESIS] Synthesize completed review perspectives

## One complete result

The synthesis entrypoint consumes committed lane results and emits deduplicated findings with source attribution, severity rationale and explicit disagreement.

Dependencies: CF-REVIEW. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `review synthesize` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: review/synthesis.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_synthesis.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Consume complete committed CF-REVIEW results through the production reader and emit one usable synthesized result with stable finding identity, original perspective/rule/evidence references, severity rationale, confidence/unknown and scope limits. Execute on actual predecessor output as well as controlled calibration fixtures.
2. Deduplicate equivalent findings while preserving all contributing attribution. Distinct or contradictory claims remain visible with explicit disagreement/disposition; disagreement cannot be erased to force consensus. Execute false-merge, missed-duplicate and severity-calibration cases with recorded expectations.
3. Reject incomplete lane sets, wrong run/revision/scope/schema, identity collisions, missing citations and unsupported claims. An absent finding is not proof of resolution. Preserve no-source-mutation and no automatic issue/publication effects.
4. Produce an independently readable artifact through the installed entrypoint and make its contract usable by remediation/test planners. A deduplication helper or authored synthesis packet without real consumption is insufficient.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `duplicate_control`, `severity_rationale`, `disagreement_retained`, `disagreement_preserved`.

pvf: `duplicate_control`, `severity_rationale`, `disagreement_retained`, `incomplete_lane_set_rejected`, `unsupported_finding_rejected`, `synthesis_contract`, `severity_calibration`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `disagreement_erased`, `finding_without_evidence`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

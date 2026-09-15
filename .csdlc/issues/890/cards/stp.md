---
issue_card_schema: adl.issue.v1
wp: "CF-REVIEW"
slug: "v0922-four-perspective-review"
title: "[v0.92.2][CF-REVIEW] Execute isolated four-perspective repository review"
labels:
  - "track:roadmap"
issue_number: 890
generated_at: "2026-09-12T00:09:56.378553+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "implementation_and_executed_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/890"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Wait for accepted merged output of #881, #855. Re-observe upstream closure, merged implementation PR and required contract/proof acceptance before binding; refresh this plan against those exact revisions. #864 WP-01 has accepted planning delivery via merged PR #865; other listed upstream issues remain open at preparation snapshot. Preparation is allowed; implementation is blocked."
pr_start:
  enabled: true
  slug: "v0922-four-perspective-review"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:09:56.378553+00:00

# Structured Task Prompt

## Summary

The production review runner executes correctness, security, adversarial and constitutional lanes on one scoped packet and retains attributed lane results before synthesis.

Dependencies: CF-EVIDENCE, RT-PROVIDER. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Goal

The production review runner executes correctness, security, adversarial and constitutional lanes on one scoped packet and retains attributed lane results before synthesis.

Dependencies: CF-EVIDENCE, RT-PROVIDER. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Required Outcome

The production review runner executes correctness, security, adversarial and constitutional lanes on one scoped packet and retains attributed lane results before synthesis.

Dependencies: CF-EVIDENCE, RT-PROVIDER. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Deliverables

Implement `review run` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: review/runner.rs and review/lanes.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_review.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

The production review runner executes correctness, security, adversarial and constitutional lanes on one scoped packet and retains attributed lane results before synthesis.

Dependencies: CF-EVIDENCE, RT-PROVIDER. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Acceptance Criteria

1. Run correctness, security, adversarial and constitutional lanes through the registered production provider on one admitted scoped packet. Persist all four attributed results with exact run/revision/scope, lane contract, evidence references, severity rationale and confidence/unknown. A configured adapter or canned fixture output is not generated-review proof.
2. Every lane receives the same redacted evidence and its perspective instructions, and commits before receiving peer findings. Capture actual input manifests and reject peer output injected through direct input, retrieval or cached context. Synthesis alone later consumes peer results. Four perspectives do not require four paid vendors.
3. Failures, timeouts, cancellation and absent lanes remain explicit and never satisfy complete four-lane status. Hostile repository instructions cannot trigger source mutation, tool authority or publication. Reject unsupported findings/missing evidence and retain bounded sanitized errors.
4. Execute deterministic controlled-provider negatives plus a separately authorized nonzero actual registered OpenAI review run on the selected scoped source. Pin actual model/profile and candidate; no credential inspection or paid invocation is authorized by issue creation. Prove no inspected-source changes and no raw credential retention.

## Repo Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][CF-REVIEW] Execute isolated four-perspective repository review

## One complete result

The production review runner executes correctness, security, adversarial and constitutional lanes on one scoped packet and retains attributed lane results before synthesis.

Dependencies: CF-EVIDENCE, RT-PROVIDER. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `review run` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: review/runner.rs and review/lanes.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_review.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Run correctness, security, adversarial and constitutional lanes through the registered production provider on one admitted scoped packet. Persist all four attributed results with exact run/revision/scope, lane contract, evidence references, severity rationale and confidence/unknown. A configured adapter or canned fixture output is not generated-review proof.
2. Every lane receives the same redacted evidence and its perspective instructions, and commits before receiving peer findings. Capture actual input manifests and reject peer output injected through direct input, retrieval or cached context. Synthesis alone later consumes peer results. Four perspectives do not require four paid vendors.
3. Failures, timeouts, cancellation and absent lanes remain explicit and never satisfy complete four-lane status. Hostile repository instructions cannot trigger source mutation, tool authority or publication. Reject unsupported findings/missing evidence and retain bounded sanitized errors.
4. Execute deterministic controlled-provider negatives plus a separately authorized nonzero actual registered OpenAI review run on the selected scoped source. Pin actual model/profile and candidate; no credential inspection or paid invocation is authorized by issue creation. Prove no inspected-source changes and no raw credential retention.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `four_lanes_execute`, `attributed_evidence_findings`, `provider_route_consumed`, `isolated_pre_synthesis_inputs`, `perspectives_preserved`, `evidence_citations`, `no_silent_mutation`.

pvf: `four_lanes_execute`, `attributed_evidence_findings`, `provider_route_consumed`, `peer_output_in_input_rejected`, `partial_lane_not_complete`, `no_source_mutation`, `review_fixtures`, `isolated_lane_inputs`.

stop_conditions: `finding_without_evidence`, `hidden_perspective`, `source_mutation`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `peer_results_exposed_before_lane_commit`, `partial_lane_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate.
Execution prerequisite: #881 (CF-EVIDENCE); accepted output is required before dependent execution.
Execution prerequisite: #855 (RT-PROVIDER); accepted output is required before dependent execution.

Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance.

## Dependencies

Wait for accepted merged output of #881, #855. Re-observe upstream closure, merged implementation PR and required contract/proof acceptance before binding; refresh this plan against those exact revisions. #864 WP-01 has accepted planning delivery via merged PR #865; other listed upstream issues remain open at preparation snapshot. Preparation is allowed; implementation is blocked.

## Target Files / Surfaces

Implement `review run` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: review/runner.rs and review/lanes.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_review.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Validation Plan

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused command after selected test is authored: cargo test --manifest-path adl/Cargo.toml --test codefriend_review. Also run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. This proposed test target is not present proof. Record installed production-consumer execution and nonzero exact scenario denominators; controlled transport cannot replace required actual provider output. Runtime PVF covers controlled semantics; required actual generated-review proof is a distinct provider lane with separately scoped authorization and pinned model/profile. Required macOS/Linux CI and manual accessibility observations are distinct from local deterministic results; unresolved required proof blocks acceptance.

## Demo Expectations

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused command after selected test is authored: cargo test --manifest-path adl/Cargo.toml --test codefriend_review. Also run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. This proposed test target is not present proof. Record installed production-consumer execution and nonzero exact scenario denominators; controlled transport cannot replace required actual provider output. Runtime PVF covers controlled semantics; required actual generated-review proof is a distinct provider lane with separately scoped authorization and pinned model/profile. Required macOS/Linux CI and manual accessibility observations are distinct from local deterministic results; unresolved required proof blocks acceptance.

## Non-goals

acceptance: `four_lanes_execute`, `attributed_evidence_findings`, `provider_route_consumed`, `isolated_pre_synthesis_inputs`, `perspectives_preserved`, `evidence_citations`, `no_silent_mutation`.

pvf: `four_lanes_execute`, `attributed_evidence_findings`, `provider_route_consumed`, `peer_output_in_input_rejected`, `partial_lane_not_complete`, `no_source_mutation`, `review_fixtures`, `isolated_lane_inputs`.

stop_conditions: `finding_without_evidence`, `hidden_perspective`, `source_mutation`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `peer_results_exposed_before_lane_commit`, `partial_lane_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Issue-Graph Notes

Wait for accepted merged output of #881, #855. Re-observe upstream closure, merged implementation PR and required contract/proof acceptance before binding; refresh this plan against those exact revisions. #864 WP-01 has accepted planning delivery via merged PR #865; other listed upstream issues remain open at preparation snapshot. Preparation is allowed; implementation is blocked.

## Notes

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

Wait for accepted merged output of #881, #855. Re-observe upstream closure, merged implementation PR and required contract/proof acceptance before binding; refresh this plan against those exact revisions. #864 WP-01 has accepted planning delivery via merged PR #865; other listed upstream issues remain open at preparation snapshot. Preparation is allowed; implementation is blocked.

Preparation only: no implementation, acceptance proof, implementation review, publication or integration claimed. Branch/worktree names are proposed, not bound. Shared adl/src/cli/codefriend_cmd.rs, cli/mod.rs, cli/usage.rs, lib.rs and codefriend module registrations require explicit per-issue ownership and serialized integration edits with Sprint2 and sibling Sprint4 workers. Keep fixture subdirectories issue-specific; rebase and rerun installed dispatch regressions after shared-path changes. No new umbrella; management belongs to #926.

## Tooling Notes

Native v3 issue/edit/validate preparation in resolved Git metadata. Bind through native v3 only after dependency and ownership recheck. Create an issue-bound session goal before implementation. Never hand-edit generated cards or weaken stale guards.

---
issue_card_schema: adl.issue.v1
wp: "CF-UX"
slug: "v0922-publication-approval"
title: "[v0.92.2][CF-UX] Enforce exact-artifact publication approval"
labels:
  - "track:roadmap"
issue_number: 895
generated_at: "2026-09-12T00:10:15.986006+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/895"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "#891 (CF-SHELL) and #881 (CF-EVIDENCE) are closed/accepted as of live read on 2026-09-16; before bind, recheck active CodeFriend path ownership and do not absorb #893/#894."
pr_start:
  enabled: true
  slug: "v0922-publication-approval"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:10:15.986006+00:00

# Structured Task Prompt

## Summary

The publication-control entrypoint binds approval to exact run, findings, renderer and target identities and supports withholding and invalidation before any publication.

Dependencies: CF-SHELL, CF-EVIDENCE. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Goal

The publication-control entrypoint binds approval to exact run, findings, renderer and target identities and supports withholding and invalidation before any publication.

Dependencies: CF-SHELL, CF-EVIDENCE. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Required Outcome

The publication-control entrypoint binds approval to exact run, findings, renderer and target identities and supports withholding and invalidation before any publication.

Dependencies: CF-SHELL, CF-EVIDENCE. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Deliverables

Implement `publication` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/approval.rs and publication/manifest.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_ux.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

The publication-control entrypoint binds approval to exact run, findings, renderer and target identities and supports withholding and invalidation before any publication.

Dependencies: CF-SHELL, CF-EVIDENCE. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Acceptance Criteria

1. Through the installed product inspect exact run, finding set, artifact manifest, rendering contract/version, claims/nonclaims and destination identity; record explicit approval, withholding or invalidation with provenance. An approval flag unbound to identities is insufficient.
2. Require the exact fresh approval at the production publication admission boundary, and exercise an approved local controlled target plus denied/withheld attempts. This issue delivers approval enforcement and manifest admission; successor renderers deliver their own complete output. Do not claim a renderer exists from a fixture.
3. Mutate scope, finding set, rendered artifact/renderer version or target after approval and prove publication is denied until a new matching decision. Reject absent/stale approval, partial run misrepresented as complete, missing provenance, tampered manifest and redaction failure. Repository instructions never authorize approval.
4. Allow the operator to inspect withheld state and recover by making a new explicit decision. No default approval, automatic remote publishing or public hosting is introduced. Preserve the shared CF-EVIDENCE schema and its conformance vectors rather than a private UI contract.

acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`.

pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`.

stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

## Repo Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][CF-UX] Enforce exact-artifact publication approval

## One complete result

The publication-control entrypoint binds approval to exact run, findings, renderer and target identities and supports withholding and invalidation before any publication.

Dependencies: CF-SHELL, CF-EVIDENCE. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `publication` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/approval.rs and publication/manifest.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_ux.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Through the installed product inspect exact run, finding set, artifact manifest, rendering contract/version, claims/nonclaims and destination identity; record explicit approval, withholding or invalidation with provenance. An approval flag unbound to identities is insufficient.
2. Require the exact fresh approval at the production publication admission boundary, and exercise an approved local controlled target plus denied/withheld attempts. This issue delivers approval enforcement and manifest admission; successor renderers deliver their own complete output. Do not claim a renderer exists from a fixture.
3. Mutate scope, finding set, rendered artifact/renderer version or target after approval and prove publication is denied until a new matching decision. Reject absent/stale approval, partial run misrepresented as complete, missing provenance, tampered manifest and redaction failure. Repository instructions never authorize approval.
4. Allow the operator to inspect withheld state and recover by making a new explicit decision. No default approval, automatic remote publishing or public hosting is introduced. Preserve the shared CF-EVIDENCE schema and its conformance vectors rather than a private UI contract.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`.

pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`.

stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate.
Execution prerequisite: #891 (CF-SHELL); accepted output is required before dependent execution.
Execution prerequisite: #881 (CF-EVIDENCE); accepted output is required before dependent execution.

Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:47e883b113f99a596ba2b9abb774b7625291ccf80f060df8aca7441d0fff6435 -->

## Dependencies

#891 (CF-SHELL) and #881 (CF-EVIDENCE) are closed/accepted as of live read on 2026-09-16; remaining pre-bind gate is active path ownership/collision recheck.

## Target Files / Surfaces

Implement `publication` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/approval.rs and publication/manifest.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_ux.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Validation Plan

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_ux`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof.

`git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo Expectations

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_ux`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof.

`git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Non-goals

acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`.

pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`.

stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## Issue-Graph Notes

#891 (CF-SHELL) and #881 (CF-EVIDENCE) are closed/accepted as of live read on 2026-09-16; before bind, recheck active CodeFriend path ownership and do not absorb #893/#894.

## Notes

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

Live dependency read on 2026-09-16 confirms #891 (CF-SHELL) and #881 (CF-EVIDENCE) are closed/accepted. Before bind, recheck path ownership against active CodeFriend PRs/worktrees and preserve #893/#894 sibling boundaries.

Prepared, not bound. Execution Sprint 4 is a scheduling assignment, not dependency satisfaction. All implementation steps, proving runs, implementation review, PR and terminal state remain unstarted. Shared CLI dispatch/library, fixture directories and evidence/publication admission surfaces require current owner coordination after predecessors land.

Branch codex/895-v0922-publication-approval and its proposed FastWork path are unbound planning values only. Issue #926 owns umbrella management for all eleven sprints; it is not the Sprint 4 umbrella or a child execution prerequisite.

## Tooling Notes

Use native v3 with current authenticated authority. Preparation is stored only in resolved Git metadata. Schedule dependencies false; do not bind until accepted prerequisites and owner recheck. Create an issue-bound session goal before implementation. Do not hand-edit rendered cards.

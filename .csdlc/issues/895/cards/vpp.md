---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-publication-approval-validation-plan"
issue: 895
task_id: "issue-0895"
run_id: "issue-0895"
version: "0.92.2"
title: "[v0.92.2][CF-UX] Enforce exact-artifact publication approval"
branch: "codex/895-v0922-publication-approval"
generated_at: "2026-09-12T00:10:15.986006+00:00"
card_status: "ready"
status: "executed_local_review_pending"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution"
validation_family: "codefriend_publication_admission"
validation_size_split: "Focused named tests and touched-owner regressions; macOS/Linux CI separately observed"
expected_proof_cost: "Low-confidence estimate 1800 local validation seconds; excludes cold build, CI queue and independent review"
planned_validation_seconds: "1800"
planned_validation_tokens: "9000"
issue_goal_ref: "Active Sprint 4 #930 execution goal; #895 is the current bounded child objective"
sprint_goal_ref: "v0.92.2 execution Sprint 4; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/895/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/895"
  - kind: "stp"
    ref: ".csdlc/issues/895/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/895/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/895/cards/spp.md"
selected_lanes:
  - "runtime; deterministic installed CLI, canonical approval-store revocation, alternate-store and deleted-tail replay denial, exact verified-byte snapshot admission, and retained evidence/review compatibility; local runs passed and distinct remediation review pending"
parallel_groups:
  - "Focused local proof ran serially around the shared Cargo target; a distinct exact-head review follows the immutable remediation commit."
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml atomic_publication_uses_the_verified_snapshot_not_a_second_source_read --lib; cargo test --manifest-path adl/Cargo.toml --test codefriend_ux; cargo test --manifest-path adl/Cargo.toml --test codefriend_evidence; cargo test --manifest-path adl/Cargo.toml --test codefriend_review; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --all-targets --all-features -- -D warnings; git diff --check"
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "Executed local proof covers installed prepare/approve/inspect/admit behavior; approve-to-invalidate and approve-to-withhold denial; copied-old-approval alternate-store replay denial; deleted revocation-tail detection through the committed authoritative head; exact verified-byte snapshot publication without a second source read; binding mutations; incomplete runs; missing provenance; manifest/redaction/symlink/collision failures; evidence compatibility; review compatibility; formatting; strict warnings; and diff hygiene. No executed claim is made for chain fork, disconnected-gap, mixed-binding, non-JSON-entry, remote publication, provider, cloud, hosting, or Linux proof. Two independent reviews failed and their actionable findings are remediated locally; a distinct fresh review is required."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

With #891 and #881 accepted, recheck shared-owner paths and then adopt exact evidence/publication identity contracts; implement inspect/approve/withhold/invalidate decision records and manifest admission; enforce matching fresh human decision at the real publication boundary with a controlled local target; independently vary scope, finding set, artifact, renderer, destination and provenance to prove denial plus explicit-decision recovery; preserve pending successor renderer truth and obtain independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime; deterministic installed CLI, canonical approval-store revocation, alternate-store and deleted-tail replay denial, exact verified-byte snapshot admission, and retained evidence/review compatibility; local runs passed and distinct remediation review pending

## Parallelization Plan

- Parallel groups: Focused local proof ran serially around the shared Cargo target; a distinct exact-head review follows the immutable remediation commit.
- Validation runtime class: `bounded_local`
- Validation resource profile: `Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution`
- Validation family: `codefriend_publication_admission`
- Validation size split: `Focused named tests and touched-owner regressions; macOS/Linux CI separately observed`

## Goal Accounting Hooks

- Issue goal ref: `Active Sprint 4 #930 execution goal; #895 is the current bounded child objective`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/895/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1800 local validation seconds; excludes cold build, CI queue and independent review`
- Planned validation seconds: `1800`
- Planned validation token budget: `9000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml atomic_publication_uses_the_verified_snapshot_not_a_second_source_read --lib; cargo test --manifest-path adl/Cargo.toml --test codefriend_ux; cargo test --manifest-path adl/Cargo.toml --test codefriend_evidence; cargo test --manifest-path adl/Cargo.toml --test codefriend_review; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --all-targets --all-features -- -D warnings; git diff --check

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Executed local proof covers installed prepare/approve/inspect/admit behavior; approve-to-invalidate and approve-to-withhold denial; copied-old-approval alternate-store replay denial; deleted revocation-tail detection through the committed authoritative head; exact verified-byte snapshot publication without a second source read; binding mutations; incomplete runs; missing provenance; manifest/redaction/symlink/collision failures; evidence compatibility; review compatibility; formatting; strict warnings; and diff hygiene. No executed claim is made for chain fork, disconnected-gap, mixed-binding, non-JSON-entry, remote publication, provider, cloud, hosting, or Linux proof. Two independent reviews failed and their actionable findings are remediated locally; a distinct fresh review is required.

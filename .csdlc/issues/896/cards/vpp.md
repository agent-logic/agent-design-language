---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-codefriend-markdown-renderer-validation-plan"
issue: 896
task_id: "issue-0896"
run_id: "issue-0896"
version: "0.92.2"
title: "[v0.92.2][CF-RENDER-MD] Render an approved review as Markdown"
branch: "codex/896-v0922-codefriend-markdown-renderer"
generated_at: "2026-09-12T00:10:01.454600+00:00"
card_status: "ready"
status: "local_proof_passed_pending_review_and_ci"
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_consumer_and_visual_review_with_required_ci"
validation_resource_profile: "deterministic local CPU/filesystem and installed adl process; isolated temporary approval stores and destinations; no provider, network, browser, paid, cloud, HTML/PDF, or external-publication lane"
validation_family: "codefriend_renderer_consumer_contract"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization"
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "Active Sprint 4 #930 execution goal; #896 is the current bounded child objective"
sprint_goal_ref: "issue-926; all-eleven-sprint management only, not an execution dependency"
goal_metrics_rollup_ref: ".csdlc/evidence/896/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/896"
  - kind: "stp"
    ref: ".csdlc/issues/896/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/896/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/896/cards/spp.md"
selected_lanes:
  - "runtime: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md (5 passed); adjacent compatibility: codefriend_synthesis (5), codefriend_remediate (9), codefriend_testplan (8), codefriend_ux (7); strict Clippy for codefriend_render_md; cargo fmt; git diff --check; installed retained output inspection. Required PR CI and independent exact-head review remain pending."
parallel_groups:
  - "serial within this issue; independent fixtures may parallelize only with isolated state"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis --test codefriend_remediate --test codefriend_testplan --test codefriend_ux; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_render_md -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check"
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Local proof is complete and retained. Required independent exact-head review and normal PR CI remain open; no HTML/PDF parity, provider execution, browser observation, remote publication, or customer-scale claim is made."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Run focused deterministic Markdown renderer and installed CLI proof for actual accepted predecessor artifacts, positive output parity, manifest binding, stale/unapproved refusal, unsafe-link sanitation, redaction recheck, create-only target behavior, and artifact inspection; then run formatting, strict Clippy, diff hygiene, independent exact-head review, and required CI.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md (5 passed); adjacent compatibility: codefriend_synthesis (5), codefriend_remediate (9), codefriend_testplan (8), codefriend_ux (7); strict Clippy for codefriend_render_md; cargo fmt; git diff --check; installed retained output inspection. Required PR CI and independent exact-head review remain pending.

## Parallelization Plan

- Parallel groups: serial within this issue; independent fixtures may parallelize only with isolated state
- Validation runtime class: `bounded_local_consumer_and_visual_review_with_required_ci`
- Validation resource profile: `deterministic local CPU/filesystem and installed adl process; isolated temporary approval stores and destinations; no provider, network, browser, paid, cloud, HTML/PDF, or external-publication lane`
- Validation family: `codefriend_renderer_consumer_contract`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `Active Sprint 4 #930 execution goal; #896 is the current bounded child objective`
- Sprint goal ref: `issue-926; all-eleven-sprint management only, not an execution dependency`
- Goal metrics rollup ref: `.csdlc/evidence/896/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis --test codefriend_remediate --test codefriend_testplan --test codefriend_ux; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_render_md -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local proof is complete and retained. Required independent exact-head review and normal PR CI remain open; no HTML/PDF parity, provider execution, browser observation, remote publication, or customer-scale claim is made.

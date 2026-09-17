---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 898
task_id: "issue-0898"
run_id: "issue-0898"
version: "1.0.5"
title: "[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF"
branch: "codex/898-v0922-codefriend-pdf-renderer"
generated_at: "<timestamp>"
card_status: "ready"
status: "local_proof_passed_pending_review_and_ci"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "runtime"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "deterministic local CPU/filesystem and installed adl process; supplied local TrueType font; Poppler text/raster inspection; no provider, network, browser, paid, cloud, hosting, or external-publication lane"
validation_family: "<validation_family>"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned."
sprint_goal_ref: "#930"
goal_metrics_rollup_ref: "#930"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "runtime: codefriend_render_pdf (4 passed); merged-sibling regression: codefriend_render_html (5) and codefriend_render_md (5); strict Clippy for lib, bins and tests; cargo fmt; exact diff check; pdftotext semantic inspection; pdftoppm every-page visual inspection. Required PR CI and independent exact-head review remain pending."
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check; pdftotext and pdftoppm over retained report.pdf"
failure_policy: "<failure_policy>"
notes: "Local product and visual proof is complete and retained. Required independent exact-head review and normal PR CI remain open. The supplied font digest is bound in the manifest; no font bytes, credentials, remote resources, or public publication are included."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime: codefriend_render_pdf (4 passed); merged-sibling regression: codefriend_render_html (5) and codefriend_render_md (5); strict Clippy for lib, bins and tests; cargo fmt; exact diff check; pdftotext semantic inspection; pdftoppm every-page visual inspection. Required PR CI and independent exact-head review remain pending.

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `<validation_runtime_class>`
- Validation resource profile: `deterministic local CPU/filesystem and installed adl process; supplied local TrueType font; Poppler text/raster inspection; no provider, network, browser, paid, cloud, hosting, or external-publication lane`
- Validation family: `<validation_family>`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned.`
- Sprint goal ref: `#930`
- Goal metrics rollup ref: `#930`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `<planned_validation_seconds>`
- Planned validation token budget: `<planned_validation_tokens>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check; pdftotext and pdftoppm over retained report.pdf

## Failure Semantics

- <failure_policy>

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local product and visual proof is complete and retained. Required independent exact-head review and normal PR CI remain open. The supplied font digest is bound in the manifest; no font bytes, credentials, remote resources, or public publication are included.

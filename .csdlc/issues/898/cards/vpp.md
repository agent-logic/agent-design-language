---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-codefriend-pdf-renderer-validation-plan"
issue: 898
task_id: "issue-0898"
run_id: "issue-0898"
version: "0.92.2"
title: "[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF"
branch: "codex/898-v0922-codefriend-pdf-renderer"
generated_at: "2026-09-12T00:10:10.730683+00:00"
card_status: "ready"
status: "local_proof_passed_pending_review_and_ci"
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_consumer_and_visual_review_with_required_ci"
validation_resource_profile: "deterministic local CPU/filesystem and installed adl process; supplied local TrueType font; Poppler text/raster inspection; no provider, network, browser, paid, cloud, hosting, or external-publication lane"
validation_family: "codefriend_renderer_consumer_contract"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization"
planned_validation_seconds: "3600"
planned_validation_tokens: "20000"
issue_goal_ref: "Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned."
sprint_goal_ref: "#930"
goal_metrics_rollup_ref: "#930"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "stp"
    ref: ".csdlc/issues/898/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/898/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/898/cards/spp.md"
selected_lanes:
  - "runtime: codefriend_render_pdf (4 passed); merged-sibling regression: codefriend_render_html (5) and codefriend_render_md (5); strict Clippy for lib, bins and tests; cargo fmt; exact diff check; pdftotext semantic inspection; pdftoppm every-page visual inspection. Required PR CI and independent exact-head review remain pending."
parallel_groups:
  - "serial within this issue; independent fixtures may parallelize only with isolated state"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check; pdftotext and pdftoppm over retained report.pdf"
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Local product and visual proof is complete and retained. Required independent exact-head review and normal PR CI remain open. The supplied font digest is bound in the manifest; no font bytes, credentials, remote resources, or public publication are included."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Wait for accepted merged Markdown #896; select and pin an available rendering engine/version with bounded subprocess and resource contract while keeping Rust CLI ownership; implement installed export pdf from governed semantic input with manifest; enforce pre-export approval, target, provenance, redaction and no external-resource inclusion; extract text for canonical claim parity and visually inspect every representative page for long text/URLs/code/tables/page breaks/non-ASCII and missing/clipped content; reject engine failure/timeout/empty or unreadable output without success manifest; retain actual PDF/visual evidence, hashes, documentation and independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime: codefriend_render_pdf (4 passed); merged-sibling regression: codefriend_render_html (5) and codefriend_render_md (5); strict Clippy for lib, bins and tests; cargo fmt; exact diff check; pdftotext semantic inspection; pdftoppm every-page visual inspection. Required PR CI and independent exact-head review remain pending.

## Parallelization Plan

- Parallel groups: serial within this issue; independent fixtures may parallelize only with isolated state
- Validation runtime class: `bounded_local_consumer_and_visual_review_with_required_ci`
- Validation resource profile: `deterministic local CPU/filesystem and installed adl process; supplied local TrueType font; Poppler text/raster inspection; no provider, network, browser, paid, cloud, hosting, or external-publication lane`
- Validation family: `codefriend_renderer_consumer_contract`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned.`
- Sprint goal ref: `#930`
- Goal metrics rollup ref: `#930`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization`
- Planned validation seconds: `3600`
- Planned validation token budget: `20000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check; pdftotext and pdftoppm over retained report.pdf

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local product and visual proof is complete and retained. Required independent exact-head review and normal PR CI remain open. The supplied font digest is bound in the manifest; no font bytes, credentials, remote resources, or public publication are included.

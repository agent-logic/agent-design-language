---
issue_card_schema: adl.issue.v1
wp: "[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions"
slug: "876-provider-definitions"
title: "[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions"
labels:
  - "track:roadmap"
issue_number: 876
generated_at: "2026-09-11T23:55:23.353658+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "production-behavior"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/876"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency."
pr_start:
  enabled: true
  slug: "876-provider-definitions"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T23:55:23.353658+00:00

# Structured Task Prompt

## Summary

[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions

## Goal

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation.

## Required Outcome

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation.

## Deliverables

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation. Include production proof, failure handling and operator documentation required by the source issue.

## Acceptance Criteria

1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer.

## Repo Inputs

Full canonical issue https://github.com/agent-logic/agent-design-language/issues/876; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.

## Dependencies

Accepted merged output required from #864, #854, #622. No sprint-wide barrier or asynchronous closeout dependency.

## Target Files / Surfaces

Extend `adl/src/provider/reload.rs`, `adl/src/provider/profiles.rs`, and the minimum wiring in `adl/src/provider/mod.rs` and `adl/src/execute/runner.rs`. Read `docs/providers/provider-profile-hot-loading.md` and `docs/provider/inference-profiles.md`: existing `ProviderReloadOwner`/`ProviderReloadSnapshot` already validate a provider-only sidecar and use the kernel watcher. Reuse that production owner; do not add another registry/watcher. Own focused reload/profile tests and accompanying provider docs. Add a named data/schema/example file only after inventorying the current format and recording its exact path; do not expand into a provider rewrite.

## Validation Plan

1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer. PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Demo Expectations

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

## Non-goals

PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Issue-Graph Notes

Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency.

## Notes

Bound implementation and focused local proof recorded in .csdlc/evidence/876/IMPLEMENTATION_PROOF.md. Independent exact-head review and PR CI remain required; no merge or closeout claim.

## Tooling Notes

Native v3 bind/edit/validate/review/publish; stable installed owners; primary main inspection-only.

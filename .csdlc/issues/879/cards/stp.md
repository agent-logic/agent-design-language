---
issue_card_schema: adl.issue.v1
wp: "[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet"
slug: "879-github-ingestion"
title: "[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet"
labels:
  - "track:roadmap"
issue_number: 879
generated_at: "2026-09-11T23:55:26.355672+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/879"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency."
pr_start:
  enabled: true
  slug: "879-github-ingestion"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T23:55:26.355672+00:00

# Structured Task Prompt

## Summary

[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet

## Goal

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs.

## Required Outcome

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs.

## Deliverables

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs. Include production proof, failure handling and operator documentation required by the source issue.

## Acceptance Criteria

1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked.

## Repo Inputs

Full canonical issue https://github.com/agent-logic/agent-design-language/issues/879; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.

## Dependencies

Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency.

## Target Files / Surfaces

Use the shared product route selected by WP-01 and the predecessor's new `adl/src/codefriend/ingestion/mod.rs`/`local.rs` packet contract. Own proposed `adl/src/codefriend/ingestion/github.rs`, narrow registration in the selected `adl/src/cli/codefriend_cmd.rs`, and focused `adl/tests/codefriend_github_ingestion.rs`. These are new intended paths, resolved against CF-ADAPTER at execution; do not invent another packet schema or credential resolver. Read the adopted contract and portable-adapter feature. Native C-SDLC GitHub lifecycle authority is separate from this product's read-only repository acquisition.

## Validation Plan

1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked. PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Demo Expectations

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

## Non-goals

PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Issue-Graph Notes

Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency.

## Notes

Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed.

## Tooling Notes

Native v3 bind/edit/validate/review/publish; stable installed owners; primary main inspection-only.

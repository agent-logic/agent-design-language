---
issue_card_schema: adl.issue.v1
wp: "919-GROUP-B"
slug: "codefriend-result-integrity"
title: "[v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability"
labels:
  - "track:roadmap"
issue_number: 1162
generated_at: "2026-09-23T17:29:32.826544+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "implemented_fix"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/1162"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "One of the aggregate #919 remediation groups under #921; seven findings and no per-finding issues."
pr_start:
  enabled: true
  slug: "codefriend-result-integrity"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-23T17:29:32.826544+00:00

# Structured Task Prompt

## Summary

Repair the seven Group B CodeFriend result-integrity and website-interoperability findings from #919 as one aggregate issue under #921, while preserving #918 and #919.

## Goal

Repair the seven Group B CodeFriend result-integrity and website-interoperability findings from #919 as one aggregate issue under #921, while preserving #918 and #919.

## Required Outcome

Reviewed fixes and proving regressions for all seven findings across the ADL producer and website consumer, with exact finding disposition evidence.

## Deliverables

ADL publication, operator-attempt, PDF verification, focused regression, PVF, and proof-wording repairs; a separate CodeFriend website component PR for v4 compatibility, asynchronous authorization, pinned workflow actions, and actual native-emitted v4 fixtures; one retained finding-to-fix-to-test disposition map for all seven findings.

## Acceptance Criteria

CODE-001: multiline, CRLF, and tab-bearing admitted excerpts preserve visible semantics in Markdown, HTML, and extracted PDF. CODE-002: retry refuses an incomplete active provider attempt and cancel/retry concurrency preserves attempt identity. SEC-003: a resealed substituted PDF fails stage verification despite self-consistent hashes and metadata. INTEGRATION-001: the website accepts authentic native v4 results while retaining v2/v3 compatibility and rejecting mixed, stale, altered, false-complete, and future-version inputs. SEC-004: protected mutation and result/download paths re-authorize after awaited work. DEP-001: privileged deployment actions use exact commit SHAs and deploy-only OIDC scope. DEMOS-001: Cargo-built proof is labeled source-built, with installed proof claimed only when actually exercised. Every finding maps to an exact fix and proving test; independent exact-head review and applicable CI are required.

## Repo Inputs

AGENTS.md; active prompt registry and 1.0.5 schemas; #1162; #921; immutable #919 finding artifacts; frozen ADL candidate 5c4a6149771c637f3c805985b86231077965eab4; frozen website candidate a45e339c13b24716edbd3fadf29dddff36ffe02e; current source in both repositories.

## Dependencies

Aggregate issue #1162 is a Group B child of #921. #919 and #918 remain independently owned and read-only. The website fixture step depends on the repaired native producer; other disjoint repairs may proceed in parallel.

## Target Files / Surfaces

adl/src/codefriend/publication/{markdown.rs,html.rs,pdf.rs,relay.rs}; adl/src/codefriend/operator/mod.rs; focused adl/tests/codefriend_* regressions and coupled PVF records; docs/codefriend/PDF_EXPORT.md; CodeFriend website app/review-assessments.mjs, app/http.mjs, deploy workflow, tests, actual native-v4 fixtures, and PVF inventory.

## Validation Plan

cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action.

## Demo Expectations

Required isolated installed-command crash-recovery and operational transport regression proof; fixture-only, synthetic credentials, no customer/provider/cloud effects.

## Non-goals

No changes to frozen #918/#919 artifacts, per-finding issues, unrelated runtime/provider work, merge, deployment, release, hosted-provider spend, or shared owner-binary replacement.

## Issue-Graph Notes

Group B owns INTEGRATION-001, CODE-001, CODE-002, SEC-003, SEC-004, DEP-001, and DEMOS-001.

## Notes

Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation.

## Tooling Notes

Use authenticated native v3 prepare/bind/edit/validate/proof/review/publish. Draft values only; coordinator replaces numeric marker and validates before dispatch.

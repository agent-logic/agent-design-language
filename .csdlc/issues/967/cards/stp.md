---
issue_card_schema: adl.issue.v1
wp: "issue-967"
slug: "issue-967-deterministic-hosted-a2a"
title: "[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats"
labels:
  - "track:roadmap"
issue_number: 967
generated_at: "2026-09-12"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "implementation"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/967"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Part of #928; corrective follow-up to merged #855/PR964."
pr_start:
  enabled: true
  slug: "issue-967-deterministic-hosted-a2a"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12

# Structured Task Prompt

## Summary

Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery. Inherited corrective working-tree proof: Runtime production dispatch 1/1, OpenAPI contracts 11/11, zero-paid five-provider matrix 5/5 and hosted-topology matrix 3/3. These are local working-tree results, not #967 committed-head, CI or paid hosted acceptance. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.

## Goal

Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.

## Required Outcome

Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.

## Deliverables

adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata. Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.

## Acceptance Criteria

Refuse invalid, unknown, self-targeted, empty and over-limit actions before provider calls; dispatch one signed peer exchange after an ordinary reply; coalesce identical model/request actions; reject conflicts before peer dispatch; preserve absent-field model actions and replay fingerprints; align OpenAPI and Runtime boundaries; pass zero-paid 5/5 and 3/3 matrices, separately authorized bounded hosted acceptance, exact-head independent review and required CI.

## Repo Inputs

Live issue #967, merged #855/PR964, canonical Observatory intent and signed A2A delivery, OpenAPI contract tests and bounded lifecycle harness.

## Dependencies

#855 / PR #964 are merged. #967 owns the post-merge correction and is part of #928; prior issue closure is not corrective acceptance.

## Target Files / Surfaces

adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata.

## Validation Plan

CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.

## Demo Expectations

Zero-paid typed-wire matrices for five provider families and three hosted topologies; a separately authorized hosted OpenAI/Anthropic/Vertex run within existing cost, call, token and time bounds.

## Non-goals

No arbitrary JSON extraction from prose; no bypass of signatures, canonical addressing, capability checks or replay protection; no credential, billing, model or cloud configuration changes; no rewriting merged #855/PR964 history.

## Issue-Graph Notes

Part of #928; corrective follow-up to merged #855/PR964.

## Notes

Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.

## Tooling Notes

Native C-SDLC v3 edit and validate only; this request is prepared but not applied. Primary main inspection-only.

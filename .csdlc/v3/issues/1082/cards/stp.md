---
issue_card_schema: adl.issue.v1
wp: "1082"
slug: "bedrock-converse-resident-bindings"
title: "[v0.92.2][Runtime][Bedrock] Adopt Converse and migrate unreliable OpenRouter residents"
labels:
  - "track:roadmap"
issue_number: 1082
generated_at: "2026-09-19"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "runtime_provider_and_identity_contract"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/1082"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Follow-on to #1079 and consumer of #855/#876; owns only Bedrock Converse and the two named resident migrations."
pr_start:
  enabled: true
  slug: "bedrock-converse-resident-bindings"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-19

# Structured Task Prompt

## Summary

Replace both Nova-specific Bedrock execution routes with typed Converse, add current provider definitions, and add explicit continuity-preserving canonical-name migration for two residents.

## Goal

Make current Bedrock text models first-class Runtime providers without coupling durable residents to replaceable model names.

## Required Outcome

Kimi K2.5 and Nemotron Super 3 120B execute through the current provider spec and Bedrock Converse; harbor.axioma and quill.axioma retain their prior internal continuity and welcome state while using replaceable provider bindings.

## Deliverables

Typed Converse request/response path in both active and compatibility adapters; bounded response handling and typed failure projection; Kimi and Nemotron profiles with stable/native identity split; explicit resident identity migration route; continuity tests; operator docs; deterministic and bounded live proof.

## Acceptance Criteria

AC-1 Converse is model-neutral and no Nova payload reaches non-Nova models; AC-2 current ProviderSpec materializes stable/native model identity, codec, capabilities and bounded controls; AC-3 account/profile guard, response bound, timeout, cancellation and one-attempt behavior remain fail closed; AC-4 errors retain credentials, quota, model-unavailable, timeout, invalid-response and transport categories; AC-5 identity migration requires the exact old canonical name and preserves internal ID, conversations, office, welcome and history; AC-6 ordinary binding replacement cannot rename an agent; AC-7 exact Kimi and Nemotron inference succeeds; AC-8 welcome, conversations, A2A, health and Observatory proof pass; AC-9 focused tests, review and CI pass.

## Repo Inputs

Issue #1082 and issues #854, #855, #876 and #1079; adl-provider-core profile/substrate/adapter/registry implementation; adl provider communication adapter; Runtime dynamic-agent and provider-health state; existing two legacy resident declarations.

## Dependencies

#855, #876, #854 and #1079 are merged prerequisites. No additional issue is required before implementation.

## Target Files / Surfaces

adl-provider-core/Cargo.toml and Cargo.lock; adl-provider-core/src/http_family.rs, tests, profiles.rs, provider.rs and provider_substrate.rs; adl/src/provider_adapter.rs and lockfile; adl-runtime-kernel/src/control.rs and lockfile; provider and resident monitoring docs.

## Validation Plan

Provider-core full library test plus strict all-target Clippy; focused ADL Converse test; Runtime agent-lifecycle and provider tests; fmt and diff checks; two single-attempt live Bedrock probes capped at 64 output tokens; Runtime conversation/A2A/health/Observatory proof; native proof; exact-head independent review; CI.

## Demo Expectations

Use agent-logic-admin in us-west-2 with the expected-account hash, at most one attempt per exact model, 64 output tokens for direct qualification, no prompt/response retention, and no retry after an ambiguous outcome.

## Non-goals

No DeepSeek migration, broad model catalog, recurring probes, provider/model agent names, identity reset, credential changes, cloud provisioning, or unrelated Runtime repair.

## Issue-Graph Notes

Provider specification and durable resident identity are separate layers and must remain independently replaceable.

## Notes

The AWS SDK can classify failures only after dispatch and a caller cancellation cannot prove server cancellation; retain one-attempt and stop-after-failure limits and never automatically replay an ambiguous call.

## Tooling Notes

Use native C-SDLC v3 in the bound FastWork worktree; keep primary main inspection-only; use the Agent Logic business AWS profile.

---
issue_card_schema: adl.issue.v1
wp: "970"
slug: "aprovider-effective-inference-configuration"
title: "[provider architecture] Make declared AProvider inference configuration effective and observable"
labels:
  - "track:roadmap"
issue_number: 970
generated_at: "2026-09-15"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "provider_runtime_contract"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/970"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Follow-up to #514, #876, and #855; discovered during #900; must preserve #901 as the recovery owner."
pr_start:
  enabled: true
  slug: "aprovider-effective-inference-configuration"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-15

# Structured Task Prompt

## Summary

Add one validated effective inference configuration to the normalized GeneralProvider dispatch target, enforce built-in codec support before execution, and prove Ollama HTTP serialization locally.

## Goal

Eliminate silent configuration loss between AProvider declarations and provider wire requests.

## Required Outcome

The normalized dispatch target carries canonical effective controls and their redacted fingerprint; selected codecs explicitly consume or reject supplied controls; Ollama HTTP sends its supported values.

## Deliverables

Typed effective inference configuration and support declarations; pre-dispatch executable and unsupported-control rejection; Ollama HTTP codec forwarding; redacted projection and fingerprint; profile/direct/reload parity proof; focused tests; operator documentation; native lifecycle evidence.

## Acceptance Criteria

AC-1 canonical normalization and schema; AC-2 profile, reload, Runtime, and direct dispatch parity; AC-3 trusted built-in codec boundary and executable-field rejection; AC-4 codec support declarations and unsupported-control rejection; AC-5 Ollama HTTP full supported-control forwarding; AC-6 redacted projection and stable fingerprint; AC-7 focused positive and negative proof; AC-8 compatibility and operator documentation.

## Repo Inputs

Issue #970; adl-provider-core ProviderSpec, profile expansion, candidate admission, ProviderInvocationTargetV1, GeneralProvider factory, Ollama HTTP codec, and local provider tests; current provider documentation.

## Dependencies

#514, #876, and #855 are closed; #901 remains a separate failure/recovery qualification boundary and is not absorbed.

## Target Files / Surfaces

adl-provider-core/src/provider_substrate.rs; adl-provider-core/src/candidate.rs; adl-provider-core/src/http_family.rs; adl-provider-core/src/http_family/tests.rs; adl-provider-core/src/profiles.rs; docs/provider/inference-profiles.md.

## Validation Plan

Focused Rust tests for normalization, candidate rejection, profiles, and Ollama wire capture; cargo fmt check for adl-provider-core; git diff --check; native proof; independent exact-head review; required CI after publication.

## Demo Expectations

Local deterministic wire-capture server only; no live provider, credentials, download, or paid execution.

## Non-goals

No lifecycle redesign, provider recovery qualification, arbitrary provider plugin system, hosted-provider live qualification, model download, or hardware work.

## Issue-Graph Notes

This issue owns only configuration normalization, codec consumption, observability, and the Ollama proving path.

## Notes

A broad rejection rule could break valid transport/auth settings; executable-field and inference-control classification must be explicit and bounded.

## Tooling Notes

Use native C-SDLC v3 for lifecycle writes and the bound FastWork worktree for all tracked work.

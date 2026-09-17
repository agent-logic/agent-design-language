---
issue_card_schema: adl.issue.v1
wp: "CF-AGENT prerequisite for Sprint 10 #936"
slug: "v0922-installed-local-review-agent"
title: "[v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent"
labels:
  - "track:roadmap"
issue_number: 1058
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "document"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/1058"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Shared #1056 protocol accepted for dependent implementation at 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, independently reviewed with seven component tests. Branch stacks on that candidate; no claim of deployed/provider acceptance. #1057 implements the website side of the agent protocol; #914 integrates completed components."
pr_start:
  enabled: true
  slug: "v0922-installed-local-review-agent"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-16T20:16:52.878587+00:00

# Structured Task Prompt

## Summary

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Goal

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Required Outcome

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Deliverables

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Acceptance Criteria

authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input.

## Repo Inputs

## Complete implementation outcome

Result: a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

Acceptance: authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input.

Dependencies and interfaces: accepted shared model-access and authenticated run-control contracts, existing local operator/review consumers, website pairing controls. Test deterministic failure cases and separately prove a real model-backed local review. #914 connects accepted components; this issue owns agent functionality, not whole-product integration or independent qualification.


## Scope and execution authority

Operator-authorized prerequisite under Sprint #936 for integration #914, created after accepted ADR0084 / PR1054. Separate from the historical69-task denominator. #914 consumes accepted implemented output; #915 independently qualifies the integrated candidate. No component-only proof can close either integration or qualification.

All three prerequisites retain both Beta1 website modes, invitation-only GitHub sign-in, Agent Logic model access for both modes, CLI support and BYOK deferral. Google sign-in is not approved. Preserve evidence/privacy/retention and exact-artifact approval; no automatic external publication or source mutation. Use native cards, an issue-bound worktree/goal, focused PVF classification, meaningful positive/negative tests, independent exact-head review and required checks. Deployment/provider execution needs explicit bounded operational authority.

## Validation contract

Record exact candidate, source, environment and actual artifact identities. Declare new tests lane, proof role, determinism, resource bounds and release-gate status. Local deterministic fixtures cover failure/authorization invariants; separately record real installed/provider/browser observations. No zero-test or synthetic-only product acceptance. Fix actionable findings before publishing implementation PR.

Execution prerequisite: #1056 accepted shared server/model-access and run-control contracts. Coordinate disjoint work after contract acceptance; #914 consumes the completed component.


## Dependencies

Shared #1056 protocol accepted for dependent implementation at 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, independently reviewed with seven component tests. Branch stacks on that candidate; no claim of deployed/provider acceptance. #1057 implements the website side of the agent protocol; #914 integrates completed components.

## Target Files / Surfaces

adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md

## Validation Plan

cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization.

## Demo Expectations

cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization.

## Non-goals

No Google sign-in, BYOK, open signup, unrelated repository migration, automatic merge/release, or claim of whole-product qualification.

## Issue-Graph Notes

Shared #1056 protocol accepted for dependent implementation at 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, independently reviewed with seven component tests. Branch stacks on that candidate; no claim of deployed/provider acceptance. #1057 implements the website side of the agent protocol; #914 integrates completed components.

## Notes

Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them.

## Tooling Notes

Native v3 only; no v2 fallback, raw lifecycle writes or installed binary replacement.

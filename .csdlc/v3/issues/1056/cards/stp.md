---
issue_card_schema: adl.issue.v1
wp: "Sprint 10 umbrella #1056"
slug: "v0922-server-review-model-access"
title: "[v0.92.2][CF-SERVER] Execute hosted reviews and provide governed model access"
labels:
  - "track:roadmap"
issue_number: 1056
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
  - "https://github.com/agent-logic/agent-design-language/issues/1056"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Coordination may start now without claiming children ready. #914 preserves all numeric prerequisites and accepted output gates, including unfinished #897 HTML report and #898 PDF report. #915 waits for the accepted exact #914 installed candidate. Sprint numbering creates no all-earlier-sprints barrier. Native finish and cleanup remain asynchronous and do not gate downstream execution."
pr_start:
  enabled: true
  slug: "v0922-server-review-model-access"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-16T20:16:52.878587+00:00

# Structured Task Prompt

## Summary

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Goal

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Required Outcome

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Deliverables

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Acceptance Criteria

real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

## Repo Inputs

## Complete implementation outcome

Result: invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

Acceptance: real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

Dependencies and interfaces: existing review pipeline consumers; shared provider definitions and privacy/retention contracts; website identity contract. Agree the model-access and run-control contracts before the local agent or website depend on them. Contract fixtures are not real-provider acceptance. Current pre-extraction code remains in ADL until separately authorized repository migration.


## Scope and execution authority

Operator-authorized prerequisite under Sprint #936 for integration #914, created after accepted ADR0084 / PR1054. Separate from the historical69-task denominator. #914 consumes accepted implemented output; #915 independently qualifies the integrated candidate. No component-only proof can close either integration or qualification.

All three prerequisites retain both Beta1 website modes, invitation-only GitHub sign-in, Agent Logic model access for both modes, CLI support and BYOK deferral. Google sign-in is not approved. Preserve evidence/privacy/retention and exact-artifact approval; no automatic external publication or source mutation. Use native cards, an issue-bound worktree/goal, focused PVF classification, meaningful positive/negative tests, independent exact-head review and required checks. Deployment/provider execution needs explicit bounded operational authority.

## Validation contract

Record exact candidate, source, environment and actual artifact identities. Declare new tests lane, proof role, determinism, resource bounds and release-gate status. Local deterministic fixtures cover failure/authorization invariants; separately record real installed/provider/browser observations. No zero-test or synthetic-only product acceptance. Fix actionable findings before publishing implementation PR.


## Dependencies

Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation.

## Target Files / Surfaces

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Validation Plan

PVF: deterministic component authorization/lifecycle negatives plus separately authorized real installed/provider/browser proof. Pin exact source, binary, environment, input/output and nonzero scenario counts. Native semantic_card_projections validator proves preparation tooling only and cannot satisfy implementation acceptance. Replace or supplement it with the authored component validators before proof/publication. real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

## Demo Expectations

PVF: deterministic component authorization/lifecycle negatives plus separately authorized real installed/provider/browser proof. Pin exact source, binary, environment, input/output and nonzero scenario counts. Native semantic_card_projections validator proves preparation tooling only and cannot satisfy implementation acceptance. Replace or supplement it with the authored component validators before proof/publication. real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

## Non-goals

No Google sign-in, BYOK, open signup, unrelated repository migration, automatic merge/release, or claim of whole-product qualification.

## Issue-Graph Notes

Coordination may start now without claiming children ready. #914 preserves all numeric prerequisites and accepted output gates, including unfinished #897 HTML report and #898 PDF report. #915 waits for the accepted exact #914 installed candidate. Sprint numbering creates no all-earlier-sprints barrier. Native finish and cleanup remain asynchronous and do not gate downstream execution.

## Notes

Local implementation explicitly authorized in ADL. Execution design and dependencies are recorded in SPP. Deployment and paid provider proof remain separate bounded approvals; no product acceptance claim until they pass.

## Tooling Notes

Native v3 only; no v2 fallback, raw lifecycle writes or installed binary replacement.

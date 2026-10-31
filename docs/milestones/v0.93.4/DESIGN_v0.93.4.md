# v0.93.4 Design

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## Purpose

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## Problem Statement

The former 59-identity platform milestone combines independent foundation, tooling and cognitive programmes. Separate them without losing contracts, historical identity or acceptance.

## Goals

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## Non-Goals

No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover. Component container packaging and other unadmitted TBD concepts remain deferred.

## Scope

C-SDLC v4 simplification. 9 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Package boundaries are first-pass design proposals under the approved C-SDLC v4 scope; validate them against actual workflow measurements.

## Requirements

| WP | Acceptance / proof | Adverse cases |
| --- | --- | --- |
| WP-01 | Approve one bounded issue-to-delivery workflow and measurable overhead/compatibility denominator before implementation. | Missing predecessor acceptance; Unsupported completion claim |
| CS4-01 | Map create/start, implementation/proof, review, publish/deliver and optional closeout to source-backed sequence diagrams; measure wall/internal/external-tool/network time and identify steps to remove. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-02 | One editable issue source, one canonical installation, minimal command transitions and precise remote identity replace duplicate projections/attestations/receipts. Define bounded recovery and explicit compatibility; retain only independently justified integrity checks. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-03 | Prototype ordinary operations and interrupted recovery against the approved denominator; demonstrate whether each sample stays below 30 seconds internal overhead and a ten-operation path below 300 seconds, reporting cold/warm and external costs separately. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-04 | Read existing v3 issue/proof/review state without rewriting history; preserve exact repository/issue/head and uncertain-effect recovery. Prove rollback and a staged cutover with one canonical installed tool set; no automatic authority fallback. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-05 | Implement only the reviewed workflow with changed-input validation and reusable proof/review, no repeated binary hashing or duplicate builds/CI, and asynchronous optional administrative closeout. Prove supported native macOS/Linux behavior. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| CS4-06 | Independent installed end-to-end acceptance covers creation through delivery, metadata-only edits, unchanged proof reuse, conflicts, lost/uncertain responses, crash recovery and optional cleanup. Every ordinary sample meets the internal-overhead goals; failed or ambiguous effects are not silently replayed. | Duplicate remote effect; Stale review accepted; Incompatible v3 state; Unaccounted internal overhead |
| REVIEW | Resolve actionable correctness, compatibility and performance findings once on the final candidate; no extra attestation/receipt campaign. | Missing predecessor acceptance; Unsupported completion claim |
| RELEASE | Obtain explicit operator cutover decision, preserve recoverable v3 state, then record one canonical admitted installation and the supported migration/rollback outcome. No cutover is authorized by this plan. | Missing predecessor acceptance; Unsupported completion claim |

## Proposed Design

The required workflow is issue intake → optional preparation/bind → bounded implementation → focused proof → independent review → reviewed publication/delivery → optional asynchronous cleanup. Start with source-backed sequence diagrams and baseline measurements; command names and storage layout are design outputs, not predetermined new machinery.

Use one editable semantic issue source and one canonical admitted installation. Validate changed inputs; reuse valid exact-input proof/review; remove redundant projection drift, attestations, receipts and repeated binary hashing unless an identified threat/recovery invariant requires a check. Keep repository/issue/head identity and uncertain-remote-effect reconciliation explicit. Do not introduce a second state engine or blanket concurrency cap.

Performance acceptance is **less than 30 seconds internal overhead for each ordinary-operation sample**, and **less than 300 seconds internal overhead for the standard ten-operation lifecycle**. Record sample count, cold/warm conditions, wall time and independently measured network/external-tool time; report unclassified time as internal rather than hiding it. These are design/performance goals, **not hard kill timers**; do not reinstate the fixed wall-clock deadlines targeted by C-SDLC #328. Report p50/p95/max without using averages to hide failed samples.

Native macOS and Linux, interrupted/uncertain remote operations, changed metadata and unchanged proof reuse, conflicting heads, old v3 records and rollback all need compatibility proof. Do not bulk-rewrite old evidence or switch shared binaries from this plan.

## Interfaces And Contracts

Runtime foundations export accepted versioned plugin/lifecycle/state/configuration interfaces. C-SDLC exports an accepted v3-compatible workflow/cutover contract. Cognitive consumers preserve identity, consent, privacy and existing resource/governance checks.

## Validation Plan

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.

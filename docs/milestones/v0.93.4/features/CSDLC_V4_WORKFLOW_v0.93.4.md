# v0.93.4 C-SDLC v4 workflow and compatibility

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.4. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## Template Rules

First-pass source-grounded plan, not implementation, installed qualification or release approval.

## Purpose

The required workflow is issue intake → optional preparation/bind → bounded implementation → focused proof → independent review → reviewed publication/delivery → optional asynchronous cleanup. Start with source-backed sequence diagrams and baseline measurements; command names and storage layout are design outputs, not predetermined new machinery.

Use one editable semantic issue source and one canonical admitted installation. Validate changed inputs; reuse valid exact-input proof/review; remove redundant projection drift, attestations, receipts and repeated binary hashing unless an identified threat/recovery invariant requires a check. Keep repository/issue/head identity and uncertain-remote-effect reconciliation explicit. Do not introduce a second state engine or blanket concurrency cap.

Performance acceptance is **less than 30 seconds internal overhead for each ordinary-operation sample**, and **less than 300 seconds internal overhead for the standard ten-operation lifecycle**. Record sample count, cold/warm conditions, wall time and independently measured network/external-tool time; report unclassified time as internal rather than hiding it. These are design/performance goals, **not hard kill timers**; do not reinstate the fixed wall-clock deadlines targeted by C-SDLC #328. Report p50/p95/max without using averages to hide failed samples.

Native macOS and Linux, interrupted/uncertain remote operations, changed metadata and unchanged proof reuse, conflicting heads, old v3 records and rollback all need compatibility proof. Do not bulk-rewrite old evidence or switch shared binaries from this plan.

## Context

Operator scope decision on 2026-10-30; preserve prior accepted contracts and explicitly distinguish proposed design choices.

## Coverage / Ownership

Cognitive-SDLC owns implementation, installation and native tests. ADL owns this cross-milestone plan. The current authenticated v3 package remains operational authority until a separately accepted v4 cutover.

## Overview

The required workflow is issue intake → optional preparation/bind → bounded implementation → focused proof → independent review → reviewed publication/delivery → optional asynchronous cleanup. Start with source-backed sequence diagrams and baseline measurements; command names and storage layout are design outputs, not predetermined new machinery.

Use one editable semantic issue source and one canonical admitted installation. Validate changed inputs; reuse valid exact-input proof/review; remove redundant projection drift, attestations, receipts and repeated binary hashing unless an identified threat/recovery invariant requires a check. Keep repository/issue/head identity and uncertain-remote-effect reconciliation explicit. Do not introduce a second state engine or blanket concurrency cap.

Performance acceptance is **less than 30 seconds internal overhead for each ordinary-operation sample**, and **less than 300 seconds internal overhead for the standard ten-operation lifecycle**. Record sample count, cold/warm conditions, wall time and independently measured network/external-tool time; report unclassified time as internal rather than hiding it. These are design/performance goals, **not hard kill timers**; do not reinstate the fixed wall-clock deadlines targeted by C-SDLC #328. Report p50/p95/max without using averages to hide failed samples.

Native macOS and Linux, interrupted/uncertain remote operations, changed metadata and unchanged proof reuse, conflicting heads, old v3 records and rollback all need compatibility proof. Do not bulk-rewrite old evidence or switch shared binaries from this plan.

## Design

The required workflow is issue intake → optional preparation/bind → bounded implementation → focused proof → independent review → reviewed publication/delivery → optional asynchronous cleanup. Start with source-backed sequence diagrams and baseline measurements; command names and storage layout are design outputs, not predetermined new machinery.

Use one editable semantic issue source and one canonical admitted installation. Validate changed inputs; reuse valid exact-input proof/review; remove redundant projection drift, attestations, receipts and repeated binary hashing unless an identified threat/recovery invariant requires a check. Keep repository/issue/head identity and uncertain-remote-effect reconciliation explicit. Do not introduce a second state engine or blanket concurrency cap.

Performance acceptance is **less than 30 seconds internal overhead for each ordinary-operation sample**, and **less than 300 seconds internal overhead for the standard ten-operation lifecycle**. Record sample count, cold/warm conditions, wall time and independently measured network/external-tool time; report unclassified time as internal rather than hiding it. These are design/performance goals, **not hard kill timers**; do not reinstate the fixed wall-clock deadlines targeted by C-SDLC #328. Report p50/p95/max without using averages to hide failed samples.

Native macOS and Linux, interrupted/uncertain remote operations, changed metadata and unchanged proof reuse, conflicting heads, old v3 records and rollback all need compatibility proof. Do not bulk-rewrite old evidence or switch shared binaries from this plan.

## Execution Flow

```mermaid
flowchart LR
  I[Issue intake] --> S[Optional start / bind]
  S --> W[Implement]
  W --> P[Focused proof]
  P --> R[Independent review]
  R --> D[Publish / deliver green head]
  D --> C[Optional async cleanup]
```

CS4-01 must produce sequence diagrams for each operation with observed current steps, removal candidates and retained invariants.

## Determinism and Constraints

No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover.

## Integration Points

Consume accepted predecessor interfaces through their owning repositories; do not restore sibling source or duplicate Runtime/tooling engines.

## Validation

CS4-06 must demonstrate the per-sample and lifecycle overhead goals, native macOS/Linux compatibility, no duplicate effects and safe restart/rollback at the accepted candidate. One final review accepts the integrated result; earlier valid evidence is reused.

## Acceptance Criteria

CS4-06 must demonstrate the per-sample and lifecycle overhead goals, native macOS/Linux compatibility, no duplicate effects and safe restart/rollback at the accepted candidate. One final review accepts the integrated result; earlier valid evidence is reused.

## Risks

Compatibility, recovery and uncertain external effects require bounded issue-level proof. Estimates and fixtures are not live acceptance.

## Future Work

Any expansion is separately planned; component container packaging remains deferred.

## Notes

Publication/claims and implementation activation require their own accepted evidence and operator decisions.

---
issue_card_schema: adl.issue.v1
wp: "[v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch"
slug: "877-uts-package"
title: "[v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch"
labels:
  - "track:roadmap"
issue_number: 877
generated_at: "2026-09-11T23:55:24.168359+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/877"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency."
pr_start:
  enabled: true
  slug: "877-uts-package"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T23:55:24.168359+00:00

# Structured Task Prompt

## Summary

[v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch

## Goal

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result.

## Required Outcome

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result.

## Deliverables

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result. Include production proof, failure handling and operator documentation required by the source issue.

## Acceptance Criteria

1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue.

## Repo Inputs

Full canonical issue https://github.com/agent-logic/agent-design-language/issues/877; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.

## Dependencies

Accepted merged output required from #864. No sprint-wide barrier or asynchronous closeout dependency.

## Target Files / Surfaces

Read `docs/specs/uts/README.md`, `UTS_V1.0_SCHEMA.md`, `UTS_V1.1_SCHEMA.md`, and `adl-spec/schemas/uts/`. Existing types are in `adl/src/uts.rs`; conformance in `adl/src/uts_conformance.rs`; ACC compilation in `adl/src/uts_acc_compiler/`; production consumers include `adl/src/resident_tool_execution.rs`, `adl/src/tool_registry.rs` and `adl/src/governed_executor.rs`. UTS describes tools; ACC retains runtime authority. Documentation calls v1 the guaranteed baseline while source also includes v1.1 types; reconcile actual supported semantics rather than inferring complete v1.1 implementation from type presence.  Selected package location: a new in-repository `adl-uts/` Rust crate, initial package version `0.1.0`, with manifest, canonical types/schema assets and focused tests. Package version and UTS schema version are distinct. Preserve existing supported `uts.v1` and `uts.v1.1` serialization/types and declare their actual implemented compatibility separately; introducing this package does not claim every proposed v1.1 semantic is implemented. Make `adl/Cargo.toml` consume the package and migrate only necessary UTS imports/reexports and the named production dispatch path. Preserve ACC enforcement and unrelated APIs. No external registry publication or standalone repository creation is implied.

## Validation Plan

1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue. PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

## Demo Expectations

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

## Non-goals

PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

## Issue-Graph Notes

Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency.

## Notes

Prepared only. Child implementation and its review have not run.

## Tooling Notes

Native v3 bind/edit/validate/review/publish; stable installed owners; primary main inspection-only.

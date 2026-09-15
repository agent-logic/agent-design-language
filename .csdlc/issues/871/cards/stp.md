---
issue_card_schema: adl.issue.v1
wp: "SIM-05"
slug: "v0922-derived-card-projections"
title: "[v0.92.2][SIM-05] Derived cards and precise evidence invalidation"
labels:
  - "track:roadmap"
issue_number: 871
generated_at: "2026-09-15T16:31:32Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "implementation_and_executed_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/871"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Dependency #870 is satisfied. Existing umbrella remains #866. Bind #871 through the active native v3 owner, then implement only derived projections and evidence invalidation; SIM-06 retains copied-record conversion."
pr_start:
  enabled: true
  slug: "v0922-derived-card-projections"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-15T16:31:32Z

# Structured Task Prompt

## Summary

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Goal

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Required Outcome

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Deliverables

- Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation).
- Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`.
- Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources.
- Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents.

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Acceptance Criteria

1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged.
2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner.
3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table.
4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation.
5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions.
6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout.
7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task.

## Repo Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][SIM-05] Rebuild six card projections from the semantic issue record

## Outcome

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Dependencies and execution boundary

- Depends on SIM-04's merged application/transaction owner and semantic record.
- Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer.
- Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation.

## Current source and bounded implementation

- Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation).
- Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`.
- Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources.
- Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents.

## Acceptance and executed proof

1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged.
2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner.
3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table.
4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation.
5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions.
6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout.
7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task.

## Validation / PVF

Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer.

## Stop conditions and non-goals

Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot.


## Canonical sprint links

Planning and issue-creation owner: #864.
Sprint umbrella: #866 (coordination; does not gate SIM-01 startup).
Execution prerequisite: #870 (SIM-04), with accepted merged output before dependent execution.

Creation contract reviewed at `ed2a93338c92fda62ba63761d56af21b50483eb4`. Issue creation is not implementation start or live activation.

## Dependencies

SIM-04/#870 is accepted and closed: PR #969 head 25b84230e8acdabc7affebcd372f0f8cbfd687e8 merged as 41f6132aa0bff2ec264a00276d237b936bc5006d; native finish authenticated issue/PR terminal truth.

## Target Files / Surfaces

- Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation).
- Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`.
- Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources.
- Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents.

## Validation Plan

Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer.

Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient):
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`
- `git diff --check`
Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Demo Expectations

Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer.

Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient):
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`
- `git diff --check`
Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Non-goals

Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot.

## Issue-Graph Notes

Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding.

## Notes

- Depends on SIM-04's merged application/transaction owner and semantic record.
- Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer.
- Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation.

Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding.

Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits.

## Tooling Notes

Native v3 issue/edit/validate preparation in resolved Git metadata. Bind through native v3 only after dependency and ownership recheck. Create an issue-bound session goal before implementation. Never hand-edit generated cards or weaken stale guards.

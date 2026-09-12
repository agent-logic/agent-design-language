---
issue_card_schema: adl.issue.v1
wp: "CSDLC-MERGE"
slug: "v0922-merge-linkage-admission"
title: "[v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge"
labels:
  - "track:roadmap"
issue_number: 849
generated_at: "2026-09-12T00:22:05.416044+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/849"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "#864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added."
pr_start:
  enabled: true
  slug: "v0922-merge-linkage-admission"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:22:05.416044+00:00

# Structured Task Prompt

## Summary

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Goal

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Required Outcome

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Deliverables

Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes.

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Acceptance Criteria

- [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue.
- [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open.
- [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation.
- [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected.
- [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract.
- [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards.
- [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race.
- [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence.

## Repo Inputs

Complete source contract (live issue snapshot; preserve all requirements):

## Problem

At reviewed revision `25498d7709cb5fe13242aac81988ecdb344db968` (PR #847, issue #844), native PR merge admission does not bind or validate the PR's current closing/non-closing relation to the reviewed `PublicationLinkage`.

A `PartOf` PR body can change to `Closes #<parent>` without a head change. Exact-head review and check admission can still succeed, allowing merge to close the parent. Conversely, removing the closing relation from a `Closing` PR can leave its target open. Later `finish` validation cannot undo the merge or issue closure.

Independent reviewer: `review_835`; severity P2; finding `MERGE-LINKAGE-001`. Discovered during #835 release proof and handed to #522/#833. Operator explicitly allocated this repair to **v0.92.2**. This allocation is not a behavioral pass or v0.92.1 release approval.

## Evidence

- `csdlc-v3/src/commands/remote/mod.rs:205`: `PullRequestMerge` lacks `PublicationLinkage` and mode.
- `csdlc-v3/src/commands/remote/merge.rs:38`: authenticated query omits PR body/closing relation.
- `csdlc-v3/src/commands/remote/merge.rs:345`: review admission binds repository, issue and revision, but not the current PR relation.

Existing publication and finish guards retain their prior guarantees; the defect is the new merge path. The exact merged revision passed 240 unfiltered C-SDLC tests, strict all-target Clippy and the V3-A contract check; these checks do not cover this missing guard.

## Required change

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Acceptance criteria

- [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue.
- [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open.
- [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation.
- [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected.
- [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract.
- [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards.
- [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race.
- [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence.

## Focused validation

PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof.

Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission.

## Scope

Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition.




## Current milestone execution links

Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution.

The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge.




## Execution sprint assignment

**Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force.

## Dependencies

#864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.

## Target Files / Surfaces

Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes.

## Validation Plan

PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof.

Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission.

Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory.

## Demo Expectations

PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof.

Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission.

Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory.

## Non-goals

Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition.

## Issue-Graph Notes

#864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.

## Notes

Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution.

The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge.

#864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.

Preparation only: proposed branch/worktree unbound; implementation and proof unstarted. #926 owns all-eleven umbrella management only. Preserve active SIM changes and do not replace stable installed writers or change lifecycle authority. For #906, registered .worktrees/adl-process-status-fanout at branch codex/reduce-process-status-fanout contains dirty process_cmd.rs and cli_smoke/process_status.rs plus unrelated finish files. Preserve all bytes and resolve both source/test owners before any extraction/reuse. For #849, resolve remote merge/types/test ownership with current SIM and #907 work; no live merge is authorized. Shared-path readiness remains unchecked/unresolved, not a new semantic dependency.

## Tooling Notes

Native v3 issue/edit/validate preparation in resolved Git metadata. Bind through native v3 only after dependency and ownership recheck. Create an issue-bound session goal before implementation. Never hand-edit generated cards or weaken stale guards.

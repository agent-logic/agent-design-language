# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0849
Run ID: issue-0849
Version: 0.92.2
Title: [v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge
Branch: codex/849-v0922-merge-linkage-admission
Card Status: ready
Generated: 2026-09-12T00:22:05.416044+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/849
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/849
- Docs: AGENTS.md; csdlc-v3/AGENTS.md for native owner edits; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml; ATOMIC_TASK_CONTRACTS_v0.92.2.json; current issue source and exact source baseline
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with native v3 `csdlc bind` only if execution later becomes necessary.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Lifecycle Semantics
- Lifecycle stage: `SIP`
- Activation state: active after issue-intent review.
- Next stage: `STP`, where the selected task or solution is made explicit.
- Downstream planning path: `STP -> SPP -> VPP -> SRP -> SOR` once execution planning becomes concrete.
- Legacy compatibility: older references may call this an input card, but new issue work should treat it as the Structured Issue Prompt.

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .csdlc/issues/849/cards/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

## Execution
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug: v0922-merge-linkage-admission
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.

## Required Outcome

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

## Inputs

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

## Target Files / Surfaces

Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes.

## Validation Plan

PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof.

Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission.

Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory.

## Demo / Proof Requirements

PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof.

Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission.

Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use authenticated native C-SDLC v3 for lifecycle routing. Retained typed v2 requires explicit issue-scoped rollback or remediation approval.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after native v3 `csdlc bind`.
- Keep validation focused on the touched surface.

## System Invariants (must remain true)

- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition.

## Notes / Risks

#849 remains bound under Sprint 7 #933. Accepted origin/main integrated at 900f1bf107; merged #948 pending-receipt assertions and curl configuration guard preserved. Source 94a39db15 fixes two publication-only receipt fixtures. Full native suite passes 262 tests with zero ignored/filtered; hosted CI remains a separate exact pushed-head gate. No merge or shared binary installation authorized.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

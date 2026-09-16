# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0906
Run ID: issue-0906
Version: 0.92.2
Title: [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor
Branch: codex/906-v0922-process-parser-simplification
Card Status: ready
Generated: 2026-09-12T00:22:05.416044+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/906
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/906
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
  output_card: .csdlc/issues/906/cards/sor.md
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
- Source issue-prompt slug: v0922-process-parser-simplification
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Required Outcome

One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Acceptance Criteria

1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged.
2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification.
3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms.
4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary.

PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor

## One complete result

One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Exact responsibility and ownership

Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output.

The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite.

## Executed acceptance

1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged.
2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification.
3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms.
4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary.

PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`.

pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`.

stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`.

non_goals: `repo_wide_rewrite`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 7; this grouping adds no execution gate.
Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution.

Reviewed creation source: `d955fd1bdbe7f79433dbcf1ea8426536ec8274a7`. This issue records a complete task; creation does not claim execution or acceptance.

## Target Files / Surfaces

Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output.

The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite.

## Validation Plan

Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification.

## Demo / Proof Requirements

Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification.

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

acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`.

pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`.

stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`.

non_goals: `repo_wide_rewrite`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

## Notes / Risks

Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

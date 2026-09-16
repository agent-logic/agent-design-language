# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0862
Run ID: issue-0862
Version: 0.92.2
Title: [v0.92.2][C-SDLC v3][refactor] Decompose the local command owner
Branch: codex/862-v0922-local-command-decomposition
Card Status: ready
Generated: 2026-09-16T00:27:06.103043+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/862
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/862
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md; docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md; csdlc-v3/AGENTS.md; csdlc-v3/README.md
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
  output_card: .csdlc/issues/862/cards/sor.md
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
- Source issue-prompt slug: v0922-local-command-decomposition
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior.

## Required Outcome

Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior.

## Acceptance Criteria

1. Every extracted local module has one named responsibility and no command-domain cycles.
2. All local production routes use the decomposition; moving code into a replacement god module does not qualify.
3. Public and serialized contracts, digests, error codes and paths remain compatible.
4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision.

## Inputs

Full live source contract, retained without dropping requirements:

## Outcome

Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior.

## Complete task

- Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules.
- Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch.
- Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics.
- Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory.

## Acceptance

1. Every extracted local module has one named responsibility and no command-domain cycles.
2. All local production routes use the decomposition; moving code into a replacement god module does not qualify.
3. Public and serialized contracts, digests, error codes and paths remain compatible.
4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision.

## Remote decomposition routing

`CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction.

## Non-goals

Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation.


## Operator-authorized task split

The operator requested one complete task per issue in the #864 / PR #865 planning review. Additional work is preserved in `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`. The earlier scope-only edit created no follow-on issues; the operator subsequently authorized their creation and independent review through #864. All implementation tasks include their required tests, failure handling and documentation.


<!-- csdlc-v3-operation:59c2d9609840f5ff7993e765cb441904734af432a3c34d4eb8dc388d7c4012e9 -->

## Current milestone execution links

Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution.

The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge.

Separate complete tasks: CSDLC-REMOTE / #907. Their scopes and acceptance remain separate from this issue.


<!-- csdlc-v3-operation:cecdae1f02b944b3a812c800f49cd9f1fb7d479564a19c8b70a40a18186201b9 -->

## Execution sprint assignment

**Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force.


<!-- csdlc-v3-operation:5e863d373f23aba2aaefe31239c730d99cd13de16bef957c05248703e936a708 -->

## Target Files / Surfaces

- Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules.
- Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch.
- Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics.
- Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory.

Own csdlc-v3/src/commands/local/mod.rs and proposed cohesive sibling modules beneath commands/local/ for binding, editing, validation, registry/registration, transactions and state persistence. Coordinate csdlc-v3/tests/local_commands.rs, operational_cli_commands.rs, transactions.rs and affected boundary tests with SIM owners; final file names follow the reviewed responsibility inventory.

## Validation Plan

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

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

Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation.

`CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction.

## Notes / Risks

#864 WP-01 is accepted via merged PR #865 and the all-69 creation/review gate is satisfied. #849 is accepted via merged PR #952. #862 has no dependency on unfinished SIM-06 through SIM-09 or on an entire earlier sprint. Live ownership reconciliation on 2026-09-15 found the former #867 conflict obsolete: #867 through #871 are closed with merged PRs. The current #872 worktree changes only issue-872 rehearsal scripts and csdlc-v3/tests/copied_record_conversion_rehearsal.rs; it does not modify csdlc-v3/src/commands/local/mod.rs or the #862 focused test owners. No branch, worktree, or PR exists for #862 before this bind. Bind is authorized for execution readiness only. Implementation, validation execution, review, publication, merge, stable binary replacement, and live conversion remain unstarted and unauthorized by this preparation. Recheck live ownership before implementation and coordinate any later shared test or installed-binary write. Sprint 7 coordination is #933; #926 is historical umbrella-creation management, not a child dependency.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0794
Run ID: issue-0794
Version: 1.0.4
Title: [v0.92.2][tooling] Add native typed duplicate and no-op issue closure
Branch: codex/794-native-typed-issue-close
Card Status: ready
Generated: <timestamp>

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/794
- PR:
- Source Issue Prompt: <source_issue_prompt>
- Docs: <docs_context>
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with `csdlc-bind` only if execution later becomes necessary.
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
  output_card: <output_card>
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
- Source issue-prompt slug: native-typed-issue-close
- Required outcome type: native C-SDLC v3 GitHub issue mutation tooling
- Demo required: <demo_required>

## Goal

Add a native typed C-SDLC v3 issue-close operation for duplicate, superseded, and no-op dispositions without fabricating implementation completion.

## Required Outcome

Operators can close an explicitly selected duplicate/superseded/no-op issue through the v3 GitHub issue owner with exact repo/issue identity, durable intent, authenticated readback, idempotent recovery, and documented usage.

## Acceptance Criteria

- `GithubMutation` supports issue-close without raw gh or v2 fallback.
- Close requests require exact repository, non-zero issue, non-empty rationale, and duplicate owner when disposition is duplicate.
- The route records GitHub closed/not_planned state and authenticates readback before receipt.
- Restart/replay reconciles an already-closed exact operation without replaying mutation.
- Focused fake-adapter tests cover success, replay, and rejection cases.
- CLI usage documents and emits the same typed dispatch.

## Inputs

<inputs>

## Target Files / Surfaces

- `csdlc-v3/src/commands/remote/mod.rs`
- `csdlc-v3/src/main.rs`
- `csdlc-v3/src/commands/remote/tests.rs`
- `csdlc-v3/tests/operational_cli_commands.rs`
- `docs/csdlc-v3/CONTRACT.md`

## Validation Plan

- `cargo fmt --manifest-path csdlc-v3/Cargo.toml`
- `cargo test --manifest-path csdlc-v3/Cargo.toml issue_close -- --nocapture`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands -- --nocapture`
- `cargo test --manifest-path csdlc-v3/Cargo.toml commands::remote::tests -- --nocapture`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest -- --nocapture`
- `git diff --check`

## Demo / Proof Requirements

<demo_proof_requirements>

## Constraints / Policies

- Follow `AGENTS.md`.
- Use the typed C-SDLC v2 operator skills and Rust binaries for lifecycle routing.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after `csdlc-bind`.
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

- Do not close implementation-complete issues; native finish remains the implementation terminal route.
- Do not absorb #791 storage repair or broader lifecycle rewrite.
- Do not reintroduce v2 or raw gh as default authority.

## Notes / Risks

<notes_risks>

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

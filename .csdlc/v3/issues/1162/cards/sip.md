# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1162
Run ID: issue-1162
Version: 0.92.2
Title: [v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability
Branch: codex/1162-codefriend-result-integrity
Card Status: ready
Generated: 2026-09-23T17:29:32.826544+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1162
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1162
- Docs: AGENTS.md; active prompt registry and 1.0.5 schemas; #1162; #921; immutable #919 finding artifacts; frozen ADL candidate 5c4a6149771c637f3c805985b86231077965eab4; frozen website candidate a45e339c13b24716edbd3fadf29dddff36ffe02e; current source in both repositories.
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
  output_card: .csdlc/issues/1162/cards/sor.md
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
- Source issue-prompt slug: codefriend-result-integrity
- Required outcome type: implemented_fix
- Demo required: true

## Goal

Repair the seven Group B CodeFriend result-integrity and website-interoperability findings from #919 as one aggregate issue under #921, while preserving #918 and #919.

## Required Outcome

Reviewed fixes and proving regressions for all seven findings across the ADL producer and website consumer, with exact finding disposition evidence.

## Acceptance Criteria

CODE-001: multiline, CRLF, and tab-bearing admitted excerpts preserve visible semantics in Markdown, HTML, and extracted PDF. CODE-002: retry refuses an incomplete active provider attempt and cancel/retry concurrency preserves attempt identity. SEC-003: a resealed substituted PDF fails stage verification despite self-consistent hashes and metadata. INTEGRATION-001: the website accepts authentic native v4 results while retaining v2/v3 compatibility and rejecting mixed, stale, altered, false-complete, and future-version inputs. SEC-004: protected mutation and result/download paths re-authorize after awaited work. DEP-001: privileged deployment actions use exact commit SHAs and deploy-only OIDC scope. DEMOS-001: Cargo-built proof is labeled source-built, with installed proof claimed only when actually exercised. Every finding maps to an exact fix and proving test; independent exact-head review and applicable CI are required.

## Inputs

AGENTS.md; active prompt registry and 1.0.5 schemas; #1162; #921; immutable #919 finding artifacts; frozen ADL candidate 5c4a6149771c637f3c805985b86231077965eab4; frozen website candidate a45e339c13b24716edbd3fadf29dddff36ffe02e; current source in both repositories.

## Target Files / Surfaces

adl/src/codefriend/publication/{markdown.rs,html.rs,pdf.rs,relay.rs}; adl/src/codefriend/operator/mod.rs; focused adl/tests/codefriend_* regressions and coupled PVF records; docs/codefriend/PDF_EXPORT.md; CodeFriend website app/review-assessments.mjs, app/http.mjs, deploy workflow, tests, actual native-v4 fixtures, and PVF inventory.

## Validation Plan

cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action.

## Demo / Proof Requirements

Required isolated installed-command crash-recovery and operational transport regression proof; fixture-only, synthetic credentials, no customer/provider/cloud effects.

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

No changes to frozen #918/#919 artifacts, per-finding issues, unrelated runtime/provider work, merge, deployment, release, hosted-provider spend, or shared owner-binary replacement.

## Notes / Risks

Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

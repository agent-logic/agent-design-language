# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0879
Run ID: issue-0879
Version: v0.92.2
Title: [v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet
Branch: codex/879-github-ingestion
Card Status: ready
Generated: 2026-09-11T23:55:26.355672+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/879
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/879
- Docs: docs/milestones/v0.92.2/
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
  output_card: .csdlc/issues/879/cards/sor.md
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
- Source issue-prompt slug: 879-github-ingestion
- Required outcome type: production-behavior
- Demo required: yes

## Goal

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs.

## Required Outcome

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs.

## Acceptance Criteria

1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked.

## Inputs

Source issue, canonical v0.92.2 contracts and accepted dependency outputs.

## Target Files / Surfaces

Use the shared product route selected by WP-01 and the predecessor's new `adl/src/codefriend/ingestion/mod.rs`/`local.rs` packet contract. Own proposed `adl/src/codefriend/ingestion/github.rs`, narrow registration in the selected `adl/src/cli/codefriend_cmd.rs`, and focused `adl/tests/codefriend_github_ingestion.rs`. These are new intended paths, resolved against CF-ADAPTER at execution; do not invent another packet schema or credential resolver. Read the adopted contract and portable-adapter feature. Native C-SDLC GitHub lifecycle authority is separate from this product's read-only repository acquisition.

## Validation Plan

1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked. PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Demo / Proof Requirements

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

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

PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Notes / Risks

Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

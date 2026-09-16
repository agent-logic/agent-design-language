# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0896
Run ID: issue-0896
Version: 0.92.2
Title: [v0.92.2][CF-RENDER-MD] Render an approved review as Markdown
Branch: codex/896-v0922-codefriend-markdown-renderer
Card Status: ready
Generated: 2026-09-12T00:10:01.454600+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/896
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/896
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md
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
  output_card: .csdlc/issues/896/cards/sor.md
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
- Source issue-prompt slug: v0922-codefriend-markdown-renderer
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

The Markdown renderer consumes the governed publication contract and produces a complete usable report and manifest with traceable claims.

Dependencies: CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Required Outcome

The Markdown renderer consumes the governed publication contract and produces a complete usable report and manifest with traceable claims.

Dependencies: CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Acceptance Criteria

1. Consume actual approved publication inputs plus completed synthesis, remediation and test-plan outputs; produce a complete Markdown report and bound manifest through the installed renderer. Include source scope/revision, findings and citations, attribution/severity/uncertainty, disagreements, actionable plans, failures/omissions and approval/renderer identity.
2. Claims and finding identities match the governed semantic input exactly. Execute canonical claim-set parity and snapshot checks, including empty findings, long text, partial/not-comparable evidence and withheld publication. A template filled by hand is not proof.
3. Refuse missing/stale approval, changed renderer/target/artifact identity and missing provenance. Recheck redaction at render/export, reject leaked secrets and sanitize unsafe links/embedded content. Write only to the explicitly selected local target; no external publication is implied.
4. Open/read the actual emitted report and manifest, verify usable citations and complete content, and retain actual output hashes. HTML/PDF parity later compares against this canonical semantic report; Markdown does not claim those renderers work.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][CF-RENDER-MD] Render an approved review as Markdown

## One complete result

The Markdown renderer consumes the governed publication contract and produces a complete usable report and manifest with traceable claims.

Dependencies: CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `export markdown` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/markdown.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_md.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Consume actual approved publication inputs plus completed synthesis, remediation and test-plan outputs; produce a complete Markdown report and bound manifest through the installed renderer. Include source scope/revision, findings and citations, attribution/severity/uncertainty, disagreements, actionable plans, failures/omissions and approval/renderer identity.
2. Claims and finding identities match the governed semantic input exactly. Execute canonical claim-set parity and snapshot checks, including empty findings, long text, partial/not-comparable evidence and withheld publication. A template filled by hand is not proof.
3. Refuse missing/stale approval, changed renderer/target/artifact identity and missing provenance. Recheck redaction at render/export, reject leaked secrets and sanitize unsafe links/embedded content. Write only to the explicitly selected local target; no external publication is implied.
4. Open/read the actual emitted report and manifest, verify usable citations and complete content, and retain actual output hashes. HTML/PDF parity later compares against this canonical semantic report; Markdown does not claim those renderers work.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `markdown_report_rendered`, `manifest_binding`, `claim_parity`, `output_parity`.

pvf: `markdown_report_rendered`, `manifest_binding`, `claim_parity`, `unapproved_render_denied`, `redaction_rechecked`, `renderer_snapshot`, `redaction_recheck`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `renderer_claim_drift`, `redaction_check_failed`, `approval_missing_or_stale`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate.
Execution prerequisite: #895 (CF-UX); accepted output is required before dependent execution.
Execution prerequisite: #892 (CF-SYNTHESIS); accepted output is required before dependent execution.
Execution prerequisite: #893 (CF-REMEDIATE); accepted output is required before dependent execution.
Execution prerequisite: #894 (CF-TESTPLAN); accepted output is required before dependent execution.

Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance.

## Target Files / Surfaces

Implement `export markdown` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/markdown.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_md.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Validation Plan

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Concrete planned commands after implementation: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check. Execute the installed export against passing and denied fixtures; required nonzero behavior and semantic parity cannot be replaced by a green generic suite. Read the emitted Markdown and bound manifest, resolve citations and verify complete semantic content and target hashes.

## Demo / Proof Requirements

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Concrete planned commands after implementation: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check. Execute the installed export against passing and denied fixtures; required nonzero behavior and semantic parity cannot be replaced by a green generic suite. Read the emitted Markdown and bound manifest, resolve citations and verify complete semantic content and target hashes.

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



## Notes / Risks

Use the v0.92.2 creation selections and adopted design contract. Preserve the pinned Vector scope, both license notices, read-only source, immutable CodeFriend evidence semantics, stdout/stderr separation, and the shared CLI registration boundary. Dependencies are satisfied on current origin/main: #892/PR #1001, corrective #893/PR #1027, #894/PR #1026, and #895/PR #1025 are merged and ancestral to the bound base 870e4d14f3c74547cf3523f6807a4d3e338c873f. #926 is management only, not an execution prerequisite. Implementation, proof, review, publication, and integration remain pending; no provider, paid, cloud, or external-publication effect is authorized.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

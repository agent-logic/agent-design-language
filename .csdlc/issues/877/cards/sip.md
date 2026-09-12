# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0877
Run ID: issue-0877
Version: v0.92.2
Title: [v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch
Branch: codex/877-uts-package
Card Status: ready
Generated: 2026-09-11T23:55:24.168359+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/877
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/877
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
  output_card: .csdlc/issues/877/cards/sor.md
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
- Source issue-prompt slug: 877-uts-package
- Required outcome type: production-behavior
- Demo required: yes

## Goal

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result.

## Required Outcome

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result.

## Acceptance Criteria

1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue.

## Inputs

Source issue, canonical v0.92.2 contracts and accepted dependency outputs.

## Target Files / Surfaces

Read `docs/specs/uts/README.md`, `UTS_V1.0_SCHEMA.md`, `UTS_V1.1_SCHEMA.md`, and `adl-spec/schemas/uts/`. Existing types are in `adl/src/uts.rs`; conformance in `adl/src/uts_conformance.rs`; ACC compilation in `adl/src/uts_acc_compiler/`; production consumers include `adl/src/resident_tool_execution.rs`, `adl/src/tool_registry.rs` and `adl/src/governed_executor.rs`. UTS describes tools; ACC retains runtime authority. Documentation calls v1 the guaranteed baseline while source also includes v1.1 types; reconcile actual supported semantics rather than inferring complete v1.1 implementation from type presence.  Selected package location: a new in-repository `adl-uts/` Rust crate, initial package version `0.1.0`, with manifest, canonical types/schema assets and focused tests. Package version and UTS schema version are distinct. Preserve existing supported `uts.v1` and `uts.v1.1` serialization/types and declare their actual implemented compatibility separately; introducing this package does not claim every proposed v1.1 semantic is implemented. Make `adl/Cargo.toml` consume the package and migrate only necessary UTS imports/reexports and the named production dispatch path. Preserve ACC enforcement and unrelated APIs. No external registry publication or standalone repository creation is implied.

## Validation Plan

1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue. PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

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

PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

## Notes / Risks

Prepared only. Child implementation and its review have not run.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

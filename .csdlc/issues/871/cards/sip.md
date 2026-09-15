# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0871
Run ID: issue-0871
Version: 0.92.2
Title: [v0.92.2][SIM-05] Derived cards and precise evidence invalidation
Branch: codex/871-v0922-derived-card-projections
Card Status: ready
Generated: 2026-09-15T16:31:32Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/871
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/871
- Docs: AGENTS.md; csdlc-v3/AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md
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
  output_card: .csdlc/issues/871/cards/sor.md
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
- Source issue-prompt slug: v0922-derived-card-projections
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Required Outcome

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Acceptance Criteria

1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged.
2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner.
3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table.
4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation.
5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions.
6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout.
7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task.

## Inputs

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

## Demo / Proof Requirements

Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer.

Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient):
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`
- `git diff --check`
Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

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

Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot.

## Notes / Risks

SIM-04/#870 is accepted and closed: PR #969 head 25b84230e8acdabc7affebcd372f0f8cbfd687e8 merged as 41f6132aa0bff2ec264a00276d237b936bc5006d, and native finish recorded terminal closed-out truth. Preserve owners #849/#862/#907 and recheck shared paths before edits. Use the active pre-conversion native owner to bind this legacy prepared record; build and test the merged semantic candidate only in isolated destinations. No live record conversion, shared owner replacement, or writer activation is authorized.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

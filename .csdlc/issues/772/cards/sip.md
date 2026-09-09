# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0772
Run ID: issue-0772
Version: 1.0.4
Title: [v0.92.1][TAIL-06.16][security] Prove GCP-B audit and log posture
Branch: codex/772-prove-gcp-b-audit-log-posture
Card Status: ready
Generated: 2026-09-09T00:00:00-07:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/772
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/772
- Docs: Root AGENTS.md native C-SDLC v3 workflow, #772 live issue body, #520 source review T520-TEST-004, #522 parent remediation, #769 R520-013 redaction prerequisite, and current GCP-B/#740 proof artifacts.
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
  output_card: .csdlc/issues/772/cards/sor.md
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
- Source issue-prompt slug: gcp-b-audit-log-posture
- Required outcome type: security proof remediation with sanitized cloud readback evidence
- Demo required: authorized GCP read-only audit/log posture proof

## Goal

Prove GCP-B-ac-1 audit configuration and representative log delivery/readback for the intended GCP-B environment without retaining credentials or raw sensitive payloads.

## Required Outcome

A candidate-bound, sanitized proof packet must define the exact audit/log assertions, prove them against the accepted GCP-B project and provider identity, validate stale/wrong-project/unredacted failure modes, and preserve the six proved and four amended #740 rows.

## Acceptance Criteria

- The exact audit services, sinks/settings, log classes, identity, project/environment, and readback assertions owned by GCP-B-ac-1 are explicit.
- Authorized evidence proves the intended configuration exists in the intended GCP environment.
- A representative audit/log record is shown to be delivered and readable through the approved path, with sensitive fields safely omitted or generalized.
- The retained proof binds provider identity, environment identity, timestamp, candidate SHA, command/assertion class, and result without embedding credentials or local key paths.
- The GCP-B proof validator and exact-current semantic mapping pass for GCP-B-ac-1 and reject stale or mismatched evidence.
- The six proved and four amended historical rows retain their existing closed dispositions.

## Inputs

- GitHub issue #772.
- Parent remediation #522 and internal review #520 / T520-TEST-004.
- Exact reviewed candidate c24f8fa65ce445b03ce6cd69007307291d78b60c.
- `docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md`.
- `.csdlc/evidence/740/*.redacted.json` and `.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh`.
- `docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/current-exceptions.json`.
- #769 / R520-013 publication redaction boundary.

## Target Files / Surfaces

- `.csdlc/prepared/issues/772/run-gcp-b-audit-log-posture.sh`
- `.csdlc/prepared/issues/772/validate-gcp-b-audit-log-posture.sh`
- `.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-b/audit-log-posture.md`
- narrowly coupled semantic mapping/update evidence for `GCP-B-ac-1`

## Validation Plan

- Run static validator for script assertions and retained-artifact redaction checks.
- Run authorized read-only GCP audit/log posture proof using the approved GCP-B credential source without printing key contents.
- Run negative fixtures for missing log readback, stale candidate SHA, wrong project/provider identity, and unsafe retained content.
- Run redaction/audit checks before retaining proof because #772 depends on #769.

## Demo / Proof Requirements

The proof must demonstrate configuration readback and representative audit/log delivery/readback from the intended GCP-B environment; static configuration presence alone is not sufficient.

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

- Do not rerun unrelated paid GCP proofs.
- Do not reopen the six proved or four accepted-amendment #740 criteria.
- Do not embed credentials, local key paths, account secrets, token values, or raw sensitive log payloads.
- Do not expand GCP architecture beyond the existing GCP-B-ac-1 contract.
- Do not claim external publication readiness while #769 redaction gating remains unsatisfied.

## Notes / Risks

#769/R520-013 is still open, so #772 can prepare and prove locally but final retained-publication truth must remain gated until the complete manifest redaction boundary is satisfied or equivalent issue-local redaction proof is reviewed.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

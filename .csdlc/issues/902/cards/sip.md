# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0902
Run ID: issue-0902
Version: 0.92.2
Title: [v0.92.2][QUAL-EVIDENCE] Validate criterion-bound Runtime qualification evidence
Branch: codex/902-v0922-runtime-criterion-evidence
Card Status: ready
Generated: 2026-09-12T00:14:10.636008+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/902
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/902
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
  output_card: .csdlc/issues/902/cards/sor.md
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
- Source issue-prompt slug: v0922-runtime-criterion-evidence
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

Deliver and execute a fail-closed five-row evidence consumer that admits current criterion-specific producer/execution records only after exact criterion, provenance, scenario and independent-review checks. This is a working validator consuming real completed producer results, not a manually authored aggregate packet or a substitute for missing qualification.

## Required Outcome

Deliver and execute a fail-closed five-row evidence consumer that admits current criterion-specific producer/execution records only after exact criterion, provenance, scenario and independent-review checks. This is a working validator consuming real completed producer results, not a manually authored aggregate packet or a substitute for missing qualification.

## Acceptance Criteria

1. Admit exactly `RUST-01-ac-4`, `DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2` and `DRT-C-ac-3`. Bind each exact criterion text/digest to its canonical source revision, producer revision, execution profile/log/artifact digests, required scenario set and independently reviewed result. Preserve explicit many-to-one producer mappings without permitting cross-criterion evidence substitution.
2. Execute the consumer against actual completed QUAL-INVENTORY two-revision measurements, QUAL-RESIDENT population/workload/signed-restore proofs, QUAL-PROVIDER failure/recovery proofs and #852 dispatch/WSS/correlation/redaction proofs. Missing producer execution leaves the relevant row not-proven and blocks aggregate completion; authoring a mapping is insufficient.
3. Preserve original 19 findings separately from the five criterion rows. Preserve the five cloud-control gaps and two execution-proof gaps as distinct historical categories. Reconcile references for historical consumers #522 and #833 without reopening/closing them or asserting all their original findings newly proved by five rows.
4. Negative fixtures individually remove/alter criterion text/digest, source/producer revision, scenario, execution log, signature/provenance and independent review; substitute another criterion's evidence, self-authored approval, stale/partial evidence or synthetic metadata. Every invalid case must fail admission with actionable reasons, not merely fail JSON parsing.
5. Verify producer evidence and actual outcomes rather than trusting a success boolean. Independent review binds exact candidate and artifacts; absence or unresolved findings cannot become accepted. Report per-row pass/fail/not-proven plus complete/excluded/missing denominators and residual risks.
6. Expose this validator as the real consumer used by the current qualification evidence workflow, and retain a positive run on complete real inputs plus all negatives. Public sanitized manifests may reference protected artifacts under authorized verification, but inaccessible proof cannot be represented as independently verified. No release approval is conferred.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][Runtime qualification][QUAL-EVIDENCE] Validate complete criterion-bound Runtime qualification evidence

## One complete result

Deliver and execute a fail-closed five-row evidence consumer that admits current criterion-specific producer/execution records only after exact criterion, provenance, scenario and independent-review checks. This is a working validator consuming real completed producer results, not a manually authored aggregate packet or a substitute for missing qualification.

## Owned implementation and proof paths

Select `adl/tools/validate_v0922_runtime_qualification.py` and a focused negative-test companion under `adl/tools/` as new paths unless an equivalent existing current consumer is found and explicitly selected before implementation. Inputs are the completed #852 failure-event proof, QUAL-RESIDENT, QUAL-PROVIDER and QUAL-INVENTORY outputs. Inspect retained `docs/milestones/v0.92.1/evidence/release/tail-01/required-lane-denominator.json`, source issue text preserved under `.csdlc/evidence/864/task-scope-revision/`, and the existing #851 branch/worktree only after ownership reconciliation. Do not rewrite historical packets or hand-edit any lifecycle cards.

## Executed acceptance

1. Admit exactly `RUST-01-ac-4`, `DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2` and `DRT-C-ac-3`. Bind each exact criterion text/digest to its canonical source revision, producer revision, execution profile/log/artifact digests, required scenario set and independently reviewed result. Preserve explicit many-to-one producer mappings without permitting cross-criterion evidence substitution.
2. Execute the consumer against actual completed QUAL-INVENTORY two-revision measurements, QUAL-RESIDENT population/workload/signed-restore proofs, QUAL-PROVIDER failure/recovery proofs and #852 dispatch/WSS/correlation/redaction proofs. Missing producer execution leaves the relevant row not-proven and blocks aggregate completion; authoring a mapping is insufficient.
3. Preserve original 19 findings separately from the five criterion rows. Preserve the five cloud-control gaps and two execution-proof gaps as distinct historical categories. Reconcile references for historical consumers #522 and #833 without reopening/closing them or asserting all their original findings newly proved by five rows.
4. Negative fixtures individually remove/alter criterion text/digest, source/producer revision, scenario, execution log, signature/provenance and independent review; substitute another criterion's evidence, self-authored approval, stale/partial evidence or synthetic metadata. Every invalid case must fail admission with actionable reasons, not merely fail JSON parsing.
5. Verify producer evidence and actual outcomes rather than trusting a success boolean. Independent review binds exact candidate and artifacts; absence or unresolved findings cannot become accepted. Report per-row pass/fail/not-proven plus complete/excluded/missing denominators and residual risks.
6. Expose this validator as the real consumer used by the current qualification evidence workflow, and retain a positive run on complete real inputs plus all negatives. Public sanitized manifests may reference protected artifacts under authorized verification, but inaccessible proof cannot be represented as independently verified. No release approval is conferred.

## Dependencies and preserved scope

Execution prerequisites: QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: five_criterion_mappings_complete, producer_revision_and_criterion_digest_bound, independent_review_bound, exact_criterion_binding, independent_review, criterion_text_digest_source_and_producer_revision_execution_logs, historical_19_findings_5_cloud_control_2_execution_gaps_preserved, all_scenarios_present, self_authored_and_missing_review_evidence_rejected.
- PVF: five_criterion_mappings_complete, producer_revision_and_criterion_digest_bound, independent_review_bound, synthetic_metadata_rejected, cross_criterion_substitution_rejected, stale_or_partial_evidence_rejected, evidence_integrity_negatives, criterion_text_digest_source_and_producer_revision_execution_logs, historical_19_findings_5_cloud_control_2_execution_gaps_preserved, all_scenarios_present, self_authored_and_missing_review_evidence_rejected.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.


## Canonical execution links

Planning owner: #864. Creation/review batch: 5; this grouping adds no execution gate.
Execution prerequisite: #852 (QUAL-RUNTIME); accepted output is required before dependent execution.
Execution prerequisite: #900 (QUAL-RESIDENT); accepted output is required before dependent execution.
Execution prerequisite: #901 (QUAL-PROVIDER); accepted output is required before dependent execution.
Execution prerequisite: #899 (QUAL-INVENTORY); accepted output is required before dependent execution.

Reviewed creation source: `b31eb6904d66bdb006b6eaeda6cdcd5f2a401260`. This issue records a complete task; creation does not claim execution or acceptance.

## Target Files / Surfaces

Select `adl/tools/validate_v0922_runtime_qualification.py` and a focused negative-test companion under `adl/tools/` as new paths unless an equivalent existing current consumer is found and explicitly selected before implementation. Inputs are the completed #852 failure-event proof, QUAL-RESIDENT, QUAL-PROVIDER and QUAL-INVENTORY outputs. Inspect retained `docs/milestones/v0.92.1/evidence/release/tail-01/required-lane-denominator.json`, source issue text preserved under `.csdlc/evidence/864/task-scope-revision/`, and the existing #851 branch/worktree only after ownership reconciliation. Do not rewrite historical packets or hand-edit any lifecycle cards.

Selected companion: adl/tools/test_validate_v0922_runtime_qualification.py; tightly coupled proof inventory under .csdlc/evidence/902/. Current qualification workflow invokes adl/tools/validate_v0922_runtime_qualification.py directly with explicit input manifest; document the final argv after producer interfaces land. New scripts are proposed absent paths. Do not change producer runtime logic in this issue.

## Validation Plan

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Concrete planned checks after implementation: python3 adl/tools/test_validate_v0922_runtime_qualification.py (new focused test path; nonzero independently failing mutation cases required); python3 adl/tools/validate_v0922_runtime_qualification.py --help and actual declared manifest invocation on complete real producer inputs; git diff --check. Pin exact CLI flags when implemented and update SPP/VPP before running. Enumerate every five-row source/producer/scenario/review mapping and negative case in the PVF inventory. Preserve original19, cloud5, execution2 separately. Run required shared-owner regression/CI only for touched surfaces. A fixture positive does not replace the required current real-producer positive invocation.

## Demo / Proof Requirements

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Concrete planned checks after implementation: python3 adl/tools/test_validate_v0922_runtime_qualification.py (new focused test path; nonzero independently failing mutation cases required); python3 adl/tools/validate_v0922_runtime_qualification.py --help and actual declared manifest invocation on complete real producer inputs; git diff --check. Pin exact CLI flags when implemented and update SPP/VPP before running. Enumerate every five-row source/producer/scenario/review mapping and negative case in the PVF inventory. Preserve original19, cloud5, execution2 separately. Run required shared-owner regression/CI only for touched surfaces. A fixture positive does not replace the required current real-producer positive invocation.

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

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Notes / Risks

Execution prerequisites: QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

All four producer prerequisites are accepted and merged: #899 via PR #961 at reviewed head 713776d7dd481b8f7ea06a1a20478be9ef3ae278; #852 via PR #963 at reviewed head a00a3d2286afe5b4b315517874a8b0b9939ed0c9; #900 via PR #973 at reviewed head 2e406b75fbefd4a825dc2700bf5ae4dd668d1f77 and merge b13069dd71d1ccd70083c4018c777f8e56daafd6; #901 via PR #974 at reviewed head 3fb8606a438dcc6e7c112eaaaad01cb1fa6012cc and merge f69019c24a9b61511e912c93f95442f96fa66d92. Hosted CI and aggregate coverage passed at each exact producer head. Authoritative private #900/#901 evidence is retained under .git/csdlc-v3/local/evidence/900/retained and .git/csdlc-v3/local/evidence/901/retained with verified archive hashes. The preserved #851 worktree remains dirty and unreviewed; its changed Runtime paths do not overlap this issue's proposed validator/test paths, and its bytes remain untouched. #931 is the Sprint 5 umbrella. No row may pass when producer execution or exact evidence binding is missing.

Preparation only: implementation, real producer consumption, acceptance proof, implementation review and publication have not run. Existing generic evidence-bundle helpers are historical/shared context, not proof of this criterion-specific consumer. Root main remains inspection-only.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

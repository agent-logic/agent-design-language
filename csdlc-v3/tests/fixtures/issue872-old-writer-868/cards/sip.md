# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0868
Run ID: issue-0868
Version: 0.92.2
Title: [v0.92.2][SIM-02] One current installed command contract
Branch: codex/868-v0922-installed-command-contract
Card Status: ready
Generated: 2026-09-11T23:59:41.750973+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/868
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/868
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
  output_card: .csdlc/issues/868/cards/sor.md
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
- Source issue-prompt slug: v0922-installed-command-contract
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

One current descriptor-backed command contract makes installed help, accepted input schemas, effect classifications, result envelopes, source routing, current documentation and conformance tests agree. Ordinary operational errors cannot silently select historical construction semantics.

Dependency: SIM-01 merged and proven. Coordinates under SIM-UMBRELLA; hands its descriptor, provenance and result contract to SIM-03. This is executable contract reconciliation, not a documentation-only inventory.

## Required Outcome

One current descriptor-backed command contract makes installed help, accepted input schemas, effect classifications, result envelopes, source routing, current documentation and conformance tests agree. Ordinary operational errors cannot silently select historical construction semantics.

Dependency: SIM-01 merged and proven. Coordinates under SIM-UMBRELLA; hands its descriptor, provenance and result contract to SIM-03. This is executable contract reconciliation, not a documentation-only inventory.

## Acceptance Criteria

1. The descriptor covers every frozen current command and explicit alias disposition in the SIM-03 command inventory. Installed candidate help, route recognition, required inputs, effect class and result schema derive from or are exhaustively checked against that single contract. No command disappears through a denominator change.
2. Cover all 26 manifest commands plus guarded `rollback`. Distinguish currently operational routes, read-only helpers, historical proof/construction and administrative routes. Put retained construction/import/simulation under explicit proof/administrative discovery; preserved historical artifacts do not advertise alternate operational authority.
3. Operational authority, context, parse, stale request or discovery failures fail closed. They cannot call a construction-report fallback and present it as successful operational work. An explicit historical/proof invocation remains available only under its declared non-operational contract. Prove the distinction at public installed entrypoints, including genuine non-primary worktrees.
4. A versioned result envelope consistently represents command, issue or explicit issue-not-applicable, status, authority status, issue version, effects, findings and allowed next operations. `blocked`, failed, expected no-op and recovery-required remain distinct; output never reports observation after a mutation. Preserve exit/status semantics and honest partial outcomes.
5. Isolated installation records source revision, build identity and installed digest. Help/schema/result checks invoke that executable rather than a different target-directory binary. Demonstrate mismatch rejection using stale installed-help/provenance and omitted-route fixtures; retain baseline and corrected observations.
6. Extend SIM-01's journey corpus with install/help/schema/prepared-start, draft-to-ready/uncertain-remote response and terminal-readback/result/cleanup cases. Existing typed review, authority, cleanup and publication linkage checks remain enforced. stdout/stderr separation, secret redaction and compatibility logging pass.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][C-SDLC v3][SIM-02] Make the installed command contract consistent and discoverable

## One complete result

One current descriptor-backed command contract makes installed help, accepted input schemas, effect classifications, result envelopes, source routing, current documentation and conformance tests agree. Ordinary operational errors cannot silently select historical construction semantics.

Dependency: SIM-01 merged and proven. Coordinates under SIM-UMBRELLA; hands its descriptor, provenance and result contract to SIM-03. This is executable contract reconciliation, not a documentation-only inventory.

## Current evidence and owned paths

The launch primary installed binary's `--help` omitted `release-preflight`, while `csdlc-v3/src/main.rs` and `docs/csdlc-v3/v3-command-manifest.json` include it. Source dispatches a typed guarded `rollback` alias not listed in root help or the 26-command manifest. These are observed source/installed discrepancies, not proof of a new runtime failure; resolve installed provenance before attributing cause.

Own `csdlc-v3/src/main.rs`, `csdlc-v3/src/commands/mod.rs`, route descriptor/schema glue adjacent to it, `docs/csdlc-v3/v3-command-manifest.json`, relevant current schemas in `docs/csdlc-v3/`, `csdlc-v3/tests/command_manifest.rs`, `csdlc-v3/tests/operational_cli_commands.rs` and `csdlc-v3/tests/proof_parity_install_commands.rs`. Update only contract-relevant guidance in `csdlc-v3/AGENTS.md`, `csdlc-v3/README.md`, and `docs/csdlc-v3/CURRENT_AUTHORITY.md`. Coordinate installation/provenance checks with `adl/tools/install_owner_binaries.sh`; do not replace the live binary. Avoid taking over #861's complete man-page deliverable or #862's decomposition.

## Acceptance

1. The descriptor covers every frozen current command and explicit alias disposition in the SIM-03 command inventory. Installed candidate help, route recognition, required inputs, effect class and result schema derive from or are exhaustively checked against that single contract. No command disappears through a denominator change.
2. Cover all 26 manifest commands plus guarded `rollback`. Distinguish currently operational routes, read-only helpers, historical proof/construction and administrative routes. Put retained construction/import/simulation under explicit proof/administrative discovery; preserved historical artifacts do not advertise alternate operational authority.
3. Operational authority, context, parse, stale request or discovery failures fail closed. They cannot call a construction-report fallback and present it as successful operational work. An explicit historical/proof invocation remains available only under its declared non-operational contract. Prove the distinction at public installed entrypoints, including genuine non-primary worktrees.
4. A versioned result envelope consistently represents command, issue or explicit issue-not-applicable, status, authority status, issue version, effects, findings and allowed next operations. `blocked`, failed, expected no-op and recovery-required remain distinct; output never reports observation after a mutation. Preserve exit/status semantics and honest partial outcomes.
5. Isolated installation records source revision, build identity and installed digest. Help/schema/result checks invoke that executable rather than a different target-directory binary. Demonstrate mismatch rejection using stale installed-help/provenance and omitted-route fixtures; retain baseline and corrected observations.
6. Extend SIM-01's journey corpus with install/help/schema/prepared-start, draft-to-ready/uncertain-remote response and terminal-readback/result/cleanup cases. Existing typed review, authority, cleanup and publication linkage checks remain enforced. stdout/stderr separation, secret redaction and compatibility logging pass.

## Validation and PVF

Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute.

## Exclusions and stop conditions

No replacement intent CLI implementation owned by SIM-03, semantic transaction migration owned by SIM-04, broad manuals/decomposition, historical evidence rewriting or live activation. Stop on authority ambiguity, ownership collision, incompatible contract not explicitly classified, or missing installed proof. Record unresolved source-versus-installed provenance honestly; do not rebuild the active binary to erase baseline evidence.

## Authority and execution boundary

This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes.

This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure.

## Source contract

- `docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md`
- `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`
- `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`
- `csdlc-v3/AGENTS.md` and root `AGENTS.md`

Launch inspection baseline: `ace209ad9c701a855d165da817164ff60a755203`. This is source inspection, not execution proof or a promise that defects survive to the implementation baseline. Retain source/candidate revisions, installed binary digest/provenance and clean fixture identity with every run.


## Canonical sprint links

Planning and issue-creation owner: #864.
Sprint umbrella: #866 (coordination; does not gate SIM-01 startup).
Execution prerequisite: #867 (SIM-01), with accepted merged output before dependent execution.

Creation contract reviewed at `ed2a93338c92fda62ba63761d56af21b50483eb4`. Issue creation is not implementation start or live activation.

## Target Files / Surfaces

The launch primary installed binary's `--help` omitted `release-preflight`, while `csdlc-v3/src/main.rs` and `docs/csdlc-v3/v3-command-manifest.json` include it. Source dispatches a typed guarded `rollback` alias not listed in root help or the 26-command manifest. These are observed source/installed discrepancies, not proof of a new runtime failure; resolve installed provenance before attributing cause.

Own `csdlc-v3/src/main.rs`, `csdlc-v3/src/commands/mod.rs`, route descriptor/schema glue adjacent to it, `docs/csdlc-v3/v3-command-manifest.json`, relevant current schemas in `docs/csdlc-v3/`, `csdlc-v3/tests/command_manifest.rs`, `csdlc-v3/tests/operational_cli_commands.rs` and `csdlc-v3/tests/proof_parity_install_commands.rs`. Update only contract-relevant guidance in `csdlc-v3/AGENTS.md`, `csdlc-v3/README.md`, and `docs/csdlc-v3/CURRENT_AUTHORITY.md`. Coordinate installation/provenance checks with `adl/tools/install_owner_binaries.sh`; do not replace the live binary. Avoid taking over #861's complete man-page deliverable or #862's decomposition.

## Validation Plan

Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute.

Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient):
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands`
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`
- `git diff --check`
Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Demo / Proof Requirements

Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute.

Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient):
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands`
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

No replacement intent CLI implementation owned by SIM-03, semantic transaction migration owned by SIM-04, broad manuals/decomposition, historical evidence rewriting or live activation. Stop on authority ambiguity, ownership collision, incompatible contract not explicitly classified, or missing installed proof. Record unresolved source-versus-installed provenance honestly; do not rebuild the active binary to erase baseline evidence.

## Notes / Risks

This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes.

This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure.

Wait for accepted merged output of #867; then refresh this plan against that exact source revision and re-resolve path ownership before binding.

Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.

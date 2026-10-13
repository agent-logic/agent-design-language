# Refactor Plan: D-MIXED-ADL manifest, portable library and legacy CLI boundary

## Refactor Plan Summary

- Status: partial
- Run id: issue1223-public-cli-partition
- Planned slices: 4

## Scope

- Scope: D-MIXED-ADL manifest, portable library and legacy CLI boundary
- Mode: plan_refactor
- Target paths:
- adl/Cargo.toml
- adl/src/lib.rs
- adl/src/main.rs
- adl/src/cli/mod.rs
- adl/src/cli/commands.rs
- adl/src/cli/run.rs
- adl/tools/install_owner_binaries.sh
- .github/workflows/ci.yaml

## Current Behavior

The adl manifest builds public adl plus private Runtime, CodeFriend and operational binaries. lib.rs unconditionally exports private modules. cli/mod.rs dispatches portable and product commands; default YAML handling calls run_workflow. cli/run.rs imports execute/provider-backed modules even for print-plan and print-prompts. The successor Runtime already exposes adl-runtime run <yaml>, resume, keygen, sign, verify and other product commands. That command correspondence is not authenticated consumer qualification. Public five-package closure is independent and cannot prove this CLI. Source baseline cc5cc6788a and Runtime 11ce6614603532738e9ceffab093e35d96b262b5.

## Refactor Intent

Preserve public command semantics while separating portable planning from private execution and maintaining one implementation owner. Do not remove commands, silently change default dispatch, or treat source equality as execution qualification.

## Invariants

- Existing YAML operands, print-plan/print-prompts output, plan ordering and errors remain stable.
- Workflow run/resume/sign/verify preserve artifact formats, exit codes, cwd, argument boundaries and stdout/stderr separation.
- No shell-expanded dispatch, implicit package install, provider calls during characterization, or arbitrary PATH execution.
- Five public packages retain their isolated private-free dependency closure.
- Current native C-SDLC source, receipts and installer remain usable; cognitive-sdlc#100 is not silently added as a prerequisite.
- Original 4134 identities and rollback history remain immutable.

## Risk Inventory

- R-001: Existing CLI tests mix pure public cases with private Runtime scenarios; a name-only filter is insufficient. Mitigation: Address or validate before the affected slice lands.
- R-002: Runtime entrypoint exists but authenticated ADL installed-consumer discovery and argument/output parity are not yet proven. Mitigation: Address or validate before the affected slice lands.
- R-003: CodeFriend command compatibility is separate from Runtime command compatibility. Mitigation: Address or validate before the affected slice lands.
- R-004: Default installer still lists private binaries and hashes local source; dropping manifest bins alone breaks it. Mitigation: Address or validate before the affected slice lands.

## Refactor Slices

### CLI-01: Separate portable planning from effectful execution

- Intent: Extract existing argument interpretation and pure plan/prompt rendering from run.rs; use adl-legacy-contracts for portable declarations. Keep the current execution branch calling its existing implementation until the consumer adapter is proven. Do not route print-plan through an installed private Runtime.
- Behavior change: false
- Target files: adl/src/cli/run.rs, adl/src/cli/commands.rs, adl/src/cli/mod.rs
- Invariants: Existing YAML operands, print-plan/print-prompts output, plan ordering and errors remain stable., Workflow run/resume/sign/verify preserve artifact formats, exit codes, cwd, argument boundaries and stdout/stderr separation., No shell-expanded dispatch, implicit package install, provider calls during characterization, or arbitrary PATH execution., Five public packages retain their isolated private-free dependency closure., Current native C-SDLC source, receipts and installer remain usable; cognitive-sdlc#100 is not silently added as a prerequisite., Original 4134 identities and rollback history remain immutable.
- Validation commands: Characterize current and candidate YAML plan/prompt bytes, option combinations, error status and stdout/stderr separation using synthetic fixtures., Run selected existing cli_smoke public cases.
- Rollback notes: Retain base commit cc5cc6788a and original preservation archive; revert only this scoped source delta, never semantic state or published effects.
- Residual risk: Overlay and resume options share parsing with pure planning; classify these before extraction.
- Follow-up: CLI-02

### CLI-02: Qualify product command adapters before unlinking implementations

- Intent: Use the already present successor adl-runtime run/resume/keygen/sign/verify entrypoints through explicit authenticated installed-package selection; establish each retained legacy command mapping and exact behavior. Keep CodeFriend and lifecycle commands separately classified. No PATH fallback or silent installation.
- Behavior change: false
- Target files: adl/src/cli/mod.rs, adl/src/cli/run.rs, adl/src/cli/commands.rs
- Invariants: Existing YAML operands, print-plan/print-prompts output, plan ordering and errors remain stable., Workflow run/resume/sign/verify preserve artifact formats, exit codes, cwd, argument boundaries and stdout/stderr separation., No shell-expanded dispatch, implicit package install, provider calls during characterization, or arbitrary PATH execution., Five public packages retain their isolated private-free dependency closure., Current native C-SDLC source, receipts and installer remain usable; cognitive-sdlc#100 is not silently added as a prerequisite., Original 4134 identities and rollback history remain immutable.
- Validation commands: Real pinned Runtime run/resume/sign/verify with synthetic local state, exact stdout/stderr/exit and argument forwarding comparison., Missing package, invalid manifest, wrong repository, metacharacter operand and failure propagation negatives.
- Rollback notes: Retain base commit cc5cc6788a and original preservation archive; revert only this scoped source delta, never semantic state or published effects.
- Residual risk: Installed-owner consumer admission is not yet established by the successful independent producer build.
- Follow-up: CLI-03

### CLI-03: Cut the public manifest and installer together

- Intent: After adapter parity, remove private binary declarations, module exports and dependencies from public ADL; keep portable compatibility facade. Change default owner installation and CI to public-owned targets and explicit installed product consumers. Preserve C-SDLC source-dependent installer until its supported replacement is independently qualified.
- Behavior change: false
- Target files: adl/Cargo.toml, adl/Cargo.lock, adl/src/lib.rs, adl/src/main.rs, adl/tools/install_owner_binaries.sh, .github/workflows/ci.yaml
- Invariants: Existing YAML operands, print-plan/print-prompts output, plan ordering and errors remain stable., Workflow run/resume/sign/verify preserve artifact formats, exit codes, cwd, argument boundaries and stdout/stderr separation., No shell-expanded dispatch, implicit package install, provider calls during characterization, or arbitrary PATH execution., Five public packages retain their isolated private-free dependency closure., Current native C-SDLC source, receipts and installer remain usable; cognitive-sdlc#100 is not silently added as a prerequisite., Original 4134 identities and rollback history remain immutable.
- Validation commands: cargo metadata --locked --offline --manifest-path adl/Cargo.toml --no-deps, Build retained public CLI without private source directories and run public characterization cases., Validate installer selected-binary and CI-selector contracts with no package installation.
- Rollback notes: Retain base commit cc5cc6788a and original preservation archive; revert only this scoped source delta, never semantic state or published effects.
- Residual risk: Compile feature gates alone do not retire still-referenced private implementations.
- Follow-up: CLI-04

### CLI-04: Retire only unreachable superseded components

- Intent: Use final dependency and consumer checks to enumerate exact whole-component deletions and corresponding original inventory IDs; include tools/tests/CI/demo references and qualify changed retained boundaries. Preserve original history and obtain exact scoped retirement decision.
- Behavior change: false
- Target files: docs/milestones/v0.93.1/repository-decomposition/rd07-1223/DISPOSITION_REGISTER.json, tools/source_retirement, adl/src, adl/tests, adl-runtime, adl-runtime-kernel, adl-provider-core
- Invariants: Existing YAML operands, print-plan/print-prompts output, plan ordering and errors remain stable., Workflow run/resume/sign/verify preserve artifact formats, exit codes, cwd, argument boundaries and stdout/stderr separation., No shell-expanded dispatch, implicit package install, provider calls during characterization, or arbitrary PATH execution., Five public packages retain their isolated private-free dependency closure., Current native C-SDLC source, receipts and installer remain usable; cognitive-sdlc#100 is not silently added as a prerequisite., Original 4134 identities and rollback history remain immutable.
- Validation commands: Exact inventory and reference negatives with reviewed removal manifest., Five final public packages, seven consumers, 21 UTS fixtures, five installed negatives; retained public CLI proof separately.
- Rollback notes: Retain base commit cc5cc6788a and original preservation archive; revert only this scoped source delta, never semantic state or published effects.
- Residual risk: No deletion by directory prefix or matching bytes; final source set is not yet determined.
- Follow-up: none


## Validation Plan

- Compare exact baseline/candidate stdout, stderr and status for synthetic public YAML planning and prompt cases; establish selected cases before running.
- Run retained CLI smoke cases on a private-free public candidate; do not substitute package tests.
- Verify real pinned Runtime consumer run/resume/sign/verify with local synthetic fixtures, including missing/wrong artifact, metacharacter argument and nonzero exit propagation negatives.
- cargo metadata --locked --offline --manifest-path adl/Cargo.toml --no-deps
- Final five-package proof remains required once at final committed candidate.

## Rollback Notes

- Revert this slice independently if validation for adl/Cargo.toml fails.
- Revert this slice independently if validation for adl/src/lib.rs fails.
- Revert this slice independently if validation for adl/src/main.rs fails.
- Revert this slice independently if validation for adl/src/cli/mod.rs fails.
- Revert this slice independently if validation for adl/src/cli/commands.rs fails.

## Residual Risk

- No legacy CLI retirement approval exists. Preserve it.
- Public argument/output characterization and authenticated Runtime/CodeFriend consumer entrypoint parity remain unproven.
- Plan does not authorize removal of additional inventory rows.

## Stop Boundary

- Performed refactor: false.
- Changed behavior: false.
- Created issues: false.
- Created PRs: false.
- Committed changes: false.
- Mutated repository: false.

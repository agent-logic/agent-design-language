# Structured Review Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/630/create-request.json
.csdlc/prepared/issues/630/design.md
.csdlc/prepared/issues/630/diagram.mmd
.csdlc/prepared/issues/630/finalize-request.json
.csdlc/prepared/issues/630/recover-review-after-cleanup-denial.json
.csdlc/prepared/issues/630/recover-review-after-sealed-readback.json
.csdlc/prepared/issues/630/recover-review-after-symlink-parent-escape.json
.csdlc/prepared/issues/630/repair-spp-affected-areas.json
.csdlc/prepared/issues/630/repair-vpp-lanes.json
.csdlc/prepared/issues/630/validate-v3-h4-terminal-clean-cutover.sh
.csdlc/evidence/630/630-diff-hygiene.log
.csdlc/evidence/630/630-full-v3-regression.log
.csdlc/evidence/630/630-issue-validator.log
.csdlc/evidence/630/630-rustfmt.log
.csdlc/evidence/630/630-terminal-clean-cutover-tests.log
.csdlc/evidence/630/630-typed-issue-validation.log
.csdlc/evidence/630/issue-630-terminal-clean-cutover-validation.log

## Prompts

- Can any caller manufacture terminal, cleanup, or cutover authority without authenticated evidence?
- Does cleanup actually consult Git worktree registration and preserve distinct outcomes?
- Does cutover remain a non-authoritative decision packet before #505?
- Are all tests and validators exercising behavior instead of strings?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 finish, clean, and cutover remain construction-only until explicit V3-F/#505 cutover; no live terminal authority is granted by this issue.

## Review Result

Revision: Some("git-blake3:8933a5e2f4012f5e8b4568c39124ab927bc092be:20408bc08e7d250e23a82d640579495f04be25efa380cdc0c450c8d74b78bc45")

Reviewer: Some("subagent:/root/review_630_common_dir_final")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/terminal/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
csdlc-v3/tests/command_manifest.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/630/design.md
.csdlc/prepared/issues/630/diagram.mmd
.csdlc/prepared/issues/630/validate-v3-h4-terminal-clean-cutover.sh

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

- C-SDLC v3 terminal, cleanup, and cutover routes remain construction-only and non-authoritative until explicit #505 cutover.

## Review Result

Revision: Some("git-blake3:05b17cc83f9e17b7adcfcf22f05498f579508f74:a39833aed2d177f872f73a92afd17c6c5495f594ffd0e6104f116cbbe11f14f6")

Reviewer: Some("codex-reviewer:review_630_fast")

Result: pass

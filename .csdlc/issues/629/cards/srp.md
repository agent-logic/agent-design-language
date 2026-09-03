# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/629/design.md
.csdlc/prepared/issues/629/diagram.mmd
.csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh

## Prompts

- Verify remote authority is not caller-forgeable.
- Verify publication refuses stale or missing review truth.
- Verify closing publications produce visible and typed Closes #xxx linkage.
- Verify credentials are redacted and no raw gh lifecycle writes are used.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remote/publication routes remain construction-only and non-authoritative until explicit #505 cutover.
- Retained #629 evidence logs are stale relative to this exact head and must not be used as exact-head proof without fresh validation.

## Review Result

Revision: Some("git-blake3:8aa002b74432363edd49181e7953f21f3b0d0438:4505e1e6439d828e638d3ad35bc587a3471fb8cded749f104d44d2d6fd57d2a4")

Reviewer: Some("codex-reviewer:review_629_fast")

Result: pass

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
- The #629 canary is intentionally tolerant of implemented, reviewed, or published pre-terminal lifecycle phases so lifecycle metadata publication does not make exact-head source proof brittle.

## Review Result

Revision: Some("git-blake3:c9728373330f5ae6dbebbab26b0970bda0a310af:526398bedede01b4a1cc9bcb7c1d400d07c18137e9fcc6cd610bb2cd5573124e")

Reviewer: Some("codex-reviewer:review_629_c972_micro")

Result: pass

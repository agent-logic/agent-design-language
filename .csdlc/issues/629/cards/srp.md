# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/real_issue_canary.rs
csdlc-v3/tests/transactions.rs

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

- C-SDLC v3 GitHub and publication routes remain non-authoritative until explicit V3-F/#505 cutover.
- Live authenticated-success --observe-github was not run with a real token; fake-token dispatch proof verifies adapter shape and fail-closed behavior without exposing credentials.
- V3 PR create/update writes remain incomplete; typed C-SDLC v2 remains publication authority until #505.

## Review Result

Revision: Some("git-blake3:3fa957d36cfe3df2b125f97f63615c88a6d2ffdb:f9649a215b5f34ff2d7f3f945953a6b2a564b6f2f3fba657e6b2e7319cf7213d")

Reviewer: Some("codex-reviewer:review_629_head_3fa957")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/remote_publication_commands.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/629

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

- Public remote route planning intentionally fails closed until authenticated GitHub adapter and typed review receipt ingestion are implemented.

## Review Result

Revision: Some("git-blake3:eb51488dee5c491653f24c8f521b8033b12e4e12:74c1b2417ca959edb117d37d30029845ee76511ac4723db1814ef25ce820a44f")

Reviewer: Some("codex-reviewer:review_629_r3_exact_head")

Result: pass

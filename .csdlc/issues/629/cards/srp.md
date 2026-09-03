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

- The public remote routes remain construction-only and fail-closed until authenticated adapter/receipt ingestion is implemented and #505 explicitly cuts over authority.
- This review does not authorize v3 publication, finish, cleanup, or GitHub mutation as live operational authority before #505.

## Review Result

Revision: Some("git-blake3:c096824f578007b965f0bb0bb60a2ac4ae35aa1e:6803a98b5df3c975f8fd1c0823a0767aa11e2872e0cb6a1b1ce75b4ebfa7fd93")

Reviewer: Some("subagent:/root/review_629_head_c096824f5")

Result: pass

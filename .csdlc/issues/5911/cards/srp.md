# Structured Review Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.adl/worktree-policy.json
AGENTS.md
adl/tools/archive_codex_sessions_to_fastwork.sh
adl/tools/test_archive_codex_sessions_to_fastwork.sh
csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/code_repository_migration.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate4.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Check path containment and canonicalization bypasses.
- Check that rejection occurs before Git mutation.
- Check archive privacy, completeness, and checksum verification.
- Confirm no deletion or existing-worktree relocation occurred.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Local transcript deletion remains deliberately unperformed and requires separate explicit operator approval.
- Existing legacy worktrees outside the canonical parent are not relocated by this issue.
- The local shared Git common directory contains unrelated retained terminal-state drift; exact clean hosted C-SDLC v2 validation is the authoritative full-suite proof.

## Review Result

Revision: Some("git-blake3:d23ec6c17e013bbd16e2bb35978ac2c6714726be:9cf3f477c5736cf2565b4ca2f0648521ed8efa00bf0e6db75dcd0b77e5687bcb")

Reviewer: Some("codex-subagent-issue-5911-review")

Result: pass

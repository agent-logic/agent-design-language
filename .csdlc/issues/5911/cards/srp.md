# Structured Review Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.adl/worktree-policy.json
.csdlc/issues/5911
.csdlc/prepared/issues/5911/design.md
.csdlc/prepared/issues/5911/design.mmd
AGENTS.md
adl/tools/archive_codex_sessions_to_fastwork.sh
adl/tools/test_archive_codex_sessions_to_fastwork.sh
csdlc-v2/src/lifecycle.rs

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

## Review Result

Revision: Some("git-blake3:1a2902545b4392ae28b767959d92f6da5215e1b4:0486381b1cd16436f889ab59a2d8e14d7063ffc9d8c720d90ea00a1214e4a1a4")

Reviewer: Some("codex-subagent-issue-5911-review")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/lib.rs
csdlc-v2/src/projection_cleanup.rs
csdlc-v2/tests/archived_projection_cleanup.rs
.csdlc/issues/299
.csdlc/evidence/299

## Prompts

- Can cleanup start without exact terminal+ancestral #298 recovery authority or a matching completed recovery/canonical/archive binding?
- Can any path, digest, symlink, recursive walk, or unrecorded inode authorize deletion?
- Do regular files, empty directories, root placeholder, tombstones, and disposal counterparts have explicit type-correct pre/post exchange and removal receipts?
- Does every restart boundary adopt only exact receipt-owned identities and preserve ambiguous, replaced, unsupported, non-empty, or third states?
- Do immutable cleanup receipts and #298 recovery evidence survive successful cleanup?
- Do unrelated sentinels and replacement inodes survive all success, failure, and race cases?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only exact-head inspection and did not rerun validation; it verified the owner-recorded exact-head focused, full hosted-geometry, strict Clippy, fmt, diff-check evidence and SOR hash truth.
- The live worktree HEAD c98d4d7653dc9bbaf146cda997b3bca353bffb2b contains typed assignment metadata only; substantive source/test review was against aa717ba8043b344ab8f83fbea7ba325009416094.
- Earlier r15 gate_github_actions RED remains historical/classified issue truth and is not itself claimed as a PASS; current resynced broad hosted-geometry evidence at 586b24441513f8062b9495eac4fdc70e0b9e9929 is PASS.

## Review Result

Revision: Some("git-blake3:aa717ba8043b344ab8f83fbea7ba325009416094:e9d3431dac0c7cdf97dc2ca39d03e7df7c96d5c19a342c60c8d007b941a32e84")

Reviewer: Some("fresh-session:30dd7666-9512-486d-91b4-c7fe6036e8f2")

Result: pass

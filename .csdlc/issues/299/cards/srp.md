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

- Review was inspection-only and did not rerun validation; it verified the owner-recorded exact-head focused, strict Clippy, fmt, and diff evidence plus SOR truth.
- Prior serial full-suite RED in gate_github_actions remains preserved and is not converted to PASS by focused #299 proof.

## Review Result

Revision: Some("git-blake3:203c5aa4b10abd00645a2f4c2b3250596c00c7ec:555a95f593bce60087da1ca698c742220a7e4f2370e07c17d6e536568dec1b57")

Reviewer: Some("fresh-session:2d97c7db-3bf8-4613-85ec-4428272d6ad3")

Result: pass

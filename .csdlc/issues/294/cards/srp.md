# Structured Review Prompt

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/store.rs
csdlc-v2/tests/card_identity.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/294
.csdlc/evidence/294

## Prompts

- Review every acceptance criterion with special attention to lifecycle authority, path traversal/symlink safety, append-only audit integrity, atomicity, and linked-worktree regression proof.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer observed retained exact-head evidence logs from predecessor source head for older validation lanes; reviewer reran card_identity at 58f99146506b8999ad0170f3b4d3cc445bd93688 and treated file-backed PR #385 gate5/gate2/check/clippy logs as the current remediation proof.

## Review Result

Revision: Some("git-blake3:9afd91ce84a28bd118299cf3b12e840611f55fe3:89c2642008f9e0d129b8b4beba0303f769528c1d66f71aefa3a73c9c215d18ab")

Reviewer: Some("fresh-session:fd57172e-204f-483a-ad29-9faa4f7c7fad")

Result: pass

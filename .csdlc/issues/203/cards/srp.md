# Structured Review Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/203
.csdlc/prepared/issues/203
.csdlc/evidence/203

## Prompts

- Review every #203 acceptance criterion AC-1 through AC-8 against canonical terminal #258/#259/#260 cache and merge ancestry, zero adl-runtime and Cargo.lock diff versus exact origin/main, current 4-test identity-boundary proof, current 5-test authority-caller guard, strict Clippy, and immutable v3 receipt evidence.
- Report findings first as P0-P3 with repo-relative file and line evidence; verify typed generation, digest, reviewer identity, immutable git-blake3 revision, and assigned scope before reaching a verdict.
- Verify the historical 44-case/132-subassertion receipt is retained only as superseded historical evidence and is never used as current proof, and verify no #202, #204, #205, #208, #258, #259, or #260 implementation is absorbed by #203.
- State all validation limitations explicitly, remain read-only, and return PASS only when there are no actionable findings; otherwise return FAIL with exact remediation required.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Retained Cargo and Clippy logs were digest-verified rather than independently rerun by reviewer.
- Broad workspace and hosted CI remain publication gates.

## Review Result

Revision: Some("git-blake3:50c566fd7968eb86ce12ffe7dd0eac16be29b92b:280850b2594d0048c6517a404c43d076214d4b19dc65428950667315882287fe")

Reviewer: Some("fresh-session:64ce444e-ef62-40e9-9b21-cf396db07f0b")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/src/publication.rs
csdlc-v2/tests/publication_tail.rs
.csdlc/issues/306
.csdlc/prepared/issues/306

## Prompts

- publication ordering and metadata tail safety
- exact-clean finish preservation
- retry and interruption determinism, including interrupted-after-intent, interrupted-after-push, and interrupted-after-record
- no broad untracked metadata allowlist
- Sprint 6 finish-blocking map

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused exact-head review only; reviewer ran cargo test --manifest-path csdlc-v2/Cargo.toml --test publication_tail with 7 passed, but did not run the full Rust suite.
- Reviewer did not perform GitHub writes, live PR mutation, lifecycle writes, or merge; publication remains to be performed through typed csdlc-publish.

## Review Result

Revision: Some("git-blake3:dd1ff74299d542a12ee99c2de49fa57bba6230f7:0a3cc0dc827c52484959c1ce440b1d2bfde1f039856c3b4515232fec3a9fc215")

Reviewer: Some("fresh-session:3d7416dd-bcc2-4f71-ad12-c1323f85cbc0")

Result: pass

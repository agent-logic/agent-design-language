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

- Focused exact-head review only; reviewer ran cargo test --manifest-path csdlc-v2/Cargo.toml --test publication_tail with 6 passed, but did not run the full Rust suite.
- Reviewer did not perform GitHub writes, live PR mutation, or lifecycle writes; publication remains to be performed through typed csdlc-publish.

## Review Result

Revision: Some("git-blake3:e7a846174f5bd7fffbff3993d7bd7f1305c58bbf:9224484b4b2a0e44c61bc4067731f48b5e820394702edae35b6706ad25bd8802")

Reviewer: Some("fresh-session:3a77f416-cd8d-49f2-bbc7-22e7baae54f6")

Result: pass

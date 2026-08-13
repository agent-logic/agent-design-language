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

- Focused exact-head review only; local validation covered cargo test --manifest-path csdlc-v2/Cargo.toml --test publication_tail with 8 passed and strict clippy for publication_tail; full Rust suite deferred.
- Reviewer did not perform GitHub writes, live PR mutation, lifecycle writes, CI observation, or merge; publication remains to be performed through typed csdlc-publish.

## Review Result

Revision: Some("git-blake3:308e65651b2812a836db1be53b4a5936d098ae24:486b5385fb23cb41e8d090f0486e5944ba4bb58132b08f948fcea204b47642cb")

Reviewer: Some("fresh-session:8749ae45-5b88-4483-acba-13d50c93d9c7")

Result: pass

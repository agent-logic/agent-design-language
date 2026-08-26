# Structured Review Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/116
.csdlc/prepared/issues/116
.csdlc/evidence/116
adl-runtime-kernel/src/operator_attention.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/observatory.rs
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/styles.css
demos/html-observatory/tests/operator_attention_inbox.test.mjs

## Prompts

- Does the design prevent fabricated source identity, urgency, and authority?
- Are rate limits, grouping, deduplication, quiet modes, and retention explicit enough to prevent flooding?
- Do operator response states avoid implicit approval?
- Does #116 remain separate from #117 and downstream proof children?
- Are focused validation lanes truthful and bounded?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer confirmed current HEAD 67d89c3ac91dac833357e666147a2d359d9fe328 differs from assigned substantive revision only by review-assignment metadata.
- Reviewer reran the focused #116 local proof: preparation validator, rustfmt check, 8 Rust operator_attention tests, 1 Node Observatory inbox test, strict clippy, and git diff --check all passed.
- GitHub CI remains the remote integration gate after typed publication.

## Review Result

Revision: Some("git-blake3:c4d5f42f9ebd6ee89571d364801948a4e7f03e6f:baafc85716d4cbe62985b050416a2ad69ea6998bf114cc439cd58d4d0d0c74b5")

Reviewer: Some("fresh-session:d1e3ad16-762a-4a7f-88cd-3e4ec49bcb8a")

Result: pass

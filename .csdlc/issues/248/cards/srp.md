# Structured Review Prompt

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/parity.rs
adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
adl-runtime-kernel/tests/parity.rs
.csdlc/issues/248
.csdlc/prepared/issues/248
.csdlc/evidence/248

## Prompts

- Is precedence derived from observable server-owned state rather than timing?
- Can either terminal path leave an output artifact or live descendant?
- Are ordinary timeout, output-limit, cancellation, and cleanup semantics preserved?
- Does the diff avoid #244 and #112 surfaces?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:5f63a68189e1ca23406f34ac3da6549f4d39aea5:a4bd706a71e09cb206c2c0c0a65e00669cb0c1c9120695c4ded9bcd383ad3289")

Reviewer: Some("codex-subagent:rereview_248_process_precedence")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/cli/mod.rs
adl/tests/issue_327_removed_tooling.rs
.csdlc/issues/327

## Prompts

- Is removing real_tooling behavior-preserving given all current dispatch call sites?
- Does any v1 tooling route or authority return?
- Are focused and strict-Clippy proofs sufficient for the one-line deletion?
- Did the change avoid every #259 surface?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Inspection-only review; local tests were not rerun. Prior focused regression, strict Clippy, and hosted required CI were green before metadata-only review recovery.

## Review Result

Revision: Some("git-blake3:4cd334aedd52473bcde6d32759a83647958744b8:0d6532f6b5d18758f9350a7ed577cf8fef61ac5c6cd7c3fcff5d214ef171720b")

Reviewer: Some("fresh-session:7a81c032-1c4d-4e2f-a6b9-93d8e75f2041")

Result: pass

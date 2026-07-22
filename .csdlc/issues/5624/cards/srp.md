# Structured Review Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_cargo_validation.sh
adl/tools/test_run_cargo_validation.sh
csdlc-v2/src/operator.rs
csdlc-v2/src/proof.rs
csdlc-v2/src/readiness.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate7.rs
csdlc-v2/tests/gate7_lifecycle.rs
.csdlc/issues/5624
.csdlc/prepared/issues/5624
.csdlc/evidence/5624

## Prompts

- Can `.` validate any checkout other than the exact current terminal branch worktree?
- Can two repositories with the same relative worktree suffix collide?
- Do malformed, missing, wrong, and dirty candidates fail with unsafe_checkout?
- Can validation alter the terminal record or retained receipt?
- Does command-level proof exercise the same path as issue 5340?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:1be31976a945dd0b157e434d585d63d6ea732c7d:1bb7bcaa3d1b0a6edbff72e7d1d90b92efb6946d3aacd1be7f508aacd0c7c181")

Reviewer: Some("subagent:codex-5624-final-ci-review")

Result: pass

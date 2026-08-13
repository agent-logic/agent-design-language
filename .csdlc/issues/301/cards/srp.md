# Structured Review Prompt

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/301
.csdlc/prepared/issues/301
.csdlc/evidence/301
csdlc-v2/src/github.rs
csdlc-v2/tests/gate_github_actions.rs

## Prompts

- body byte preservation
- operation marker durability
- retry and conflict behavior
- readback reconciliation truth

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only and did not rerun cargo, csdlc-validate, CI, lifecycle, or GitHub state checks.
- Retained .csdlc/evidence/301/title-only-github-issue-update.log is older 9-test evidence; generation-14 SOR records post-merge validation as local-command evidence for 10 passing tests.
- Untracked operational files .csdlc/locks/301.lock and .csdlc/publication/301.intent.json were present outside assigned review scope and were preserved.

## Review Result

Revision: Some("git-blake3:1e5d65a5bdc29345397fed47a2a3ee64794d6254:9a00c166234e5e49b0b02ca9942564651926eb3c0f9a3f6079b8272a0a4957d0")

Reviewer: Some("fresh-session:5905b3b9-4ad3-4311-ad06-4426f171f438")

Result: pass

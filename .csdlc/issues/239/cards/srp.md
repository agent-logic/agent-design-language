# Structured Review Prompt

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/finish.rs
csdlc-v2/src/bin/csdlc-finish.rs
csdlc-v2/src/cleanup.rs
csdlc-v2/tests/gate_finish.rs
.csdlc/evidence/239/

## Prompts

- Does the fix reuse or exactly preserve the existing metadata-only policy?
- Can substantive source drift validate?
- Does the regression reproduce publication revision before the terminal metadata-only head?
- Are canonical issue, generation, digest, repository, PR, and head checks unchanged?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:a89ff0333f4a21844dd0e3504d1bf338908038ac:fdbea6d5a65a9bc094ad33e6a28887d8bf3922efab1d355fe7d865f0f2872b84")

Reviewer: Some("/root/review_239_exact_head")

Result: pass

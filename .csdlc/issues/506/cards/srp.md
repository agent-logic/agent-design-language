# Structured Review Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/506/design.md
.csdlc/prepared/issues/506/diagram.mmd
adl-runtime/src/lib.rs
adl-runtime/src/qualification/mod.rs
adl-runtime/tests/distributed_contract/main.rs
adl-runtime/tests/distributed_contract/validate_drt_a.sh
docs/milestones/v0.92.1/evidence/runtime/drt-a/qualification-contract.json

## Prompts

- Verify that #506 owns exactly DRT-A and does not absorb paid AWS/GCP execution, Observatory redesign, DRT-B, DRT-C, DRT-D, provider credentials, or public cloud exposure.
- Verify that the design maps requirements 181 and 182 and includes all four WP-specified PVF lanes.
- Verify that the planned proof denominator includes identity, authority, duplicate-denial, replay, and negative-matrix behavior.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Read-only exact-head review only; no broad, paid, cloud, or live distributed tests were run or claimed.
- Post-assignment generated review metadata dirt was outside the assigned product scope.

## Review Result

Revision: Some("git-blake3:dfa63786b98246d9eb0e919e03aa6508646d87bd:16d0be8f61a078adbcc6f80ae58cf69024a2a47fa34b80d6add75956a25da5ee")

Reviewer: Some("fresh-session:7a339974-b89c-44cd-94d3-ff7d9320c4ae")

Result: pass

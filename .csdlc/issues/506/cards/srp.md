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

- Read-only exact-head review only; no live cloud/provider proof was run or claimed.
- Post-assignment generated review metadata dirt was outside the assigned product scope.

## Review Result

Revision: Some("git-blake3:1e8d9773cbc749a3e88d7217b34cc7e79e3b0210:0c8e522add6f65fcb35a7e846696edf276360dd88435c782916c14ef64f2c0e2")

Reviewer: Some("fresh-session:459cc47f-ffff-4d3a-981e-6d2f329c8456")

Result: pass

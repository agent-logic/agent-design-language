# Structured Review Prompt

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/serving_authority.rs
adl-runtime/src/distributed/shepherd_serving_eligibility.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
adl/tools/check_coverage_impact.sh
adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_run_pr_fast_coverage_lane.sh
.csdlc/issues/273
.csdlc/prepared/issues/273
.csdlc/evidence/273

## Prompts

- Are #273 and #274 production modules/tests disjoint, with only one explicitly serialized registration line?
- Can any caller become eligible without the exact current published #272 binding?
- Can replacement, retry, restart, revoke, or expiry expose two owners or revive stale authority?
- Do receipts and projection remain exact and redacted?
- Are all predecessor cache/ancestry, validation, review, CI, and terminal gates explicit?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The focused coverage route intentionally enables internal-test-fixtures only for the exact Shepherd eligibility selector; ordinary builds exclude the fixture constructor.
- #274 remains unbound and serialized behind terminal and ancestral #273 before consuming the verified cut or touching shared registration.

## Review Result

Revision: Some("git-blake3:eca7071874c9e93914ac4c142cd667dcac9a3929:54c7c21be952c812f3040ceead21cb8ded3da4b9f3de9371396b3e9a3f217c3b")

Reviewer: Some("fresh-session:51bfaaca-42c2-4aa0-85a4-e6e9d44d9d53")

Result: pass

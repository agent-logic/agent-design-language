# Structured Review Prompt

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.adl
.csdlc/evidence/432
.csdlc/issues/432
.csdlc/requests
AGENTS.md
adl/config/worktree-policy.json
adl/tools/batched_checks.sh
adl/tools/check_no_tracked_adl.sh
adl/tools/test_check_no_tracked_adl.sh
csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Can any active path still treat .adl as authority?
- Does policy relocation preserve exact behavior?
- Are historical mentions excluded without becoming fallback authority?
- Was any sensitive local material promoted?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub CI and publication state were not part of the local exact-head review and remain publication-stage evidence.

## Review Result

Revision: Some("git-blake3:7726557167b9793cf186201d47dddfb7b421c961:f430805caadac4c8961ffd92de9cd40189f8e63bf741125b63d67aa74cdd0298")

Reviewer: Some("fresh-subagent:rereview-432-r2")

Result: pass

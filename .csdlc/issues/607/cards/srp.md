# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

infra/aws/runtime/gpu-proof
adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/build_or_prepare_issue607_warm_polis.sh
adl/tools/run_issue607_warm_polis.sh
adl/tools/issue607_probe_runtime.py
adl/tools/issue607_qualify_warm_polis.sh
adl/tools/issue607_guardian_recovery_proof.sh
adl/tools/issue607_validate_saved_plan.sh
adl/tools/test_issue607_warm_polis.sh
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
docs/operations/cloud/aws/shepherd-gpu-proof
.csdlc/prepared/issues/607/validate-preparation.sh
.csdlc/prepared/issues/607
.csdlc/evidence/607
.csdlc/issues/607

## Prompts

- Can normal launch reach any compiler package manager Git mutable download or model pull path?
- Can Terraform destroy or a trap delete the persistent warm volumes?
- Are timing denominators complete and comparable?
- Can stale or cross-AZ volume content activate?
- Are #605 SSH private-Ollama IAM and cleanup invariants preserved?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live AWS proof remains the final validation lane and must stay within the exact USD 20 reservation.

## Review Result

Revision: Some("git-blake3:5eb4de0d7a128e566975d078bdfaa275a70f1600:7d50e42115fd4476a8331b42da6069b6f9a0665dc6cdbd4d8c86543cd777fd94")

Reviewer: Some("subagent:/root/issue_607_pre_spend_exact_r2")

Result: pass

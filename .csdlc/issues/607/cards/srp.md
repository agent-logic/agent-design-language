# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/607
.csdlc/issues/607
.csdlc/prepared/issues/607
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
adl-runtime/src/guardian.rs
adl/tools/issue607_guardian_recovery_proof.sh
adl/tools/issue607_probe_runtime.py
adl/tools/issue607_qualify_warm_polis.sh
adl/tools/issue607_validate_saved_plan.sh
adl/tools/run_issue607_warm_polis.sh
adl/tools/test_issue607_warm_polis.sh
adl/tools/validate_v092_runtime_guardian_lifecycle.sh
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
docs/operations/cloud/aws/shepherd-gpu-proof
infra/aws/runtime/gpu-proof

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

- The measured 240-second service-ready result applies to the current two-8B-model configuration; larger models can take longer to load.
- Only USD 0.016714 remains under the issue ceiling, so the aggregate guard rejects any further paid qualification action.

## Review Result

Revision: Some("git-blake3:fddba8dea9d965393f01a70dcf408b75a1857189:b257b892a36ca75cca46ae780713ac23f39acdbef34e5d52f56f91aafb4a2e3c")

Reviewer: Some("subagent:issue_607_final_publication_review")

Result: pass

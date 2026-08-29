# Structured Task Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Recover, rename, harden, test, and document one optional AWS GPU Shepherd proof runner; live execution is a separately authorized validation lane and not implicit preparation work.

## Deliverables

- Issue-#345-owned reusable AWS GPU proof command
- Executable deterministic runner contract and negative tests
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- Read-only business-account, IAM, no-ingress, artifact, quota, price, and reaper preflight
- Exact-revision model-backed GPU execution and bounded evidence contract
- Owner-bound trap, guest-timer, and permanent-reaper cleanup
- Redacted cost, proof, and cleanup receipt schema
- Operator runbook separating preflight from paid execution
- Fresh exact-head review and truthful publication handoff

## Acceptance

1. AC-1: One reusable issue-owned command exposes explicit preflight, exact-revision run, and owner-bound cleanup actions.
2. AC-2: Preflight verifies the agent-logic-admin business account, exact permanent instance profile/policies, named no-ingress security group, immutable S3 manifest/object versions, GPU quota, bounded On-Demand price, deadline reaper, and zero stale issue compute before launch.
3. AC-3: Paid execution requires an exact commit, unique run identity, explicit execute flag, retained operator authorization, and declared cost/deadline bounds; it launches at most one On-Demand GPU instance with no fallback or retry.
4. AC-4: The guest proves exact source revision, artifact digests, expected GPU/driver/runtime state, nonzero model GPU residency, and one current governed Shepherd real-model result.
5. AC-5: Local trap, guest deadline timer, and permanent tag-scoped reaper provide independent cleanup; success, failure, interruption, timeout, and cleanup retry leave no temporary instance or volume.
6. AC-6: Public evidence is machine-readable, digest-bound, cost-relevant, and redacted; credentials, account/resource identifiers, prompts, responses, private paths, and environment dumps are absent.
7. AC-7: Executable negative proofs cover lock collision, wrong account/profile, IAM or security-group drift, stale revision, artifact mismatch, deadline/interruption, and cleanup failure without paid mutation.
8. AC-8: Documentation states AWS is an optional portability proof, does not change local Shepherd acceptance, and does not create production inference fallback or standing launch authority.
9. AC-9: Fresh exact-head review has no unresolved actionable finding before publication, and a separate launch-readiness review is required after any source change before paid execution.

## Dependencies

- Historical recovery commit 7a26886c47962e71c128489f5176a045ae8e9a64
- Current governed Shepherd adapter and local real-model proof contract formerly owned by #5795
- Agent Logic business AWS account and operator-provisioned profile, IAM, no-ingress security group, immutable S3 model bundle, quota, and deadline reaper
- #256 as downstream consumer only

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/345
- docs/milestones/v0.92.1/features/DISTRIBUTED_MULTI_AGENT_RUNTIME_QUALIFICATION_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md
- git:7a26886c47962e71c128489f5176a045ae8e9a64:adl/tools/run_wp5795_aws_gpu_proof.sh
- git:7a26886c47962e71c128489f5176a045ae8e9a64:.csdlc/evidence/5795/foundation
- .csdlc/prepared/issues/345/design.md

## Non Goals

- Changing the local Shepherd adapter or its acceptance criteria
- Making AWS a Runtime dependency or inference fallback
- Creating or modifying IAM roles, policies, instance profiles, security groups, quotas, or permanent reaper resources
- Opening SSH or public ingress
- Using Spot capacity, retries, or another cloud/provider as fallback
- Executing or closing #256
- Claiming distributed multi-agent qualification beyond the bounded Shepherd GPU proof

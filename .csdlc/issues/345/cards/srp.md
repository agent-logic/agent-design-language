# Structured Review Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

infra/aws/runtime/gpu-proof
adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
docs/operations/cloud/aws/shepherd-gpu-proof/README.md
.csdlc/prepared/issues/345/design.md
.csdlc/prepared/issues/345/diagram.mmd
.csdlc/issues/345

## Prompts

- Can Terraform create any topology other than exactly one regular Runtime node and one GPU Ollama node, or omit the single shared key pair, public IPv4, or required /32 SSH ingress from either node?
- Can TCP/11434 become public or reachable from any source other than the Runtime security group, or can bootstrap depend on controller-side SSM commands?
- Can paid apply occur without current exact-head review, single-use authorization, immutable artifacts, both instance and disk costs, exact network and SSH bindings, Terraform source identity, deadline, and zero stale issue resources?
- Can either node fail during package/bootstrap/model/Runtime setup without a bounded receipt and three independent termination paths covering both instances and volumes?
- Does the GPU receipt prove every configured model is simultaneously resident with its expected digest, and does the Runtime receipt prove Guardian plus six real UTS/ACC/Freedom-Gate/runtime.observe executions without overstating kernel transit?
- Can public evidence or Terraform state escape the worktree, expose credentials/private key/raw identifiers/prompts/responses, or claim 24/7 readiness beyond this bounded qualification?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The live paid two-node GPU proof remains pending this recorded review and single-use authorization.
- PR #593 merged an older head; this reviewed candidate is not yet integrated into main.
- The bounded qualification does not establish 24/7 production readiness.

## Review Result

Revision: Some("git-blake3:a537fb45cb4067e863efd4ecf7740a0b259ad689:3672627451d63c5bd82b4308076c82998f242219b356ee53615a4100fbf56957")

Reviewer: Some("fresh-session:/root/issue_345_final_launch_review")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/build.rs
adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
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

- Final paid two-node success remains pending.
- The bounded qualification does not establish 24/7 production readiness.

## Review Result

Revision: Some("git-blake3:ec75a382f9a354ccc4677b769b18043fc9c21567:b5eb73953f999ca56c81a254db103f12d40d3f00215ee88982bc1dd05e3b8226")

Reviewer: Some("fresh-session:/root/issue_345_final_launch_review")

Result: pass

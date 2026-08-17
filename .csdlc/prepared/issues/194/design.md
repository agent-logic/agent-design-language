# Issue #194 design: private Wuji-AWS recovery qualification

## Intent

Issue #194 qualifies the private, fail-safe AWS side of the WP-04.16d recovery lane before #142 integration. The issue-owned harness must create only ephemeral tagged infrastructure in the Agent Logic AWS account, prove private reachability/model readiness, preserve redacted receipts, and clean up all AWS resources after every success or failure.

## Current implementation slice

The retained harness is intentionally CloudFormation-first for this private qualification:

- `adl/tools/issue194_private_network.cloudformation.json` creates a private VPC with two private subnets, SSM interface endpoints, an S3 gateway endpoint, no Internet gateway, no NAT gateway, no public subnet route, and no public Runtime/model exposure.
- `adl/tools/issue194_private_wuji_aws_runner.sh` creates, preflights, launches, smokes, deletes, and asserts zero for issue-tagged resources.
- `adl/tools/private_wuji_aws_recovery_qualification.py` validates plans/inventory and emits redacted receipts without raw AWS identifiers.
- `adl/tools/issue194_model_health_command.py` generates the shepherd-invoked private model-health command for a GPU voter.

## Network and authority boundary

SSM is the shepherd maintenance and recovery plane. It is used to invoke bounded smoke commands, inspect readiness, and perform cleanup/recovery. It is not the agent-to-agent data plane.

Agent/voter peer traffic must use direct private TCP/IP over the private security-group mesh. The current proof records this as ACIP-ready private adjacency, not production ACIP semantics.

Private model artifacts are delivered through the regional S3 gateway endpoint and the issue-owned S3 endpoint/security-group path. Runtime/model endpoints remain loopback-only on the voter during the model-health proof.

## Live evidence split

The AWS quota currently prevents launching two `g6.xlarge` GPU voters simultaneously in the account. The implementation therefore retains two complementary private proofs:

1. `two_voter_private_network_smoke`: two private AWS voters in separate AZs prove SSM shepherd reachability, bidirectional direct private TCP adjacency, no public IPs, and zero cleanup.
2. `single_gpu_private_model_health`: one private `g6.xlarge` voter proves private S3 artifact delivery, local Ollama `gemma4:12b` health/generation, restart persistence, and zero cleanup.

This split does not claim simultaneous two-GPU model health and does not complete the serial hybrid recovery acceptance item by itself.

## Remaining acceptance gap

The full #194 outcome still requires a serial hybrid run with one Wuji voter and two AWS voters proving snapshot recovery, Wuji partition, AWS continuity, heal/demotion, and true one-of-three halt. That behavior remains unimplemented/unproven in this worktree and must not be represented as complete until a later exact live proof exists.


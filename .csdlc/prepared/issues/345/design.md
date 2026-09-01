# Issue #345 design — two-node AWS Runtime qualification

## Purpose

Provide one bounded, repeatable AWS qualification of the actual deployment
shape: a regular EC2 node runs Guardian, Runtime v3, six governed agents, UTS,
ACC, and Freedom Gate; a separate GPU EC2 node runs Ollama with at least two
simultaneously resident models. Terraform is the only infrastructure creation
path. This qualification remains temporary and budget-bound; it is not the
later 24/7 production deployment.

## Authority and scope

Issue #345 owns the issue-local Terraform root, its runner and focused tests,
the AWS proof runbook, typed issue artifacts, and redacted evidence. It does
not change Runtime admission semantics, model defaults, production DNS, or the
future multi-region/24/7 topology. The approved `agent-logic-admin` profile and
immutable versioned model bundle are inputs.

SSM is installed and enabled on both nodes for recovery only. It is never the
bootstrap transport. Cloud-init starts both workloads automatically. Both
instances always receive public IPv4 and use the same Terraform-managed EC2
key pair; TCP/22 ingress is required on both and restricted to one validated
operator IPv4 `/32`. No other public ingress exists.

## Terraform topology

The root at `infra/aws/runtime/gpu-proof` owns exactly two On-Demand instances,
two security groups, one shared key pair, separate least-privilege instance
roles/profiles, encrypted delete-on-termination gp3 root disks, and one
EventBridge Scheduler deadline targeting both exact instance IDs.

The GPU security group admits TCP/11434 only from the Runtime security group.
The Runtime cloud-init receives the GPU private address from Terraform. The
GPU node restores the immutable Ollama runtime and every configured model from
S3, starts one persistent Ollama service with infinite keep-alive and a loaded
model limit matching the configured set, exercises every model, and publishes
a digest-bound readiness receipt only after all expected models have nonzero
GPU residency.

The Runtime node waits for that private readiness surface, restores the exact
reviewed repository archive from versioned S3, then runs the Guardian lifecycle
proof and six real Runtime-agent cycles through UTS, ACC, Freedom Gate, and
`runtime.observe`. The governed Shepherd smoke contract is run once per
configured model against the private GPU endpoint. Evidence does not claim an
unimplemented Runtime-v3 kernel-to-Ollama transit path.

## Launch and cost controls

`preflight` is read-only with respect to paid compute. It verifies the business
account, both AMIs and instance prices, GPU quota, VPC/subnet, immutable S3
manifest, SSH key and `/32`, Terraform source identity, total compute/storage/
IPv4/request cost, and zero stale issue instances or volumes.

Paid `run` requires a clean reviewed revision, a unique run ID, explicit
`--execute`, and one retained single-use authorization binding both instance
types, both disks, immutable artifacts, network inputs, SSH key hash, `/32`,
Terraform source identity, deadline, and combined cost ceiling. The runner
creates a saved Terraform plan and applies exactly that file. Its digest is
retained with the final evidence. There is no Spot fallback, retry launch, or
unreviewed infrastructure path.

## Resilience and cleanup

Three independent termination paths cover both nodes: the controller's
Terraform destroy trap, a guest-local systemd deadline shutdown on each node,
and the tag-constrained Terraform-managed Scheduler target. Cleanup is
owner-bound and verifies zero matching instances and volumes before releasing
the run lock. All Terraform state, plans, authorization copies, generated
bootstrap files, and receipts remain beneath `.adl/local/issue345` in the bound
worktree.

## Evidence and privacy

Public evidence contains only source and artifact digests, model identities and
residency facts, bounded component outcomes, cost inputs, the saved-plan digest,
and cleanup counts. It excludes credentials, private keys, raw AWS identifiers,
prompts, responses, private paths, and environment dumps. Local contract tests
prove the static topology and fail-closed input/authorization behavior without
launching compute. Live AWS execution is valid only after a fresh exact-head
review and typed publication.

## Rollback

Terraform destroy removes the temporary two-node deployment. The deadline
paths remain effective if the controller disappears. Rollback does not delete
the versioned S3 model bundle or change Runtime/Guardian behavior outside the
issue-owned qualification lane.

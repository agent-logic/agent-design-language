# Structured Planning Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement and validate one Terraform-owned, two-node AWS qualification: a regular Runtime/Guardian/six-agent node and a private-linked Ollama GPU node, with one shared SSH key pair, automatic cloud-init, exact cost and artifact binding, three independent cleanup paths, fresh exact-head review, and one separately authorized paid proof.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Recover published lifecycle truth and revise issue design/cards for the Terraform-owned two-node Runtime and GPU topology with one mandatory shared SSH key pair.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the isolated Terraform root, automatic Runtime/GPU cloud-init payloads, private Ollama route, S3 receipts, exact deadline scheduler, and Terraform destroy cleanup.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Replace single-host authorization and preflight with exact two-node Terraform-plan, SSH, artifact, account, cost, and zero-residue binding.",
    "acceptance_ids": [
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused no-paid Terraform, shell, receipt, authorization, Git, and live read-only AWS validation and resolve every failure.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain a fresh exact-head bounded review, fix every actionable finding, and republish PR #593 through typed lifecycle authority.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "in_progress"
  },
  {
    "id": "S6",
    "action": "After new exact-run authorization, execute one bounded real two-node AWS proof, retain redacted runtime/model/ACC/cost/cleanup evidence, re-review, and finish only when PR #593 is green.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- The approved Agent Logic business account is verified before mutation.
- Terraform owns exactly two On-Demand instances, their two security groups, one shared EC2 key pair, least-privilege roles/profiles, encrypted root disks, and the deadline scheduler.
- Both nodes always have public IPv4 and mandatory TCP/22 ingress from exactly one valid operator IPv4 /32; no other public ingress is created.
- GPU TCP/11434 is reachable only from the Runtime security group over the private address.
- Cloud-init performs normal bootstrap and SSM remains recovery-only.
- No paid apply occurs without exact current review, typed publication, single-use authorization, immutable artifacts, exact Terraform source and inputs, run ID, deadline, and combined cost bound.
- Every configured model is digest-attested and simultaneously GPU-resident before the Runtime proof begins.
- Controller destroy, guest shutdown deadline, and tag-constrained Scheduler independently cover both nodes.
- All operator/run/Terraform state stays under .adl/local/issue345 in the bound worktree and public evidence is redacted.
- AWS remains a bounded qualification lane and does not claim 24/7 production readiness.

## Risks

- Recovered shell logic may encode stale account, AMI, IAM, artifact, quota, or pricing assumptions.
- A launch/lock ordering gap could leave unowned paid compute.
- Cleanup could target another run or fail after the operator session exits.
- S3 or source drift could produce a model result that is not bound to the reviewed revision.
- Raw AWS responses or model data could leak into retained evidence.
- A long bootstrap could exceed the declared budget before proof begins.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/345/design.md

Digest: 2c10beb0d51f1f1c755fd5bfa66eaf72597e270d1ab22d5d7bd371a31884c0f7

## Diagram

.csdlc/prepared/issues/345/diagram.mmd

Digest: 4565e689e454d83ea8e9ee9e450bf4a7fcb4ef10d1383efa948d34ef068f6554

## Stop Conditions

- The configured AWS profile does not resolve to the approved business account.
- Terraform does not validate to exactly two On-Demand nodes, one shared key pair, mandatory /32 SSH on both, private SG-only Ollama, or all three deadline paths.
- Exact review, source, Terraform inputs, SSH key/CIDR, VPC/subnet/AMI, artifact, quota, or combined cost identity cannot be proven.
- A paid apply lacks current typed publication and explicit single-use exact-run authorization or exceeds its cost/deadline ceiling.
- Either cloud-init path depends on controller-side SSM commands or live Git checkout.
- The complete configured model set is not simultaneously GPU-resident with expected digests.
- Guardian, Runtime, six UTS/ACC/Freedom-Gate/runtime.observe agent executions, or truthful non-transit evidence cannot be proven.
- Cleanup ownership is ambiguous or any matching instance or volume remains.
- State escapes the bound worktree or evidence cannot be redacted without losing the proving denominator.
- Fresh review reports an unresolved actionable finding.

## Handoff

Proceed only after doctor readiness.

# Structured Review Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact current Git commit 0a739f44c157b26af7d2e25efe9c6ff4feb6f6f5 on branch codex/731-gcp-d1-private-foundation-non-gpu-disposal, plus uncommitted typed recovery/reassignment metadata only.
Substantive live-completion evidence commit 78490bd7bc8f0815914469fb4c0323d23437dd13 and assignment metadata commit 0a739f44c157b26af7d2e25efe9c6ff4feb6f6f5.
Terraform private-foundation implementation in infra/gcp/platform/main.tf, variables.tf, terraform.tfvars.example, and .terraform.lock.hcl.
Issue-owned #731 validators and evidence under .csdlc/prepared/issues/731, .csdlc/evidence/731, and .csdlc/issues/731.
Verify the live mutation was explicitly authorized in .csdlc/evidence/731/mutation-authorization-request.json and bound to applied plan digest e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df.
Verify that #731 intentionally labels GCP resources with issue=493 per the CSM parent resource contract; do not treat issue=493 labels as drift from #731.
Verify the successful foundation apply/readback proves the accepted denominator without public exposure, broad IAM, GPU, NAT/LB/DNS, or unintended workload resources.
Verify the disposable workload evidence proves exactly one labelled private e2-micro with no external IP, auto-delete standard boot disk, successful delete, and zero residual instances, run-labelled instances, disks, or addresses.
Verify SOR/SRP truth is current for publication and that the PR may use Closes #731 if the live proof satisfies the issue contract.

## Prompts

- Does the saved Terraform plan exactly match the #731/#493 denominator with no public exposure, broad IAM, replacement, or extra resources?
- Are live operations gated by fresh explicit authorization naming every required identity, digest, deadline, and rollback command?
- Does the proof use short-lived impersonation without static keys or credential exposure?
- Does the disposable VM prove RUNNING without external IP, then destroy the VM and boot disk within the authorized window?
- Does independent zero-residue readback cover every required resource class and exact run label?
- Does the final SOR distinguish read-only preflight, live authorized mutation, cleanup, deferred CI, and non-goals truthfully?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The steady-state private GCP-D1 foundation remains deployed as the accepted #731 output; later teardown requires a separate explicit cleanup issue/authorization.

## Review Result

Revision: Some("git-blake3:0a739f44c157b26af7d2e25efe9c6ff4feb6f6f5:fbea409937136d5bb44c66d53f4c8065c798d685118c9ba84268fd734d83bf4c")

Reviewer: Some("fresh-session:d34c60f5-1acf-4407-93d0-3db6e62f156a")

Result: pass

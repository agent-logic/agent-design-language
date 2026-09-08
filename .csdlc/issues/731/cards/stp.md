# Structured Task Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bootstrap, design-review, bind, and execute only the bounded GCP-D1 private foundation plus one disposable non-GPU workload proof, with a hard stop at the live mutation authorization packet before any apply/run.

## Deliverables

- Typed #731 lifecycle cards and issue-local design packet
- Reviewed saved Terraform plan proof for the exact private-foundation denominator
- Read-only GCP preflight proof for required APIs, quota, IAM, billing, remote state, and impersonation readiness
- Fresh operator authorization packet for any live apply/run
- Live foundation readback evidence with redacted identifiers
- One disposable e2-micro no-external-IP workload run receipt with health/readiness observation
- Unconditional workload destroy receipt and independent zero-residue readback
- Foundation post-apply drift/readback proof or exact rollback proof
- Truthful SOR/SRP evidence and exact-head independent review

## Acceptance

1. AC-1: A reviewed saved foundation plan contains exactly the bounded denominator; unexpected replacement, public IP/route, public ingress, broad IAM, or any extra resource fails closed before apply.
2. AC-2: Short-lived impersonation is used for live operations; no static service-account key is required or exposed.
3. AC-3: Live readback proves the private foundation, separate human/workload identities, five storage-owner boundaries, labels, and watchdog metric.
4. AC-4: The one disposable VM reaches RUNNING without an external IP and emits one bounded health/readiness observation through the declared workload identity.
5. AC-5: The VM and its auto-delete boot disk are destroyed within the authorization window.
6. AC-6: Independent post-destroy readback enumerates instances, disks, addresses, forwarding/load-balancer resources, firewall overrides, VM-specific IAM, storage objects/all versions, and Terraform state for the exact run labels; every disposable-workload denominator is empty.
7. AC-7: Foundation resources remain only if their exact post-apply readback matches the reviewed plan. Any partial or drifted foundation is rolled back.
8. AC-8: Exact-head independent review and required CI are green before merge.

## Dependencies

- #490 accepted long-term project and us-west2 authority
- GCP-B1 corrective merged, with usable remote state and impersonation proof
- #492 / PR #580 historical design provenance
- #493 / PR #587 historical private-foundation design provenance
- Required APIs, quota, IAM, and billing read-only preflight

## Inputs

- agent-logic/agent-design-language#731
- agent-logic/agent-design-language#490
- agent-logic/agent-design-language#492
- agent-logic/agent-design-language#493
- agent-logic/agent-design-language#580
- agent-logic/agent-design-language#587
- infra/gcp/platform/**
- .adl/docs/TBD/GCP_ACCOUNT_MOVE_IN_PLAN.md
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md

## Non Goals

- GPU qualification or any machine larger than e2-micro
- Production traffic, public ingress, Shared VPC, NAT expansion, DNS, or load balancing
- AWS or cross-cloud work
- Redesigning the #493 Terraform foundation
- Enabling APIs, broad IAM repair, or billing mutation unless separately authorized and decomposed
- Publishing or merging without exact-head independent review and required CI

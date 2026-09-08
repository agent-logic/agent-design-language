# Structured Intent Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deploy the already-reviewed private GCP foundation in the accepted company project, run exactly one private disposable non-GPU workload, destroy it, and retain independent zero-residue proof.

## Required Outcome

The accepted company GCP project has the bounded private foundation proved by live readback, one e2-micro no-external-IP disposable VM is proved running and then destroyed within the authorized window, and independent post-destroy readback proves no disposable workload residue.

## Scope

- infra/gcp/platform/**
- .csdlc/prepared/issues/731/**
- .csdlc/issues/731/**
- .csdlc/evidence/731/**

## Authority

- Issue authority is agent-logic/agent-design-language#731.
- The only target project is cs-host-377d41e71a824f92802120.
- The only region and zone are us-west2 and us-west2-a.
- The only CSM/environment pair is axioma/dev.
- The only network/subnet names are axioma-dev-csm-private and axioma-dev-csm-private-us-west2 with CIDR 10.42.0.0/24.
- Foundation mutation is limited to resources already declared by infra/gcp/platform/**: one custom VPC, one subnet, three firewall rules, OS Login metadata, two operator IAM memberships, one workload service account, five labelled buckets and their declared IAM memberships, one project log-writer membership, and one logging metric.
- Disposable workload mutation is limited to exactly one e2-micro Linux VM with one auto-delete standard persistent boot disk no larger than 10 GiB, no external IP, no static address, no GPU, no additional disk, no load balancer, no DNS, and no production traffic.
- Live GCP mutation requires fresh operator authorization naming exact project, region, zone, plan digest, network/subnet, run ID, instance name, deadline, impersonated identity, and rollback commands.
- Authorization expires 120 minutes after issuance; VM lifetime is capped at 30 minutes.
- Total proof-run spend cap is USD 1 and foundation steady-state incremental spend cap is USD 5 for the first 30 days.
- C-SDLC typed lifecycle state remains the workflow authority; raw GitHub lifecycle writes are not authorized by this issue.

## Assumptions

- none

## Operator Constraints

- Do not apply Terraform, create a VM, enable APIs, alter IAM, or spend money before fresh explicit operator authorization.
- Do not print, copy, retain, or commit credentials, tokens, static service-account keys, account secrets, or raw private GCP responses containing sensitive data.
- Use short-lived impersonation for live proof; no static service-account key is required.
- Stop before apply if read-only preflight, saved plan denominator, or cost estimate exceeds the exact issue bounds.
- Keep the primary checkout inspection-only and preserve unrelated .csdlc/issues/725 staging.
- Do not widen into GPU qualification, production traffic, public ingress, Shared VPC, NAT expansion, DNS, load balancing, AWS, or cross-cloud work.

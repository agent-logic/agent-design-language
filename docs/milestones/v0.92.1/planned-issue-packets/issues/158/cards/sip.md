# Structured Intent Prompt

Template: 1.0.0

Issue: 158

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Migrate production AWS infrastructure into the Agent Logic business account with DNS, TLS, email, storage, monitoring, workload, and rollback proof.

## Required Outcome

DNS TLS email storage monitoring workload and rollback receipts is produced at an exact revision and independently reproducible.

## Scope

- Agent Logic business AWS identity, Route53, ACM, SES, S3, CloudFront, compute, IAM, monitoring, backups, budgets, account contacts, and temporary migration resources.

## Authority

- Issue CORP-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound

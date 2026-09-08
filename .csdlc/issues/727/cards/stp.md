# Structured Task Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One AWS-D account-foundation reconciliation and redacted proof packet.

## Deliverables

- saved Terraform plan digest
- redacted preflight and operational readbacks
- truthful C-SDLC cards and evidence

## Acceptance

1. AC-1 exact account region source backend workspace and saved-plan preflight
2. AC-2 explicit operator mutation envelope
3. AC-3 exact declared resources only
4. AC-4 redacted operational readbacks pass
5. AC-5 evidence contains digests and booleans without sensitive values
6. AC-6 partial failure reconciles or reports residue
7. AC-7 fresh exact-head review before publication

## Dependencies

- #487 and PR #574 terminal
- #516 release-tail admission
- operator live-write authorization

## Inputs

- infra/aws/account-foundation
- docs/operations/cloud/aws/audit-security/AWS_AUDIT_SECURITY_BASELINE.md
- docs/milestones/v0.92.1/evidence/cloud/aws-d

## Non Goals

- rewrite #487
- multi-account rollout
- website Runtime DDNS public edge GCP or application security

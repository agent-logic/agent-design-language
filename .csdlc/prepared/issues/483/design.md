# CORP-B Design

Issue: #483

## Objective

Produce a concise corporate account custody register and action list for the
critical-service denominator derived from the accepted CORP-A asset schedule.

## Narrowed Operator Boundary

This issue is read-only with respect to external services. It may inspect and
record redacted readback evidence, but it must not transfer domains, move hosted
zones, change DNS, mutate AWS accounts, alter account administrators, alter
billing, weaken or change MFA, exercise secret recovery flows, move vault
records, use break-glass credentials, or change any service configuration.

## Acceptance Conflict

The GitHub issue text asks for recovery and break-glass exercises proving
corporate custody. The current operator boundary forbids those operations in
this issue. The deliverable therefore accepts a register and follow-up action
list, not a claim that every service has already completed live custody,
recovery, vault, and break-glass remediation.

## Inputs

- `docs/operations/corporate/asset-register/critical-asset-schedule.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json`
- `.git/csdlc-v2/domain-transfers/*.json` as nonsecret local operational
  receipts for five completed internal Route53 registration transfers
- Live read-only GitHub issue and PR state

## Deliverables

- `docs/operations/corporate/account-custody/corporate-custody-register.md`
- `docs/operations/corporate/account-custody/corporate-custody-register.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-b/readback-receipts.v1.json`
- Issue-local validator and evidence proving denominator coverage, redaction,
  domain-transfer receipt ingestion, and action-list ownership.

## Register Semantics

Each row records service class, source asset, current readback, company custody
claim level, billing/admin/MFA/recovery/vault/break-glass posture, later owner,
and action status. A row may be accepted as a truthful register row while still
carrying `follow_up_required`; that status is not a custody-complete claim.

## Non-Goals

- No external account or provider mutation.
- No DNS, hosted-zone, registrar, billing, admin, MFA, vault, recovery, or
  break-glass operation.
- No credentials, secrets, PII, payment data, tax data, private documents, or
  recovery materials in repository artifacts.
- No scheduling or milestone gate for `v-*.ai` backlog domain transfers,
  including `v-dev.ai`.
- No CORP-C operational-control transfer work.

## Validation

- `ruby .csdlc/prepared/issues/483/validate-custody-register.rb`
- `git diff main...HEAD --check`

## Review Prompts

- Does the register cover every CORP-A critical service class exactly once or
  route it as an explicit non-service/supporting row?
- Are the five completed domain-registration transfers recorded factually
  without claiming DNS hosted-zone migration?
- Are `v-*.ai` domains, including `v-dev.ai`, treated only as unscheduled
  backlog?
- Does the register avoid credentials, PII, payment data, tax data, private
  instruments, and recovery material?
- Does the PR avoid overclaiming the original #483 recovery/break-glass ACs
  under the narrowed read-only boundary?

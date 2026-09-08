# Structured Task Prompt

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One impersonation-only remote-state bootstrap and recovery proof; no platform workload work.

## Deliverables

- impersonation-only Terraform/provider and command path
- reviewed exact saved-plan guard
- remote backend recovery proof
- local-state cleanup proof
- redacted evidence

## Acceptance

1. AC-1: Terraform and commands use short-lived impersonation and reject static-key execution.
2. AC-2: The reviewed plan and apply contain exactly the approved bucket and bucket-IAM denominator.
3. AC-3: Live readback proves private versioned recoverable backend controls.
4. AC-4: An immutable-generation canary is recovered with matching digest from a clean backend initialization.
5. AC-5: Repository and worktree contain zero local state, plan, backend, provider-cache, or credential residue.
6. AC-6: Rollback and budget/time caps fail closed and exact-head review is green.

## Dependencies

- #490 accepted hierarchy/project/region authority
- #491 and PR #575 historical provenance
- approved company source identity with Token Creator

## Inputs

- infra/gcp/bootstrap/**
- .csdlc/prepared/issues/491/**
- docs/operations/cloud/gcp/terraform-bootstrap/**
- docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md

## Non Goals

- GCP platform or workload deployment
- GPU or Runtime work
- broader IAM cleanup
- billing or project mutation

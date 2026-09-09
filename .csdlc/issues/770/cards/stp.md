# Structured Task Prompt

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remediate S520-SEC-002 under #770.

## Deliverables

- infra/aws/csm-runtime-spot/tests/run_contract.sh
- Public SSH preconditions, negative Terraform tests, updated example and recovery runbook, sanitized live/disposal evidence.
- infra/aws/csm-runtime-spot/tests/run_live_proof.sh

## Acceptance

1. AC-1: Public null or blank key is rejected before apply.
2. AC-2: Public empty SSH CIDRs are rejected before apply.
3. AC-3: One existing approved key and explicit /32 SSH documented and proved.
4. AC-4: Runtime application ingress, IMDSv2 and encrypted EBS preserved.
5. AC-5: Separate private-only deployment is unchanged.
6. AC-6: Authorized bounded AWS reachability/isolation proof and disposal evidence retained.

## Dependencies

- #520 security finding; operator key selection and bounded cloud approval for live proof.

## Inputs

- infra/aws/modules/csm-runtime-spot
- infra/aws/csm-runtime-spot
- infra/aws/csm-runtime-spot/tests/run_contract.sh

## Non Goals

- Additional key pairs
- Changes to separate private-only deployment roots
- Unrelated network redesign or unapproved cloud apply

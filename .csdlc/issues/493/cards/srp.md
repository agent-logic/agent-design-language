# Structured Review Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/493/exact-diff-hygiene.log
.csdlc/evidence/493/gcp-d-static-product.log
.csdlc/evidence/493/terraform-fmt.log
.csdlc/issues/493
.csdlc/prepared/issues/493/design.md
.csdlc/prepared/issues/493/diagram.mmd
.csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh
adl-runtime/tests/config_reload.rs
adl-runtime/tests/guardian_cli.rs
docs/milestones/v0.92.1/evidence/cloud/gcp-d/gcp-d-platform-foundation-proof.md
docs/operations/cloud/gcp/platform-foundation/README.md
docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh
infra/gcp/platform

## Prompts

- Does the design keep #493 to GCP-D private platform foundation without absorbing GCP-E, XCL-01, Observatory, Unity, production traffic, or AWS work?
- Are private network, IAP/OS Login operator access, identity separation, storage ownership, telemetry/watchdog, and cleanup selectors specified with machine-checkable proof?
- Does the live GCP proof plan avoid credential disclosure and require explicit authorization before mutation?
- Are dependency and #492 terminal-cache boundaries truthful rather than hidden acceptance of broad GCP drift?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

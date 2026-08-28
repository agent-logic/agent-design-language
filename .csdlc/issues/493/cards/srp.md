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

- No live GCP, AWS, paid, Terraform init/plan/apply/destroy, or cloud readback proof was run for this issue.
- PR #587 CI must be renewed after publication to prove the runtime coverage remediation on GitHub standard runners.

## Review Result

Revision: Some("git-blake3:0e197b3c08f6dbab60eeae3b6c5d9be6597f57d7:f55099a9e8d76e13f225a8ea33a27a86789e061b34c24f1572e811983ae9667c")

Reviewer: Some("fresh-session:30e8dbac-f7d1-4b75-9fe5-ebccdadb50bc")

Result: pass

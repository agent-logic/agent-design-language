# Structured Review Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/493
.csdlc/prepared/issues/493
.csdlc/evidence/493
infra/gcp/platform
docs/operations/cloud/gcp/platform-foundation/README.md
docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh
docs/milestones/v0.92.1/evidence/cloud/gcp-d/gcp-d-platform-foundation-proof.md

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

- No live GCP apply/destroy/readback proof was performed for #493; this issue remains bounded to static local product proof unless explicit cloud mutation is separately authorized.
- No credential, cloud, GitHub, or lifecycle mutation was performed by the reviewer.
- Terraform provider initialization and live plan validation were not part of the review; local proof covered the issue-owned static validator, shell syntax checks, and scoped diff hygiene.

## Review Result

Revision: Some("git-blake3:30086bd7398ec75fd99d2b54775cfcf770dbe6d9:231e4dacfafd6b6a08663b8cbf4796a98f15021835023ba082500b299f3ee32b")

Reviewer: Some("fresh-session:f9f482f0-f6e8-42a6-ae73-9abc80e21167")

Result: pass

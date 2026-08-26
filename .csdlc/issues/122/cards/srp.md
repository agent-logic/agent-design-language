# Structured Review Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

infra/aws/csm-public-edge/
infra/aws/csm-runtime-spot/
infra/aws/csm-runtime-alb/
infra/aws/modules/csm-runtime-spot/
infra/aws/modules/csm-runtime-alb/
adl/tools/validate_csm_public_edge_static.sh
adl/tools/validate_csm_public_edge_live.sh
docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
docs/milestones/post-v0.92/runbooks/CSM_PUBLIC_EDGE_AND_RUNTIME_ORIGIN_RUNBOOK.md

## Prompts

- Can any public read, browser state, origin, or unsigned request gain Runtime write authority?
- Do the exact deployed Observatory and Runtime gateway revisions match through DNS, cache, HTTPS, and WSS paths?
- Are CORS, CSP, WSS origins, authentication, rate limits, redaction, health, and error responses fail-closed and public-safe?
- Does every resource belong to the verified Agent Logic business account with bounded ownership, rollback, and cleanup?
- Can any plan or tool create or operate EC2, Spot, or CodeBuild, or begin without separate operator authorization?
- Does #122 remain deferred beyond v0.92 and non-gating for #83 and #111-#117?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live AWS, credentialed Terraform, or WebSocket probe was run by the reviewer.
- WSS handshake remains an explicit residual gate pending the separately approved Runtime WebSocket probe.
- Historical append-only audit entries retain earlier overclaim/repair history; current VPP/SOR/runbook truth no longer relies on those overclaims.

## Review Result

Revision: Some("git-blake3:ee91c87339fdfd5ad8effdfe386bc64d6f268e0f:f0afbd15218f708a73af4d22ad28dde2a320ee4068f2f144f1d80887a87a8ead")

Reviewer: Some("fresh-session:fa41854f-fe62-4a61-9de2-220be687d341")

Result: pass

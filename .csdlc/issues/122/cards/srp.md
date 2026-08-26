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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

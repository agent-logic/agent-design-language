# Structured Review Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.gitignore
.csdlc/issues/122
.csdlc/evidence/122
.csdlc/prepared/issues/122/terraform-execution-plan.md
adl/tools/validate_csm_public_edge_static.sh
adl/tools/validate_csm_public_edge_live.sh
docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
infra/aws/csm-public-edge

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

- Live AWS plan/apply, DNS propagation, ACM issuance, and live HTTPS/WSS handshake proof remain operator-gated and are not claimed by this implementation PR.
- Terraform hostname validation intentionally enforces exact origin shape and wildcard/path rejection; it does not attempt to be a complete public DNS hostname parser.

## Review Result

Revision: Some("git-blake3:0a0a1d14dad1374a4856710deabd2166f4b1bbb9:57eac29b9708aa6e91305534a6072fe6ea53b79200ef8ca16b0016e151aea427")

Reviewer: Some("fresh-session:485afd45-1935-4819-9616-f232c9743e2b")

Result: pass

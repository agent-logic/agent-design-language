# Structured Review Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Fresh no-context exact-head review for #122 at clean commit ed39c2be6f82222b3ed746b9899e01c04e128cbe on branch codex/122-csm-public-edge-terraform in /Volumes/FastWork/adl-worktrees/adl-issue-122-csm-public-edge-terraform.
Review full #122 current implemented scope: permanent CSM public edge Terraform, disposable Spot EC2 and ALB Runtime-origin modules/roots, operator runbook, AWS live evidence, certificate/DNS cleanup truth, lifecycle cards, and prior review finding dispositions.
Specifically verify the prior P2 runbook finding is resolved: no API Gateway WebSocket front-door claim, WSS is documented as CloudFront to configured WSS-capable HTTPS origin, and runbook variable names match Terraform variables: zone_name, runtime_origin_url, wss_origin_https_url, websocket_path_pattern, origin_fqdn_override, and subnet_ids.
Verify permanent edge remains non-compute, disposable origin smoke proof is operator-authorized and torn down, additional allowed origins are exact and fail closed for wildcards, reusable ACM lookup is truthful, stale patch artifact is no longer an applicable patch claim, and no #83/#111-#117/#550 scope is absorbed.
Findings-first PASS/FAIL; read-only; no source/lifecycle/GitHub/AWS mutation.

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

- Reviewer did not run live AWS reads/writes, Terraform apply/destroy, GitHub writes, or lifecycle mutation.
- Reviewer did not perform a fresh live WSS handshake; retained evidence and static/local validation were reviewed.
- Source and implementation evidence was pinned to HEAD ed39c2be6f82222b3ed746b9899e01c04e128cbe; dirty .csdlc/issues/122 files were current review-assignment metadata only.

## Review Result

Revision: Some("git-blake3:ed39c2be6f82222b3ed746b9899e01c04e128cbe:d2d8286b47ed90ac04258749dc078d6ea5df8ec525d0318d8518599101a5850d")

Reviewer: Some("fresh-session:be7f4e21-5885-4af8-afea-f012251658d4")

Result: pass

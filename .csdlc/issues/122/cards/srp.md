# Structured Review Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Fresh no-context exact-head review for #122 at clean post-whitespace-fix commit 1f68195a2ac1c79156409e0f68fbfb0a24e9bd16 on branch codex/122-csm-public-edge-terraform in /Volumes/FastWork/adl-worktrees/adl-issue-122-csm-public-edge-terraform.
Review full #122 current implemented scope after merging origin/main e2c1d1649 and fixing the P3 evidence whitespace finding: permanent CSM public edge Terraform, disposable Spot EC2 and ALB Runtime-origin modules/roots, operator runbook, AWS live evidence, certificate/DNS cleanup truth, lifecycle cards, prior review finding dispositions, and no collision with merged #482/#540/#541/corporate/onboarding/runtime-config changes.
Specifically verify prior P3 is resolved: git diff --check origin/main...HEAD is clean for .csdlc/evidence/122/gemini-review-diff.historical.txt and all #122 evidence/lifecycle surfaces.
Verify the prior P2 runbook finding remains resolved: no API Gateway WebSocket front-door claim, WSS is documented as CloudFront to configured WSS-capable HTTPS origin, and runbook variable names match Terraform variables: zone_name, runtime_origin_url, wss_origin_https_url, websocket_path_pattern, origin_fqdn_override, and subnet_ids.
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

- Reviewer did not run AWS live checks, Terraform apply/destroy, or credentialed operations.
- Reviewer did not rerun terraform init/validate during review; parent session reran static validator, Terraform validates, diff hygiene, and typed validation before assignment.
- PASS is for the assigned exact head and committed implementation/evidence truth, not future live DNS, cert, Runtime, Caddy, CloudFront propagation, or AWS account state drift.

## Review Result

Revision: Some("git-blake3:1f68195a2ac1c79156409e0f68fbfb0a24e9bd16:de12e7b482ecb43d434bb9d953eb574c5447de3c58a7efe02d3bafe4b7d27e04")

Reviewer: Some("fresh-session:570ae1e6-92f1-4422-95cc-40452d9f05cb")

Result: pass

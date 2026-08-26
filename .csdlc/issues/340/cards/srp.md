# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact clean head 43286fbcb4e525ad2ff0c02e57ded338d033e1e1 after typed recovery projection.
Product/source/test workflow content remains at reviewed conflict-resolution content from 15240578ff541ca3220513adfd23ed1cbb649178 plus typed lifecycle metadata-only commits.
Final conflict resolution for .github/workflows/ci.yaml and adl/tools/test_ci_runtime_contracts.sh, including standard runner enforcement and optional linker preservation.
#340 HTML Observatory / CSMctl product behavior and bounded learner test-harness fixes retained; no #341/#343/Unity/AWS/provider/#84/#122/#251 scope.

## Prompts

- Does #340 prove the live Runtime v3 start/stop/restart path rather than fixture/static rendering only?
- Does CSMctl start require /v1/ready, /v1/observatory, and /v1/health HTTP 200 before success?
- Does CSMctl stop prove graceful checkpoint/dehydration behavior and script-owned PID/lease cleanup?
- Does the replay/reconnect evidence prove bounded replay, no duplicate application, fresh correlation, unchanged authorization, and redacted projections?
- Does the change avoid Unity/#84/#122/#251, AWS/public, provider, #341/#343, and HTML child implementation scope?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was scoped to the final conflict-resolution delta and retained #340 product/test surfaces; no #341, #343, Unity, AWS/public hosting, provider credential, #84, #122, or #251 scope was claimed.
- GitHub CI remains the authoritative integration proof for the published PR head.

## Review Result

Revision: Some("git-blake3:43286fbcb4e525ad2ff0c02e57ded338d033e1e1:e1e0111b48531fd067d4800018290469f295a379594e54e94e2fdc73e8184284")

Reviewer: Some("issue340_final_conflict_review")

Result: pass

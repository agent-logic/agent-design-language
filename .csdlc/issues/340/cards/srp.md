# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
.csdlc/issues/340

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

- Reviewer scope was intentionally minimal after prior full #340 reviews and focused only on the R6 P2 repair, lifecycle truth, and scope non-absorption.
- Reviewer did not rerun Cargo or llvm-cov lanes because the review assignment was read-only; retained current proof and source/test contracts were inspected instead.
- This review does not claim Unity, AWS/public hosting, provider credentials, #341, #343, #84, #122, or #251 scope.

## Review Result

Revision: Some("git-blake3:af5505659de358234d11377682d608c1499df3d7:43d95ccbcd59e3426b043ee863c9eea28129e844cbfb767b172683ff33eccace")

Reviewer: Some("fresh-session:996a1b8c-e785-4a84-a19e-6d1ae2edf6ec")

Result: pass

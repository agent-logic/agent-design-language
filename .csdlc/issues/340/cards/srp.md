# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
demos/html-observatory/app.js
demos/html-observatory/index.html
adl/tools/test_html_observatory.sh
adl/tools/validate_v092_observatory_restart_reconnect.sh
adl-runtime/tests/runtime_api_wss.rs
.csdlc/issues/340
.csdlc/prepared/issues/340
.csdlc/evidence/340

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

- Reviewer did not rerun live CSMctl/start-stop-restart validation; retained current logs and validator source were inspected instead.
- This review does not claim Unity, #84, #122, #251, AWS/public hosting, provider credentials, #341, or #343 proof.
- The reviewer observed post-HEAD review-assignment metadata dirt and treated the verdict as pinned to immutable assigned revision d14444e7dee84e6c9c6083001e4f16f8f355b592.

## Review Result

Revision: Some("git-blake3:d14444e7dee84e6c9c6083001e4f16f8f355b592:d18f9477f76fa49cd144cb0005f569b9dfc3512ebd7a04b2cc86d8132279d8e1")

Reviewer: Some("fresh-session:27248fe4-c276-4d73-a47a-78801635fca4")

Result: pass

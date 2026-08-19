# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
adl-runtime/src/bin/adl-observatory-static.rs
adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
adl-runtime/tests/runtime_api_wss.rs
adl/tools/test_html_observatory.sh
adl/tools/validate_v092_observatory_restart_reconnect.sh
demos/html-observatory/app.js
demos/html-observatory/index.html
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

- Reviewer did not rerun Cargo or llvm-cov lanes because the review assignment was read-only; retained current proof and source/test contracts were inspected instead.
- The PR #430 coverage fix is test-harness only and scoped to learner_transport::real_four_node_learner_replication leader movement under slow coverage timing.
- This review does not claim Unity, AWS/public hosting, provider credentials, #341, #343, #84, #122, or #251 scope.

## Review Result

Revision: Some("git-blake3:fd55fc517db7e85f71d0c49a0d1628c04fbafa56:cab96b2306fdb9670fb9ca402a18bfcef3d023a0058b703c98ba8aec54772ba1")

Reviewer: Some("fresh-session:fd750059-45b0-438b-9ad3-b6a50389dd7e")

Result: pass

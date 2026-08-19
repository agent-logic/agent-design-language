# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
adl-runtime/src/bin/adl-observatory-static.rs
adl-runtime/tests/runtime_api_wss.rs
adl/tools/validate_v092_observatory_restart_reconnect.sh
.csdlc/issues/340
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

- Reviewer did not rerun live launchctl/Runtime restart or Cargo tests because the review assignment was read-only; retained current evidence and source/test contracts were inspected instead.
- This review does not claim publication, merge, terminal closeout, Unity, AWS/public hosting, provider credentials, #341, #343, #84, #122, or #251 scope.
- The reviewer observed post-HEAD review-assignment metadata dirt and treated the verdict as pinned to immutable assigned revision e42b61e8187e7d5cc176a023ecbc69026c2ebbdc.

## Review Result

Revision: Some("git-blake3:e42b61e8187e7d5cc176a023ecbc69026c2ebbdc:fd1c26da0be4963b2703812a678ef374ff81b8ccbdd9337f56e50b8e7715160e")

Reviewer: Some("fresh-session:b6bbd209-0ef7-4778-8127-e5d654461703")

Result: pass

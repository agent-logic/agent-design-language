# Structured Review Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact #340 launch/restart/reconnect integration changes across CSMctl, start_CSM.sh, runbook/config examples, runtime_api_wss tests, issue-owned validator, lifecycle cards, and evidence; exclude Unity/TLS/AWS/provider/#341/#343 and HTML child redesign.

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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

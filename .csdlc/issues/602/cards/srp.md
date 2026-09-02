# Structured Review Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl/src/cli/csmctl_cmd.rs
docs/api/runtime-v3/v1/observatory.openapi.json
infra/runtime-v3/agents/ember.axioma.yaml
.csdlc/evidence/602/live-wuji-acceptance.md
.csdlc/prepared/issues/602

## Prompts

- Can any unauthorized or conflicting request mutate durable or live roster state?
- Can persistence and in-memory roster truth split after any modeled failure?
- Does restart reload preserve exact admission and reject corrupt state?
- Does csmctl keep credentials out of argv output errors and persisted state?
- Does the live proof preserve Shepherd and avoid init mutation or restart for first add?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Guardian shutdown logged master_log_drain_incomplete during isolated acceptance; clean checkpointed shutdown and agent restoration succeeded, and observability drain behavior is outside issue 602.

## Review Result

Revision: Some("git-blake3:9522142c2ae8a6ce7a29ea39a57d67373b2d4a74:0f9640cc81b5cd0c63d9a3c1ce543548689f8604d01c3dc09a447a2acda01c72")

Reviewer: Some("codex-subagent:issue_602_review")

Result: pass

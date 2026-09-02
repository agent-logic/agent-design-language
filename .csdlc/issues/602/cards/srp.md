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
adl-runtime-kernel/src/conversation_sessions_tests.rs
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

- The exact-head review reran focused conversation semantics and relied on committed live Wuji lifecycle and inference evidence for the remaining acceptance surface.
- Guardian shutdown logged master_log_drain_incomplete during isolated acceptance; clean checkpointed shutdown and agent restoration succeeded, and observability drain behavior is outside issue 602.

## Review Result

Revision: Some("git-blake3:3a913d9e6e7fa49bebf812ddf67351efe8683263:e86974db08898d509906c99bf5eda9cd40c3890580bedb8b862c2bc6df80d60e")

Reviewer: Some("codex-subagent:issue_602_timeout_review")

Result: pass

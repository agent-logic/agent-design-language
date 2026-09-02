# Structured Review Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

CSMctl
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime-kernel/tests/production_acip_wss.rs
adl-runtime-kernel/tests/support/runtime_init.rs
adl/src/cli/csmctl_cmd.rs
docs/api/runtime-v3/v1/observatory.openapi.json
infra/runtime-v3/runtime-init.toml
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

- The final review relied on committed live Wuji acceptance for the live model lane; local focused Runtime, Guardian, API, and lifecycle proof passed.
- Broader Guardian startup simplification is intentionally outside issue 602 and must be handled separately.

## Review Result

Revision: Some("git-blake3:54ab39b1212bd6cbfc501e13b39eb9c9892ec514:784aa1bd7ea6f06ac57b84ae980c1e2199f806266e00e0b26c20a45cc6311386")

Reviewer: Some("codex-subagent:issue_602_production_lease_review")

Result: pass

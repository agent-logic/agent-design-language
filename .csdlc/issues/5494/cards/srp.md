# Structured Review Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/runtime_api_auth.rs
adl-runtime/src/supervision.rs
adl-runtime/src/topology.rs
adl/src/csm_runtime_api.rs
adl/src/long_lived_agent.rs
adl/src/long_lived_agent/tests.rs
docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md

## Prompts

- Does production execute rather than merely describe the assembly?
- Can any required missing or unhealthy observation leave readiness green?
- Does the soak drive real tasks, channels, failure, and recovery?
- Is credential overlap bounded without weakening revocation?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Strict all-target Clippy is blocked by two pre-existing cav.rs test warnings outside #5494.

## Review Result

Revision: Some("git-blake3:d37e3b8f487fd0c7fb92f6d9dcab3d62ebe5e24f:ccbc5313e033bc6543ce01fedb66895d2af979b964551a7b8f51486cbbc8abe8")

Reviewer: Some("subagent:019f747a-3503-7e91-ba2a-6e4f259c1f6b")

Result: pass

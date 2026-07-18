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
adl/src/cli/csmctl_cmd.rs
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

- The hosted authoritative coverage lane must confirm the fifteen-second deadline resolves the observed instrumentation timeout; no AWS validation is required or authorized.

## Review Result

Revision: Some("git-blake3:883406c1904de358bba0b7e8745dd29e5649b65a:05c660fa1a50ad54235fe89faf829aff0fbaca3ea7a2b754a99e2c205ddae5a7")

Reviewer: Some("subagent:019f74ae-d602-7d20-b7ef-e8964d851ea2")

Result: pass

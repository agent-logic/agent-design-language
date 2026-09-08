# Structured Intent Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Create five-minute incremental continuity for every resident agent and asynchronously archive partial checkpoints to S3 with per-agent API and Observatory visibility.

## Required Outcome

Every resident, including the Shepherd, receives an atomic local partial every 300 seconds by default; an independent bounded worker archives partials to private encrypted versioned S3 without coupling cloud health to Runtime readiness.

## Scope

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests
- demos/html-observatory
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/aws/runtime/agent-checkpoint-archive
- .adl/runtime-v3/live/runtime-init.toml
- .csdlc/prepared/issues/695
- .csdlc/issues/695

## Authority

- Issue #695 owns periodic per-agent partial checkpoints, their S3 archive, public continuity projection, and Observatory rendering
- Issue #594 remains authority for Runtime log archival
- Existing full terminal checkpoints and explicit dehydrate/migrate bundles remain authoritative for their own lifecycle boundaries
- Live AWS apply and permanent Runtime rollout are post-review operator actions

## Assumptions

- none

## Operator Constraints

- Default cadence is exactly 300 seconds
- Never block Runtime readiness or agent execution on S3
- Never embed AWS credentials or secrets
- Do not restart or modify the permanent Wuji Runtime during implementation
- Do not create paid cloud resources during repository implementation
- Keep tracked work out of main and use a bound FastWork worktree

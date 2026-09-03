# Structured Intent Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make adding a provider-backed agent to a live Runtime v3 Polis one authenticated, idempotent csmctl command without editing or reloading Runtime init.

## Required Outcome

csmctl agent add validates and durably admits an Ollama-backed agent through Runtime v3, immediately exposes it as healthy and communication-eligible, preserves Shepherd readiness, and survives ordinary Runtime restart.

## Scope

- adl/src/cli/csmctl_cmd.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/Cargo.toml
- docs/api/runtime-v3/v1/observatory.openapi.json
- focused tests and issue-local live evidence

## Authority

- Typed C-SDLC v2 remains lifecycle authority
- Runtime v3 owns admission validation persistence and roster mutation
- csmctl is an authenticated client and never edits Runtime init
- Resident Shepherd startup remains #589 authority
- No paid cloud or multi-node scheduling authority

## Assumptions

- none

## Operator Constraints

- Work only in the bound #602 FastWork worktree and never on main
- Stack on the exact #589 implementation without modifying #589
- Keep the command simple and reliable on first invocation
- Do not load or download unrelated models
- Use focused validation before hosted CI

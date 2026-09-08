# Structured Intent Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Expose one truthful provider/model inference-readiness state across Runtime v3 resident agents and public roster projections.

## Required Outcome

Runtime v3 distinguishes unimplemented, unavailable, model_loading, failed, and ready inference states; only real ready adapters are communication-eligible and placeholders receive no production readiness credit.

## Scope

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/shepherd.rs
- adl-runtime-kernel/tests/assembly.rs
- .csdlc/prepared/issues/687
- .csdlc/issues/687

## Authority

- Issue #687 owns only Runtime v3 provider/model inference-readiness taxonomy and projection
- Issues #640/#653 remain authority for model-backed resident Shepherd execution
- Issues #622/#648 remain authority for provider-profile reload ownership
- Production readiness requires a real implemented adapter and successful inference evidence

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use a bound FastWork issue worktree
- Do not restart, reload, stop, or edit the live Runtime
- Do not call credential-backed providers or mutate cloud resources
- Do not implement a new provider or redesign agent lifecycle

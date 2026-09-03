# Structured Intent Prompt

Template: 1.0.0

Issue: 662

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement and prove governed agent-to-agent initiation as a Runtime-authoritative capability distinct from replying to a user.

## Required Outcome

A model-backed resident Shepherd such as Beacon Axioma can initiate a governed turn addressed to another admitted resident agent such as Ember Axioma, with distinct conversation, turn, sender, recipient, work, and correlation identity; the recipient executes through its configured model/provider and terminal outcomes remain explicit.

## Scope

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/shepherd.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- Observatory/Inspector activity projection surfaces only if existing authoritative events cannot render initiation truth
- .csdlc/prepared/issues/662
- .csdlc/issues/662

## Authority

- Issue #662 owns governed agent-to-agent initiation proof, not the separate Shepherd hardcoded-reply defect
- Runtime remains authoritative for admission, authorization, cancellation, bounded execution, continuity, failure, and correlation identity
- Recipient execution must use configured model/provider behavior; no hardcoded acknowledgement or fixture response receives live credit
- No live Runtime restart or live provider acceptance proof without separate operator authorization

## Assumptions

- none

## Operator Constraints

- Never write tracked files on main
- Use only bound FastWork worktrees under /Volumes/FastWork/adl-worktrees for tracked execution
- Do not write to /private/tmp
- Do not restart or mutate the live Runtime during implementation
- Do not use AWS, paid runners, or live provider calls without explicit operator authorization
- Do not widen into unrestricted autonomous messaging, broadcast, fan-out, or recursive unbounded conversations

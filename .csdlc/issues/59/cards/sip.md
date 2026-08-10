# Structured Intent Prompt

Template: 1.0.0

Issue: 59

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Identify the truthful implementation authority for blocked-goal replacement and route the defect without falsifying historical goal state or inventing an ADL-local substitute.

## Required Outcome

A reviewed typed readiness package proves whether ADL owns the failing transition and, if it does not, names the external authority and exact upstream contract needed before a live canary can close the issue.

## Scope

- .csdlc/prepared/issues/59
- .csdlc/issues/59

## Authority

- Issue authority is agent-logic/agent-design-language#59
- Thread-goal mutation authority is the OpenAI Codex goal-tool service
- ADL owns policy and retained telemetry consumers, not create_goal admission or persistence
- No implementation is authorized unless a repository-owned admission seam is discovered and independently reviewed

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main or under /private/tmp
- Use the typed C-SDLC v2 lifecycle and Rust-only control plane
- Do not mark the historical blocked goal complete
- Do not weaken the ADL issue-bound goal policy
- Do not invent an ADL-local goal service or shadow state
- Do not use AWS

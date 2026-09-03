# Structured Intent Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Wuji start reliably through CSM without a separate Guardian continuity-channel startup dependency.

## Required Outcome

One operator command starts or reloads Wuji, safely reconciles interrupted startup state, reaches stable HTTPS readiness on 20997, and remains supervised by the host service manager.

## Scope

- CSM Runtime v3 service start, stop, status, and reload commands
- Guardian startup readiness and kernel lease ownership
- Safe reconciliation of ownerless locks and interrupted startup journals
- Focused local and AWS-facing readiness proof

## Authority

- Guardian remains the Runtime process authority
- Single-writer safety remains fail closed for genuine live writers
- Retained Polis state and signed checkpoints are preserved
- No separate port-20998 continuity channel is required for ordinary startup

## Assumptions

- none

## Operator Constraints

- Fix forward and do not revert the current recovery work
- Do not discard retained Wuji state
- Keep validation focused and restore the always-on service

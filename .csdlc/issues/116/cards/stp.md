# Structured Task Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only #116 operator attention inbox and intervention workflow; preserve #117 and children as downstream.

## Deliverables

- Runtime attention-request model/queue/lifecycle implementation
- Observatory inbox projection and action surfaces
- Focused runtime and Observatory proof for lifecycle, spoof denial, dedup/rate/expiry, restart/reconnect, and explicit outcomes
- .csdlc/issues/116 lifecycle truth
- .csdlc/prepared/issues/116 authored design, diagram, and validator
- .csdlc/evidence/116 validation evidence

## Acceptance

1. AC-1: Every attention request has stable source identity, reason, correlation, priority, expiry, lifecycle state, and bounded retention metadata.
2. AC-2: Agents cannot fabricate authority, urgency, or another agent identity.
3. AC-3: Rate limits, grouping, deduplication, quiet modes, and retention prevent attention flooding.
4. AC-4: Operator acknowledge, reply, defer, resolve, and refuse outcomes route through governed conversation paths and never imply approval without explicit authority action.
5. AC-5: Observatory exposes live inbox state with unread/read status, filters, deep links, and notification preferences.
6. AC-6: Restart and reconnect preserve actionable requests without duplicate notifications.
7. AC-7: Exact-head review has no unresolved actionable findings; PR CI and terminal finish pass.

## Dependencies

- #111 terminal and ancestral
- #112 terminal and ancestral
- #114 terminal and ancestral
- #115 terminal and ancestral
- #265 terminal and ancestral
- #270 terminal and ancestral
- #271 terminal and ancestral
- #276 terminal and ancestral
- #277 terminal and ancestral
- #278 terminal and ancestral
- Part of #110

## Inputs

- agent-logic/agent-design-language#116
- .csdlc/prepared/issues/116/design.md
- .csdlc/prepared/issues/116/diagram.mmd
- .csdlc/prepared/issues/116/validate_preparation_bundle.py
- Current origin/main after #114 merge

## Non Goals

- Generic system-alert replacement
- Silent automatic approval
- Push-notification vendor integration
- #117 final qualification assembly
- #279/#280/#281/#282 downstream proof bundles
- Reopening #270/#271/#276/#277/#278 semantics

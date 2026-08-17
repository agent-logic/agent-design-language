# Structured Intent Prompt

Template: 1.0.0

Issue: 286

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile terminal evidence for ADR 0069, the Observatory governed Runtime consumer boundary, from WP-18A and WP-18C owners.

## Required Outcome

ADR 0069 has issue-local evidence showing whether terminal Observatory governed Runtime consumer proof exists at exact landed revisions from WP-18A/WP-18C owners, with retained human review and machine-readable outcomes.

## Scope

- Reconcile terminal Observatory governed Runtime consumer evidence from WP-18A owners.
- Reconcile terminal WP-18C Runtime/Observatory integration evidence where the WP-18C graph owns it.
- Verify exact artifact-bound revisions, machine-readable outcomes, and human review evidence are retained.
- Record issue-local evidence and residual gaps for #207.
- Preserve downstream #288 ownership of final ADR index, plan, manifest, and review-packet serialization.

## Authority

- #286 is an ADR evidence-reconciliation issue for #207.d.
- #286 does not implement Runtime, browser UI, Unity, provider, cloud, storage, or authority behavior.
- #286 does not move ADR 0069 to Accepted and does not edit shared ADR index, plan, or manifest surfaces.
- Residual gaps are allowed when exact terminal proof is incomplete, external, credential-bound, or owned by another issue.
- #288 / #207.f owns final serialized ADR index, plan, manifest, and review packet updates.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 owners for lifecycle, review, publication, and finish writes.
- Bind implementation only under /Volumes/FastWork/adl-worktrees.
- Do not mutate sibling ADR children #284/#285/#287/#288 or parent #207/#110 as part of #286.
- Do not read credentials or synthesize live provider/runtime evidence.
- Do not publish until exact-head fresh review passes and required PR checks are green.

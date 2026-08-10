# Structured Intent Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add one typed and fail-closed route for correcting contradictory STP deliverables after review recovery.

## Required Outcome

A recovered implemented issue can replace only its STP deliverables through csdlc-edit with current CAS, recovery provenance, atomic projections, and complete audit evidence.

## Scope

- Typed semantic operation for post-recovery STP deliverable correction
- Lifecycle authorization and recovery-provenance checks
- Atomic projection and audit behavior
- Focused positive and negative Rust proof
- Installation proof allowing issue #73 to consume the operation

## Authority

- Issue #78 owns only the narrow C-SDLC v2 correction route
- Existing exact review, publication, CAS, projection, and lifecycle gates remain authoritative
- Issue #73 owns its architecture and card correction
- C-SDLC v3 robustness changes remain planning work under issue #73

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle and editor routes
- Do not hand-edit rendered cards or canonical issue state
- Do not create a general administrative edit mode or phase rollback
- Run builds only under /Volumes/FastWork
- Do not use AWS
- Do not merge without explicit operator authority

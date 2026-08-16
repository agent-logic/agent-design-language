# Structured Intent Prompt

Template: 1.0.0

Issue: 367

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Bind terminal #365 Shepherd and Observatory sealed committed projections to the same verifier-derived redacted serving-authority lineage so #275 cannot combine two authentic but unrelated stores.

## Required Outcome

A read-only opaque pair verifier accepts only store-derived sealed Shepherd and Observatory projections carrying one identical redacted lineage reference; different authentic lineages, legacy missing lineage, corruption, and caller DTOs fail closed before #275 integration.

## Scope

- Redacted lineage reference derived inside serving_authority from a verified cut
- Shepherd grant projection receipt state and sealed provenance binding
- Opaque two-sealed-child lineage verifier
- Existing focused Shepherd and Observatory tests extended for authentic same-lineage and A/B restart proof

## Authority

- No new raw-lineage getter or raw lineage exposure on child or integration surfaces
- The pair verifier accepts only the two opaque #365 sealed types
- No caller-provided lineage string pairing boolean constructor or DTO conversion
- Existing Shepherd and Observatory transition policies remain unchanged
- #275 and #205 remain frozen until terminal ancestry

## Assumptions

- none

## Operator Constraints

- Exact base a4801fbb3a58bed27ba53367cbda8b31a1f56083
- Own exactly serving_authority.rs Shepherd module and the two existing focused tests
- No Observatory source mod.rs #275 or #205 edit
- Fresh UUID design PASS approval doctor and bind required before source mutation
- No optional paid runner

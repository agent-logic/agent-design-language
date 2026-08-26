# Structured Intent Prompt

Template: 1.0.0

Issue: 275

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deliver a deterministic redacted integrated snapshot across terminal #272, #273, #274, #365, and #367 paired committed authority state, exact receipts, and bounded retry/crash/rollback/corruption/capacity/recovery behavior.

## Required Outcome

A bounded durable snapshot consumes only the #367 opaque borrowed VerifiedCommittedChildLineagePair, emits exact outcome receipts, survives restart deterministically, and fails closed without publishing contradictory or overlapping eligibility.

## Scope

- New integrated serving-authority snapshot module and focused integration target
- One additive distributed module registration line
- Deterministic redacted snapshot and exact success/no-op/rejection/recovery receipts
- Retry crash rollback corruption capacity reconciliation revocation expiry replacement and transfer matrix

## Authority

- #205 remains coordination-only and owns no product implementation
- #275 consumes only terminal #367 opaque borrowed VerifiedCommittedChildLineagePair values returned after committed #273 and #274 store verification and cannot issue or verify authority
- #272 serving_authority.rs, #273 Shepherd source/test, #274 Observatory source/test, and #365 sealed-provenance paths are read-only
- No raw permit membership quorum OwnerCommit lease endpoint secret or caller eligibility boolean is accepted

## Assumptions

- none

## Operator Constraints

- Preparation base is exact origin/main c46b7cd8265a7e81566cdf82153c387595a6cccf
- Use only the clean FastWork preparation root before typed bind
- No product edit before fresh design PASS approval and doctor
- Stop rather than widening into #272 #273 #274 #365 or #205 owned paths
- No optional paid runner

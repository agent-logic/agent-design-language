# Structured Intent Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Expose authority-derived bounded redacted snapshots so issue #5877 can project distributed runtime state without caller self-attestation.

## Required Outcome

Certificate, failure-detection, placement, migration, and recovery authorities expose complete deterministic revisioned redacted snapshots with explicit unavailable rows, restart parity, and fail-closed drift semantics.

## Scope

- Authority-owned snapshot accessors in the five distributed runtime modules
- Monotonic revision or checkpoint changes on authoritative mutation
- Bounded deterministic complete enumeration and explicit unavailable rows
- One focused integration-test surface

## Authority

- Snapshot values are constructed only by the owning authorities
- No private key, signature material, raw probe payload, placement internals, or recovery payload may enter the redacted surface
- Consumers must present the observed revision and fail closed when authority changes before use
- Issue #5877 retains ownership of its three projection files and integration API

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Work only in a bound FastWork worktree
- Do not touch issue #5877 owned files
- Do not modify module registration or manifests unless compilation strictly requires a minimal change
- Merge the exact reviewed green head as soon as CI is green
- Do not delay for asynchronous closeout bookkeeping

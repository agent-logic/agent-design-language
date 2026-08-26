# Structured Intent Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement exact-authority archived-projection cleanup after #298 is terminal and ancestral.

## Required Outcome

A destructive cleanup operation consumes only completed #298 recovery authority, captures archived projection nodes by exact identity/type exchange, removes only receipt-owned inodes with type-correct unlink/rmdir, persists immutable cleanup receipts, and resumes idempotently across every durability boundary.

## Scope

- Exact terminal+ancestral #298 gate verification
- Immutable cleanup receipt ledger outside the private delete namespace
- Operation-owned private deletion namespace and type-matched placeholders
- Per-node exact identity/type archive capture
- Type-correct unlink/rmdir of captured exact inodes only
- Deterministic restart/adoption and idempotent repeat behavior
- Focused cleanup tests and exact-head review

## Authority

- #298 owns anchored classification, recovery receipt construction, verified canonical installation, and archived evidence production
- #299 owns only cleanup after completed #298 authority is terminal and ancestral
- #299 must not mutate #298, #291, #294, #296, or parent #297 terminal truth
- Cleanup authority is receipt-bound, not path-bound, recursive, digest-only, or best-effort

## Assumptions

- none

## Operator Constraints

- Do not bootstrap, bind, edit, test, or run lifecycle for #299 until #298 is terminal and ancestral
- Keep primary main clean during preparation
- Do not touch issue/worktree #298, projection_recovery.rs, store.rs, or gate5.rs until explicit release
- Use typed C-SDLC v2 owners only for lifecycle writes

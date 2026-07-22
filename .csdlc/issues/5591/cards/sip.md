# Structured Intent Prompt

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the canonical independent Runtime v3 kernel accept and execute real domain work through guardian-launched secure typed ingress with deterministic continuity and graceful pressure behavior.

## Required Outcome

At one exact revision, a guardian-launched configured Runtime v3 process accepts representative work through one canonical secure typed ingress, executes production components across bounded channels, emits retained and Observatory-readable evidence, checkpoints/replays/resumes deterministically, and serializes then shuts down gracefully under configured pressure without Runtime v2 dependencies.

## Scope

- canonical Runtime v3 kernel lifecycle, topology, bounded backpressure, service contracts, and configuration
- guardian-launched canonical secure typed local and remote ingress
- deterministic checkpoint, replay, resume, state authenticity, and duplicate-work prevention
- configured pressure detection, admission quiescence, graceful serialization, and bounded shutdown
- Observatory lifecycle, ingress, continuity, pressure, access, health, and terminal output
- negative authority, malformed-input, insecure-access, state-corruption, and pressure-failure proof
- COTS dependency use plus exact-revision source-line, module-growth, and test-count budget proof

## Authority

- #5361 owns integrated Runtime v3 acceptance and #5591 owns Parity-A implementation evidence
- #5336 is the architecture and budget authority and its integration is the sole current stop condition
- adl-runtime-kernel remains the independent canonical Runtime v3 kernel
- #5341 consumes the reviewed canonical ingress contract but does not own it
- Runtime v2 implementation, cutover, deletion, AWS, provider deployment, and new product scope are not authorized

## Assumptions

- none

## Operator Constraints

- Use only typed C-SDLC v2 lifecycle owners; no raw gh, AWS, Runtime v2 implementation files, hard-coded IP addresses, or product edits on main
- Stack #5591 on exact reviewed #5336 head 8fa1bfe66e677ed3ae160b3fee81d204d4211a37 and preserve that dependency as the implementation base
- After preparation validation, amend claim scope through csdlc-bind before touching the smallest collision-free Runtime v3 product paths
- Implementation may proceed on the reviewed stack, but publication, merge, integrated readiness, and final exact-revision acceptance remain blocked until #5336 is merged and #5591 is synchronized with main
- Implement every AC-1 through AC-8 outcome without deferred, skipped, degraded, fixture-only, library-only, prose-only, or partial acceptance
- Use /Volumes/FastWork for Rust build artifacts and maintained COTS crates where practical
- Run bounded subagent review at the exact committed revision, fix every actionable finding, and publish or merge only green work

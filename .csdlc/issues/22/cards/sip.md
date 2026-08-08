# Structured Intent Prompt

Template: 1.0.0

Issue: 22

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the immutable AWS Spot builder execute Ruby-backed repository validators without host-time installation.

## Required Outcome

Pin and provenance-record Ruby in the builder image and fail builder preflight before the requested validation command when Ruby is absent or unusable.

## Scope

- Immutable adl-builder Ruby installation and provenance
- Builder toolchain preflight for Ruby and one repository validator smoke
- Focused builder image and remote validation shell contracts

## Authority

- The official ruby-lang source archive and its verified SHA-256 are installation authority
- The immutable digest-pinned builder image remains validation-tool authority
- No Spot-host package installation is introduced

## Assumptions

- none

## Operator Constraints

- Use FastWork for the issue worktree
- Run focused shell validation only; do not launch AWS or run the broad Rust suite
- Do not touch Sprint 2 or Sprint 3 issues

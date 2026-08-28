# Structured Intent Prompt

Template: 1.0.0

Issue: 501

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one deterministic C-SDLC v3 foundation slice.

## Required Outcome

A deterministic, non-authoritative C-SDLC v3 foundation slice exposing explicit repository context, deterministic state/projection loading, a read-only foundation command, and focused proof for retained requirements #164 through #167.

## Scope

- csdlc-v3/src/bin/**
- csdlc-v3/src/application/**
- csdlc-v3/src/repository/**
- csdlc-v3/tests/foundation.rs
- .csdlc/issues/501/**
- .csdlc/prepared/issues/501/**
- .csdlc/evidence/501/**

## Authority

- C-SDLC v2 remains the sole operational lifecycle authority throughout #501.
- Issue completion is exactly delivery of one deterministic foundation slice.
- The #501 slice is read-only and non-authoritative; it must not perform lifecycle execution, GitHub mutation, publication, finish, cleanup, or authority cutover.

## Assumptions

- #500 is merged and closed before #501 implementation begins.
- The V3-A contract remains a non-authoritative construction boundary.
- Focused tests may be introduced at the exact future harness path and are not proof until they run.

## Operator Constraints

- Execute only after #500 is merged and closed.
- Do not modify C-SDLC v2 behavior or authority.
- Do not start V3-C or later implementation inside this issue.
- Keep the issue-start path simple enough to diagnose and prepare a single issue quickly without bypassing typed v2 authority.

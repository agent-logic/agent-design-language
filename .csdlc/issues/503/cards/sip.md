# Structured Intent Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one end-to-end C-SDLC v3 local preparation workflow.

## Required Outcome

A non-authoritative C-SDLC v3 local preparation workflow from issue input to a doctor-validated PVF plan, with typed contracts, topology binding, active-registry card rendering, and CLI proof for retained requirements #171 through #173.

## Scope

- csdlc-v3/src/commands/local/**
- csdlc-v3/tests/local_commands/**
- docs/templates/prompts/**
- .csdlc/issues/503/**
- .csdlc/prepared/issues/503/**
- .csdlc/evidence/503/**

## Authority

- C-SDLC v2 remains the sole operational lifecycle authority throughout #503.
- Issue completion is exactly one local preparation workflow; individual commands are internal steps and cannot close independently.
- The #503 slice is non-authoritative and must not perform PVF execution, live GitHub mutation, publication, finish, cleanup, v2 migration, or authority cutover.

## Assumptions

- none

## Operator Constraints

- Start #503 only after Sprint 5 leaves #501 and #502 merged/closed out or explicitly dispositioned.
- Keep #503 ordered after V3-C issue #502 and before V3-E issue #504.
- Keep v3 construction-only; do not treat C-SDLC v3 as live lifecycle authority before V3-F issue #505.
- Keep issue-start mechanics simple enough that prepared issue inspection, binding, and first useful work can happen in three minutes or less without bypassing typed v2 authority.

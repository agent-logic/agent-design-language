# Structured Intent Prompt

Template: 1.0.0

Issue: 500

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one reviewed C-SDLC v3 contract and construction-decision packet.

## Required Outcome

A reviewable versioned contract, exact predecessor-coverage matrix, construction decision, and rollback posture for requirements #161 through #163, while C-SDLC v2 remains sole operational authority.

## Scope

- docs/csdlc-v3/**
- csdlc-v3/Cargo.toml
- csdlc-v3/src/lib.rs
- .csdlc/issues/500/**
- .csdlc/prepared/issues/500/**
- .csdlc/evidence/500/**

## Authority

- C-SDLC v2 remains the sole operational lifecycle authority throughout #500.
- Issue completion is exactly acceptance of one v3 contract and construction decision.
- No authority cutover, v2 retirement, or later V3 implementation is authorized.

## Assumptions

- none

## Operator Constraints

- Do not bind or execute during readiness preparation.
- Do not modify C-SDLC v2 behavior or authority.
- Do not touch other Sprint 5 issues or unrelated issue records.

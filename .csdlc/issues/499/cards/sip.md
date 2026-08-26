# Structured Intent Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one behavior-preserving resilience owner-boundary refactoring slice.

## Required Outcome

One refactored resilience module family with explicit owner boundaries and a narrower change-validation surface. Baseline API, positive, negative, fault, trace, retry, timeout, cancellation, formatting, Clippy, and exact diff checks pass while the tracked validation-impact denominator is reduced or truthfully unchanged.

## Scope

- adl/src/resilience.rs
- adl/src/resilience/**
- adl/tests/**
- docs/milestones/v0.92.1/evidence/refactoring/rust-01/**
- .csdlc/prepared/issues/499

## Authority

- Issue completion is exactly one behavior-preserving resilience owner-boundary refactor; module extraction and test relocation are internal steps and line movement is not a separate result.
- Issue authority is agent-logic/agent-design-language#499
- No adjacent sprint or provider authority

## Assumptions

- none

## Operator Constraints

- No paid cloud or provider mutation
- No secrets, legal instruments, auth codes, or recovery factors in Git

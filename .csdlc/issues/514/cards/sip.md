# Structured Intent Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one shared provider inference-profile contract with deterministic Ollama materialization.

## Required Outcome

One shared provider inference-profile contract with deterministic Ollama materialization. Schema, materialization, invalid-profile, last-known-good, and redaction checks pass.

## Scope

- adl/src/provider/**
- adl-runtime/src/provider/**
- docs/provider/**
- docs/milestones/v0.92.1/evidence/provider/prov-a/**
- .csdlc/prepared/issues/514

## Authority

- Issue completion is exactly one shared provider-profile contract; provider-specific checks are evidence inputs.
- Issue authority is agent-logic/agent-design-language#514
- No adjacent sprint or provider authority

## Assumptions

- none

## Operator Constraints

- No paid cloud or provider mutation
- No secrets, legal instruments, auth codes, or recovery factors in Git

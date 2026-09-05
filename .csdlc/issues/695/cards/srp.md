# Structured Review Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Five-minute per-agent partial checkpoint cadence, local atomic storage and restore, asynchronous S3 spool/archive, per-agent Runtime API fields, Observatory rendering, Terraform security boundary, and focused tests only.

## Prompts

- Can a slow snapshot or S3 outage block Runtime readiness, conversations, or shutdown?
- Does every resident including Shepherd receive one ordered partial per cadence without overlap?
- Can any partial be replayed or restored across agent, polis, Runtime, or full-checkpoint lineage?
- Can local retention discard the only unarchived recoverable state or grow without bound?
- Do API and Observatory show backing model and truthful snapshot/archive freshness without infrastructure or secret leakage?
- Does Terraform enforce private encrypted versioned storage and least privilege without live apply?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review

# Structured Review Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

five-minute per-agent partial checkpoint cadence
atomic local persistence and bounded retention
asynchronous S3 spool, archive, and restore
Runtime API and Observatory continuity projection
AWS Terraform archive boundary
focused exact-head validation

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

- Live AWS Terraform apply and permanent Wuji rollout are intentionally deferred and are not claimed as issue #695 proof.
- Acceptance results remain bound to substantive implementation head b79b2cfab; later reviewed commits are typed lifecycle, evidence, and publication metadata only.

## Review Result

Revision: Some("git-blake3:d6b01cd29a1d313049354ef93b3ed4542daab5dc:cbeffa06f26502eb280839be13bd843cf9794e57aeec002bb16acf7315900acc")

Reviewer: Some("fresh-session:ee27eb24-a81c-4ebd-8161-d30b4f225ed9")

Result: pass

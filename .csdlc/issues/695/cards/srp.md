# Structured Review Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

in-flight snapshot removal fencing
sequence-aware archive projection and degraded-state consistency
bounded AWS CLI child execution and scheduler cleanup
non-mutating manifest-owned acceptance evidence
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

- Live AWS Terraform apply and permanent Wuji rollout remain intentionally deferred and are not claimed as issue #695 proof.

## Review Result

Revision: Some("git-blake3:466a6da85bc1fabe3165adbcf83c2e59ee5df2e9:2b0238cb99040711e1c6313ec83c309691e8cc1264214b8c358e40579fa47f34")

Reviewer: Some("fresh-session:ee27eb24-a81c-4ebd-8161-d30b4f225ed9")

Result: pass

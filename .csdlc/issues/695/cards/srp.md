# Structured Review Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

strict-clippy remediation using boxed partial records and a restore-context type alias
behavior preservation for partial checkpoint encode, restore, archive, and tombstone paths
exact cargo clippy all-targets warning-denial gate
18 focused partial-checkpoint tests and eight guardian soak tests

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

- Live AWS Terraform apply and permanent Wuji rollout remain explicitly deferred and are not claimed by this PR.

## Review Result

Revision: Some("git-blake3:36c5fb981159724b0d2867189996df12d95dd881:e5cb91a03d7e006d7dc504b2cd60d30593f3aad5c97abd323645a7121b77b235")

Reviewer: Some("fresh-session:ee27eb24-a81c-4ebd-8161-d30b4f225ed9")

Result: pass

# Structured Review Prompt

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
adl/src/long_lived_agent/tests.rs
adl/tests/csm_runtime_v3_generation.rs

## Prompts

- Is receipt identity byte-stable across independently resolved package graphs?
- Can any mismatched init, generation, executable, or receipt pass?
- Does live proof distinguish operator reply from a separately addressed and received Ember work item?
- Is rollback preserved throughout deployment?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review is limited to the assigned three-file semantic scope and focused checkpoint/restore regression; broader Runtime validation remains represented by the existing retained proof and CI.

## Review Result

Revision: Some("git-blake3:5842eb2072e8a7806037ad5f30394fb00f859a59:1b99284574e87c3d9716af7fc49b3388e356064854be0172bdb198bb95d6e8be")

Reviewer: Some("subagent:/root/review_711_multipart_restore")

Result: pass

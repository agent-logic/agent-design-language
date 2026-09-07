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

[
  {
    "id": "711-p1-multipart-a2a-checkpoint-loss",
    "severity": "p1",
    "summary": "Parent conversation checkpoints persisted only the optional legacy A2A message and could lose multipart or parts-only delegated content across restore.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Multipart A2A checkpoint and restore behavior requires correction and exact-head re-review.

## Review Result

Revision: Some("git-blake3:58d80fb0a647f179493f7a1b9cfd9107cb7f61f2:dc4e7284bbcb2dd9818f03baeb1ad7b1f13550b79971b171e9814857c28f4559")

Reviewer: Some("subagent:/root/review_711_restacked_head")

Result: changes_required

# Structured Review Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Review exact issue #510 HOT-01 runtime hot-reload changes across adl-runtime/src/config_reload.rs, adl-runtime/src/lib.rs, adl-runtime/tests/config_reload.rs, docs/runtime/config-hot-reload.md, prepared validators, validation evidence, and PR publication readiness. Reject partial snapshots, restart-required reload, invalid-update activation, missing debounce, lingering watcher tasks, stale review, merge/closeout, or DEC-01 #513 scope absorption.

## Prompts

- Does the implementation atomically swap complete configuration snapshots for readers?
- Does invalid update content preserve the last-known-good configuration without restart?
- Are file events debounced in production behavior and proven by focused tests?
- Can concurrent readers ever observe partial or mixed configuration state?
- Does the watcher shut down cleanly without lingering tasks?
- Is DEC-01 #513 clearly gated from concurrent edits to the #510 runtime files?

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

# Structured Review Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/config_reload.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/control.rs
adl-runtime/src/lib.rs
adl-runtime/tests/config_reload.rs
docs/runtime/config-hot-reload.md

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

Revision: Some("git-blake3:da2f0e9e7bf66a2ac16050644ce6eb376ae4fde1:292158182e5101f26956b13819d3d306d3f961ba1b7985733cc6eba365626eca")

Reviewer: Some("openai-responses:gpt-5.6-sol:resp_0e13a64dc209853a006a8f46af92a887d0bf83c464fa1289bb")

Result: pass

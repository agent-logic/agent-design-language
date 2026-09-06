# Structured Review Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_orientation.rs
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/control.rs
demos/html-observatory/app.js
demos/html-observatory/tests/agent_orientation.test.mjs

## Prompts

- Can any admitted agent reach its first model turn without the active orientation snapshot?
- Does the recorded digest cover the exact delivered bytes rather than a mutable source or global resource?
- Can reload misreport the package delivered to an existing agent?
- Can invalid content replace the last valid active package?
- Does any wording or control path let orientation enlarge authority?
- Is the implementation smaller than a general prompt framework?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:06cc4a8b2893d50a0a05b7331489e206d314f38c:d3f08aa984a45ba9b53944a62a28c35e36f2f6eb7fb46d87a29d81fb6baef914")

Reviewer: Some("codex:issue-708-current-agent-exact-head-audit")

Result: pass

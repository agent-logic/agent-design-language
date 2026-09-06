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
adl-runtime-kernel/src/resident_shepherd.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/shepherd.rs
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

- Review was source/read-model focused and did not perform live provider inference or paid Runtime execution; retained validation covers local Rust, protocol, and Observatory behavior.

## Review Result

Revision: Some("git-blake3:e8ffad7903cc635b69885019ca12f301704e23b9:9f4fc9b9d68b892ad76913c0e9989c083258b3db562430b15a6cfaa03e688eba")

Reviewer: Some("codex:issue-708-exact-head-review")

Result: pass

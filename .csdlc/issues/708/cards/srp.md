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
.csdlc/issues/708/index.json
.csdlc/issues/708/cards/srp.md
.csdlc/prepared/issues/708/review-assignment-request.json
.csdlc/prepared/issues/708/review-record-exact-head-pass.json

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
- The branch tail after source review contains only typed C-SDLC review/projection artifacts; current-tail review inspected those artifacts and found no P0-P3 actionable findings.

## Review Result

Revision: Some("git-blake3:85eed43ae3ee34b720be585d94d7a47bee2ec7ee:5ee6a0ff3bfeac4d1bcdc22b4d9603679377df43b0a9d16f4aa9e053006c29b4")

Reviewer: Some("codex:issue-708-exact-head-review-current-tail")

Result: pass

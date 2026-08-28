# Structured Review Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/Cargo.toml
csdlc-v3/Cargo.lock
csdlc-v3/src/lib.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/local/mod.rs
csdlc-v3/tests/local_commands.rs
.csdlc/issues/503
.csdlc/prepared/issues/503
.csdlc/evidence/503

## Prompts

- Do local preparation commands consume typed contracts and emit typed outputs without becoming live lifecycle authority?
- Does bind modeling require registered topology and reject branch-name-only authorization?
- Do cards render from the active prompt-template registry without hand edits?
- Does the doctor/PVF planning proof distinguish ready, blocked, failed, deferred, skipped, and passed outcomes?
- Does the issue-start simplification reduce ceremony while preserving typed v2 authority, validation, and review gates?
- Can any #503 surface be misread as C-SDLC v3 operational authority before V3-F?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- Issue #504 remote delivery, #570 docs/skills readiness, #571 V3-A corrective proof, and #505 authority cutover remain separate downstream gates.
- The later publication step must visibly include `Closes #503`; no v3 command is authorized to publish, finish, clean, mutate GitHub, or replace v2 authority before V3-F.

## Review Result

Revision: Some("git-blake3:57cf571dcbd72608aa98e65879548f2b1807fb41:38d223198ce70769d5994a7aeb79b53cbaa62b8fe6b4fbed878cc7f463a38d52")

Reviewer: Some("fresh-session:issue-503-exact-head-review")

Result: pass

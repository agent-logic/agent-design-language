# Structured Review Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/510/concurrent-read.log
.csdlc/evidence/510/debounce.log
.csdlc/evidence/510/invalid-retention.log
.csdlc/evidence/510/valid-reload.log
.csdlc/evidence/510/watcher-shutdown.log
.csdlc/issues/510/audit.jsonl
.csdlc/issues/510/cards/sip.md
.csdlc/issues/510/cards/sip.values.json
.csdlc/issues/510/cards/sor.md
.csdlc/issues/510/cards/sor.values.json
.csdlc/issues/510/cards/spp.md
.csdlc/issues/510/cards/spp.values.json
.csdlc/issues/510/cards/srp.md
.csdlc/issues/510/cards/srp.values.json
.csdlc/issues/510/cards/stp.md
.csdlc/issues/510/cards/stp.values.json
.csdlc/issues/510/cards/vpp.md
.csdlc/issues/510/cards/vpp.values.json
.csdlc/issues/510/index.json
.csdlc/locks/510.lock
.csdlc/prepared/issues/510/design.md
.csdlc/prepared/issues/510/diagram.mmd
.csdlc/prepared/issues/510/validate-concurrent-read.rb
.csdlc/prepared/issues/510/validate-debounce.rb
.csdlc/prepared/issues/510/validate-invalid-retention.rb
.csdlc/prepared/issues/510/validate-valid-reload.rb
.csdlc/prepared/issues/510/validate-watcher-shutdown.rb
adl-runtime/src/config_reload.rs
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

- Re-review did not rerun Cargo tests because the reviewer remained read-only; implementation session reran cargo test, clippy, and all five prepared validators before this review.

## Review Result

Revision: Some("git-blake3:4a88c582e6e47cdd897407cd77b968b5e3316f22:7d206e268eb23961ef9bfd8fb0e45c43593f59d43a86fc4b02b53c16477e5de1")

Reviewer: Some("fresh-session:codex-510-hot-01-rereview")

Result: pass

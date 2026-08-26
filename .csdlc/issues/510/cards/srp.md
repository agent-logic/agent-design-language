# Structured Review Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/510/audit.jsonl
.csdlc/issues/510/cards/sip.values.json
.csdlc/issues/510/cards/sor.values.json
.csdlc/issues/510/cards/spp.values.json
.csdlc/issues/510/cards/srp.values.json
.csdlc/issues/510/cards/stp.values.json
.csdlc/issues/510/cards/vpp.values.json
.csdlc/issues/510/index.json
.csdlc/prepared/issues/510/assign-final-api-review.json
.csdlc/prepared/issues/510/publish-final.json
.csdlc/prepared/issues/510/record-final-api-review.json
.csdlc/prepared/issues/510/recover-publication-review.json

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

Revision: Some("git-blake3:65b1c38de88b127c5b34ef64bcb5c3b78433a174:df31705b5a8c45d7b09bd18f8e7d86e8bb90fcdf345b33fe374d777243661ed5")

Reviewer: Some("openai-responses:gpt-5.6-sol:resp_07dc4f89eb9ab044006a8f48c2c82487d0b9caebc41d268624")

Result: pass

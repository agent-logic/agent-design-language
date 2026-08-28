# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/551/audit.jsonl
.csdlc/issues/551/index.json
.csdlc/issues/551/cards/sip.values.json
.csdlc/issues/551/cards/stp.values.json
.csdlc/issues/551/cards/spp.values.json
.csdlc/issues/551/cards/vpp.values.json
.csdlc/issues/551/cards/srp.md
.csdlc/issues/551/cards/srp.values.json
.csdlc/issues/551/cards/sor.values.json
.csdlc/prepared/issues/551/assign-r2-exact-review.json
.csdlc/prepared/issues/551/record-r2-exact-review.json

## Prompts

- Does validation reject an advertised Observatory origin that the combined CORS and WSS policy would not accept?
- Do REST and WSS default to the existing v2 contract, explicitly project v1 and v3, and reject unsupported schema selectors?
- Does one validated reload atomically replace every Polis parameter and Runtime presentation consumer without restart?
- Do invalid reloads preserve the complete last-known-good snapshot with bounded redacted diagnostics?
- Does HTML explicitly request v3 and render only feed-owned identity values?
- Is Unity absent from the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- The exact-head review did not perform a live browser, external network, or deployed TLS exercise; local Runtime TLS/WSS and HTML proof remain the bounded pre-publication evidence.

## Review Result

Revision: Some("git-blake3:7bf1277277e6b64c54840ff0336120d2545a5984:d65338c5630d1f3f383abf3fdbb35ac63a76016e503ff036ead8713d3647184d")

Reviewer: Some("fresh-session:8f7bb69d-260b-4c67-a4b6-6c91df40c2ed")

Result: pass

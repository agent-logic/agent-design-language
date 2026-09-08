# Structured Review Prompt

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/512/CLAUDE_REVIEW_HANDOFF.md
.csdlc/evidence/512/exact-browser-operator-proof.md
.csdlc/evidence/512/exact-browser-operator-screenshot.png
.csdlc/evidence/512/system-clock-cache-proof.md
adl/tools/test_html_observatory.sh
demos/html-observatory/README.md
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/runtime-v3.config.json
demos/html-observatory/styles.css
demos/html-observatory/tests/accessibility_responsive.test.mjs
demos/html-observatory/tests/agent_continuity.test.mjs
demos/html-observatory/tests/agent_orientation.test.mjs
demos/html-observatory/tests/agent_to_agent_activity.test.mjs
demos/html-observatory/tests/conversation_sessions.test.mjs
demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
demos/html-observatory/tests/operator_attention_inbox.test.mjs
demos/html-observatory/tests/polis_identity.test.mjs
demos/html-observatory/tests/security_privacy_adversarial.test.mjs
demos/html-observatory/tests/system_clock_cache.test.mjs

## Prompts

- Were #511 and #536 terminal before execution?
- Does the UI consume authentic Runtime projections?
- Are recovery and accessibility cases proven rather than visually asserted?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:a57e551a14089a6ed53de05f7ff88041880d175e:b533483bf0b4056d8319f20bb4a30109de3367149c73d535c5899c347f5cc89a")

Reviewer: Some("subagent:review-512-clock-cache-final")

Result: pass

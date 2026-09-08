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
adl/tools/test_html_observatory.sh
demos/html-observatory/README.md
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/runtime-v3.config.json
demos/html-observatory/styles.css
demos/html-observatory/tests/accessibility_responsive.test.mjs
demos/html-observatory/tests/agent_to_agent_activity.test.mjs
demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
demos/html-observatory/tests/security_privacy_adversarial.test.mjs

## Prompts

- Were #511 and #536 terminal before execution?
- Does the UI consume authentic Runtime projections?
- Are recovery and accessibility cases proven rather than visually asserted?

## Findings

[
  {
    "id": "512-review-executable-keyboard-proof",
    "severity": "p1",
    "summary": "The retained screenshot and regex-based tests did not execute or prove keyboard focus, panel selection, hash navigation, or screen-reader behavior.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "512-review-handoff-verdict-stale",
    "severity": "p2",
    "summary": "The review handoff retained an obsolete changes-required verdict that contradicted its current passing validation and remediation checklist.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:2a905edb805df09a46097a1353f0b30e5b970d7a:73fbeca6e74894b296c23e7d03042590ce038196e5d8622d22061955c1e9d39e")

Reviewer: Some("codex:issue-512-operator-proof-review")

Result: changes_required

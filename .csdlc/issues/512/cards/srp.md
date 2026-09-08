# Structured Review Prompt

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/512/CLAUDE_REVIEW_HANDOFF.md
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
    "id": "512-review-exact-browser-accessibility-proof",
    "severity": "p1",
    "summary": "The declared exact-head interactive browser and accessibility lane remains unproved because the Codex browser controller cannot initialize and the retained validator only checks static file and prose presence.",
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

Revision: Some("git-blake3:3b12d2a0cf55e9b180ee3f9b90025e2e2c2e72fb:f0b2be7ef11d73ca6ced44a952e0c1b013494ebeb97113a5021cfe01e87f87a1")

Reviewer: Some("codex:issue-512-publication-head-review")

Result: changes_required

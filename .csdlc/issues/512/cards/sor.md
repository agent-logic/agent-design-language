# Structured Output Record

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the Runtime v3 Observatory redesign with live polis switching, a responsive Panopticon and agent directory, chronological conversation and A2A activity, operator-access controls, recovery states, and exact per-agent orientation provenance.

## Artifacts

- .csdlc/evidence/512/CLAUDE_REVIEW_HANDOFF.md
- adl/tools/test_html_observatory.sh
- demos/html-observatory/README.md
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/styles.css
- demos/html-observatory/tests/accessibility_responsive.test.mjs
- demos/html-observatory/tests/agent_to_agent_activity.test.mjs
- demos/html-observatory/tests/security_privacy_adversarial.test.mjs

## Execution

- Reworked the Observatory information architecture, responsive styling, navigation, event stream, Panopticon, agent directory, inspector, and chat surfaces.
- Added Runtime v3 polis switching with polis-scoped token, roster, conversation, inspector, event, and component state isolation.
- Added accessible inspector tab semantics and keyboard navigation while preserving existing Runtime v3 orientation and Layer 8 projections.
- Added focused JavaScript tests for accessibility, responsive behavior, continuity, orientation, A2A activity, conversation sessions, recovery, operator attention, polis identity, and security/privacy boundaries.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Issue 512 exact-range diff hygiene",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "purpose": "Issue 512 Layer 8 Observatory compatibility",
    "outcome": "passed",
    "evidence_ref": "observatory-layer8-authority.log"
  },
  {
    "command": [
      "node",
      "--test",
      "demos/html-observatory/tests/accessibility_responsive.test.mjs",
      "demos/html-observatory/tests/agent_continuity.test.mjs",
      "demos/html-observatory/tests/agent_orientation.test.mjs",
      "demos/html-observatory/tests/agent_to_agent_activity.test.mjs",
      "demos/html-observatory/tests/conversation_sessions.test.mjs",
      "demos/html-observatory/tests/large_polis_performance_recovery.test.mjs",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs",
      "demos/html-observatory/tests/polis_identity.test.mjs",
      "demos/html-observatory/tests/security_privacy_adversarial.test.mjs"
    ],
    "purpose": "Issue 512 complete Observatory JavaScript suite",
    "outcome": "passed",
    "evidence_ref": "observatory-node-suite.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_html_observatory.sh"
    ],
    "purpose": "Issue 512 Runtime v3 Observatory contract",
    "outcome": "passed",
    "evidence_ref": "observatory-runtime-v3-contract.log"
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_observatory_transcript_history.mjs"
    ],
    "purpose": "Issue 512 transcript history compatibility",
    "outcome": "passed",
    "evidence_ref": "observatory-transcript-history.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none

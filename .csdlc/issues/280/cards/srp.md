# Structured Review Prompt

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

demos/html-observatory/app.js
demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
.csdlc/prepared/issues/280/validate_preparation_bundle.py
.csdlc/issues/280
.csdlc/evidence/280

## Prompts

- Does #280 stay limited to large-Polis performance/recovery proof and narrowly necessary Observatory fixes?
- Are #279 accessibility/responsive, #281 security/privacy/adversarial, #282 final qualification, #117, and #110 explicitly excluded?
- Do validation lanes prove large roster, long transcript, bounded metrics, backpressure, reconnect/restart/offline/version mismatch, public-safe evidence, and exact revision without credentials?
- Could any browser behavior introduced here grant authority, synthesize acknowledgements, mask refusal, or cache stale authorization?
- Are all dependency merge gates truthfully required before implementation and publication?

## Findings

[
  {
    "id": "6134-p2-missing-latency-resource-metrics",
    "severity": "p2",
    "summary": "AC-2 is unmet: the proof records fixture counts and limits but no latency or resource measurements.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6134-p2-missing-json-evidence-and-candidate-revision",
    "severity": "p2",
    "summary": "AC-4 is unmet: SOR claims .csdlc/evidence/280/large_polis_performance_recovery_metrics.json but the assigned commit lacks it, and candidate_revision resolves to fixture incarnation rather than Git revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6134-p2-transcript-window-not-integrated",
    "severity": "p2",
    "summary": "AC-1 is not proven for transcripts because retainedLargePolisWindow is only exercised by synthetic test/evaluator and not integrated into the browser transcript or DOM-facing projection path.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6134-p2-recovery-back-to-healthy-not-proven",
    "severity": "p2",
    "summary": "AC-3 is not proven: the test asserts degraded labels and a truncated action list but never exercises recovery back to healthy state, reconnect/restart execution, or absence of hidden stale state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6134-p2-prep-validator-base-not-assigned-revision",
    "severity": "p2",
    "summary": "The preparation validator and evidence validate pre-implementation HEAD 557dd28d85746a8dc5109dcc674f5a606b8c9890, not the assigned substantive revision.",
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

- Publication is held until all actionable findings are remediated and a different fresh reviewer passes the new immutable head.

## Review Result

Revision: Some("git-blake3:5a2b82d33259e3ee4dc5d0c56debeb4cdde10b2e:1ad837bb66e9c5a86241d9771088743f86554d1ccde6b7d28907e6f3e1d3a904")

Reviewer: Some("fresh-session:613498c9-7e79-4ad7-84d5-848b4e7714eb")

Result: changes_required

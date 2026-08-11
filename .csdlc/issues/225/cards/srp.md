# Structured Review Prompt

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/225
.csdlc/issues/225
.csdlc/prepared/issues/225
csdlc-v2/src/cards.rs
csdlc-v2/src/review.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Can either operation mutate any field, phase, or card outside its exact contract?
- Does SPP correction prove a real typed review recovery and cleared stale truth?
- Does SIP correction remain pre-bind and invalidate stale design approval?
- Do audit and real editor tests prove complete old/new values and atomic projections?
- Are all later phases, retained-truth states, and direct-edit paths still rejected?

## Findings

[
  {
    "id": "225-review-p1-recovery-actor",
    "severity": "p1",
    "summary": "Review recovery accepted a blank actor; actor and reason are now both required before mutation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:120455414b9a457f735a6f177742780d185a3b8b:1d6d41964c0fcc493d39f18f757d1025118c8f9f40da751808c01e492a3abeb9",
    "route": null
  },
  {
    "id": "225-review-p1-retained-sor-state",
    "severity": "p1",
    "summary": "Pre-bind correction did not reject every retained SOR lifecycle state; integration, publication, merge, and closeout state are now all guarded.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:120455414b9a457f735a6f177742780d185a3b8b:1d6d41964c0fcc493d39f18f757d1025118c8f9f40da751808c01e492a3abeb9",
    "route": null
  },
  {
    "id": "225-review-p2-proof-matrix",
    "severity": "p2",
    "summary": "The declared proof matrix omitted six authorization boundaries; Gate 2 and Gate 5 now prove every accepted and rejected case explicitly.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:120455414b9a457f735a6f177742780d185a3b8b:1d6d41964c0fcc493d39f18f757d1025118c8f9f40da751808c01e492a3abeb9",
    "route": null
  },
  {
    "id": "225-review-p2-plan-truth",
    "severity": "p2",
    "summary": "SPP execution steps lagged implementation truth; all four steps are now reconciled through typed edits.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:120455414b9a457f735a6f177742780d185a3b8b:1d6d41964c0fcc493d39f18f757d1025118c8f9f40da751808c01e492a3abeb9",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The two correction operations are intentionally field-specific; any future correction surface requires its own typed authorization contract.

## Review Result

Revision: Some("git-blake3:120455414b9a457f735a6f177742780d185a3b8b:1d6d41964c0fcc493d39f18f757d1025118c8f9f40da751808c01e492a3abeb9")

Reviewer: Some("Bernoulli:019fefc0-cd6b-7072-84d5-9fc5b78de4d5")

Result: pass

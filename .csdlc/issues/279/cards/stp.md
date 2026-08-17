# Structured Task Prompt

Template: 1.0.0

Issue: 279

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bootstrap, design-review, bind, implement, prove, review, publish, CI, and finish only #279 Observatory accessibility/responsive UX proof for the integrated candidate; do not absorb #280/#281/#282 or parent coordination scope.

## Deliverables

- .csdlc/prepared/issues/279/design.md
- .csdlc/prepared/issues/279/diagram.mmd
- .csdlc/prepared/issues/279/validate_preparation_bundle.py
- .csdlc/evidence/279
- .csdlc/issues/279
- demos/html-observatory/tests/accessibility_responsive.test.mjs
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css

## Acceptance

1. AC-1: Keyboard-only traversal reaches roster, chat/composer, rooms, history/transcripts, and attention surfaces where present without traps, hidden focus, or order that contradicts visual workflow.
2. AC-2: Core Observatory controls and state regions expose meaningful labels, roles, landmarks, selected/busy/error/delivery/refusal/read/unread state, and screen-reader-facing text without leaking private cognition or credentials.
3. AC-3: Reduced-motion mode avoids unnecessary animation and preserves state comprehension.
4. AC-4: Desktop and mobile breakpoints preserve reachable controls, readable transcript/history/attention content, stable composer behavior, and no horizontal overflow for the proof fixtures.
5. AC-5: Contrast-sensitive tokens/classes used by the touched surfaces meet the issue's documented threshold or are recorded as residual risks with exact selector evidence.
6. AC-6: Proof artifacts are deterministic, credential-free, public-safe, and tied to one exact candidate revision.
7. AC-7: Any source changes are Observatory presentation/test only and do not alter Runtime authority, acknowledgement, room routing, history durability, performance/recovery, or security/privacy semantics.
8. AC-8: Fresh exact-head review has no unresolved actionable findings, required CI is green on the published head, and typed finish derives terminal authority before #282 consumes #279.

## Dependencies

- #111 terminal cache canonical and merge ancestral
- #112 terminal cache canonical and merge ancestral
- #113 terminal cache canonical and merge ancestral
- #114 terminal cache canonical and merge ancestral
- #115 terminal cache canonical and merge ancestral
- #116 terminal cache canonical and merge ancestral
- #265 terminal cache canonical and merge ancestral
- #270 terminal cache canonical and merge ancestral
- #271 terminal cache canonical and merge ancestral
- #276 terminal cache canonical and merge ancestral
- #277 terminal cache canonical and merge ancestral
- #278 terminal cache canonical and merge ancestral
- #117 live parent remains coordination-only
- #110 live parent remains coordination-only

## Inputs

- agent-logic/agent-design-language#279
- agent-logic/agent-design-language#117
- agent-logic/agent-design-language#110
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- demos/html-observatory/tests
- terminal caches for #111/#112/#113/#114/#115/#116/#265/#270/#271/#276/#277/#278

## Non Goals

- Runtime authority, signing, acknowledgement protocol, room routing, durable history, receipt, retention, or security-policy changes
- Large-Polis performance/recovery proof owned by #280
- Security/privacy/adversarial proof owned by #281
- Final production qualification assembly owned by #282
- Parent #117 or #110 implementation/closeout
- Cloud/public deployment, Unity feature implementation, provider credentials, or paid/optional jobs

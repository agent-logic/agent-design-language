# Issue 279 design: Observatory accessibility and responsive UX proof

## Boundary

#279 is a proof and presentation issue for the existing Runtime-backed HTML Observatory. It may add deterministic accessibility/responsive validation and narrow browser presentation fixes when proof shows they are required. It must not change Runtime authority, signed-message semantics, delivery policy, durable-history behavior, security/privacy proof, performance/recovery proof, final qualification assembly, or parent coordination truth owned by #280, #281, #282, #117, or #110.

## Inputs and dependency gate

The issue starts only after the integrated Observatory candidate children are terminal and canonical:

- #111 canonical human-agent conversation sessions.
- #112 shared Layer 8 signed authority core.
- #113 live Polis roster and agent presence.
- #114 durable conversation-history coordination.
- #115 governed multi-agent rooms and routing.
- #116 operator attention inbox and intervention workflow.
- #265 Runtime kernel ingress enforcement.
- #270 trusted recipient-acknowledgement Runtime API protocol.
- #271 Observatory authority-state and delivery-state UI integration.
- #276 durable conversation journal foundation.
- #277 watermarks, idempotency, replay, and receipts.
- #278 re-authorized history APIs and transcript restoration.

Preparation and implementation validators must read canonical terminal caches rather than stale root projections, and they must fail closed if any required cache is missing, noncanonical, or not ancestral to the current base.

## Proof model

The primary deliverable is a deterministic local Observatory proof that exercises accessibility and responsive behavior without credentials, cloud deployment, Unity, provider calls, or private Runtime state. The proof should use browser/DOM-level fixtures for the exact HTML Observatory surface and record:

- keyboard traversal and focus visibility for roster, conversation, authority/delivery state, attention inbox, filters, and transcript controls;
- roles, names, status semantics, landmark structure, and screen-reader-relevant labels for dynamic Runtime states;
- contrast and reduced-motion expectations where they can be verified deterministically from CSS/class state;
- responsive desktop and narrow viewport behavior, including no unintended horizontal overflow, preserved visible controls, and usable transcript/inbox layouts;
- failure-state visibility for refusal, timeout, disconnect/reconnect, stale data, and no-selection states as rendered UI, not invented Runtime authority.

If deterministic proof exposes a small presentation defect in `demos/html-observatory`, #279 may fix it only in the HTML/CSS/JS/test surface needed for accessibility or responsive UX. Runtime behavior and sibling proof domains remain out of scope.

## Validation lanes

1. `observatory-accessibility-responsive-proof`: run the issue-owned Node/browser-style proof for keyboard, semantics, focus, reduced-motion, contrast, and responsive layout assertions.
2. `observatory-ui-regression`: rerun the existing focused Observatory UI tests touched by the change.
3. `diff-hygiene`: `git diff --check`.
4. `exact-head-review`: fresh no-context review of exact immutable #279 head before publication.
5. `github-required-ci`: ordinary required GitHub checks after typed publication.

All evidence must name the exact Git revision under test and distinguish local deterministic proof from deferred GitHub CI.

## Publication and downstream handoff

The PR must close only #279. #282 may consume #279 evidence only after #279 is terminal and ancestral. Any unresolved product/security/performance findings must route to #280, #281, #282, or a follow-on rather than being hidden in #279.

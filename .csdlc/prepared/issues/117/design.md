# Issue #117 Design: Observatory Product Hardening And Production Proof

## Outcome And Authority

Issue #117 is the final integrated hardening and proof child for the v0.92 HTML
Observatory. At one exact candidate revision, the roster, one-to-one conversation,
durable history, governed rooms, and attention inbox must operate together against
the real Runtime. The browser remains a projection and command client; Runtime,
Layer 8 policy, identity, routing, replay, history, and intervention contracts
remain authoritative.

This issue adds no new governance or Runtime authority. It integrates and hardens
the terminal outputs of #83 and #111 through #116, proves product behavior, and
records explicit degraded states when an authoritative dependency is unavailable.
Issue #122 is deferred public exposure and is not a gate, input, or execution
surface for #117.

## Integrated Security And Privacy Contract

All rendered agent, operator, message, room, history, receipt, refusal, attention,
and diagnostic content is untrusted. Dynamic content must use safe text rendering
or an equivalently reviewed sanitizer; no Runtime or provider content may become
HTML, script, URL, style, or event-handler authority. TLS and allowed-origin
assumptions, authentication token storage and transport, cache behavior, request
correlation, replay keys, and redaction boundaries must be explicit and tested.

The browser may request only actions already authorized by the Runtime contract.
Stale roster entries, history, room membership, inbox items, reconnect buffers,
deep links, browser storage, and optimistic UI state never grant identity,
recipient, intervention, or replay authority. Confused-deputy substitution,
cross-Polis identity reuse, wildcard recipients, stale revisions, duplicate
submission, version mismatch, and malformed or oversized content fail closed with
public-safe diagnostics. Screenshots, browser traces, console logs, retained
fixtures, and the artifact index must contain no credentials, signing material,
private cognition, raw provider payloads, or unauthorized agent state.

## Accessibility And Responsive Contract

The complete interface is keyboard operable with deterministic focus order,
visible focus, skip/navigation semantics, non-trapping dialogs, and focus recovery
after send, refusal, route change, reconnect, and item removal. Native semantics
are preferred; names, roles, states, relationships, validation errors, delivery
updates, reconnect status, and urgent attention changes must be meaningfully
exposed to assistive technology without noisy duplicate announcements.

Color contrast, non-color state cues, zoom and text resizing, reduced motion,
touch target size, reflow, landscape/portrait behavior, and narrow/mobile layouts
are acceptance surfaces. Responsive changes may reorder presentation but must not
hide recipient identity, policy refusal, delivery state, unread state, or recovery
controls. Desktop and mobile proofs use deterministic viewports and machine-
readable assertions in addition to screenshots.

## Resilience And Resource Contract

Network loss, Runtime restart, stream interruption, backpressure, offline mode,
version mismatch, partial feature degradation, and browser reload produce explicit
states. Recovery must re-establish authoritative snapshots and cursors before
claiming continuity. It must not duplicate sends, interventions, messages,
receipts, unread counts, room events, or attention acknowledgements. Stale data is
visibly stale and cannot silently become current.

Large rosters, long transcripts, room fan-out, inbox growth, reconnect queues,
event gaps, rendered DOM, response bodies, retries, and retained diagnostics have
declared bounds. The proof records latency, memory, DOM-node, stream-buffer, retry,
and recovery budgets and fails on overflow or zero-test selection. Long-run proof
must exercise reconnect and Runtime restart against a real local Runtime candidate,
not fixture substitution.

## Affected-Area Ownership

Issue-owned execution paths:

- `adl/tools/validate_v092_html_observatory_hardening.mjs`
- `docs/milestones/v0.92/features/OBSERVATORY_PRODUCT_HARDENING.md`
- `docs/milestones/v0.92/runbooks/OBSERVATORY_OPERATOR_RUNBOOK.md`
- `.csdlc/evidence/117`

Shared integration paths, editable only after #83 and #111 through #116 are
terminal, merged where applicable, ancestral to the execution base, and no active
owner remains:

- `adl-runtime-kernel/tests/observatory.rs`
- `adl-runtime-kernel/tests/openapi_contract.rs`
- `docs/api/runtime-v3/v1/observatory.openapi.json`
- `demos/html-observatory/app.js`
- `demos/html-observatory/index.html`
- `demos/html-observatory/styles.css`
- `adl/tools/test_html_observatory.sh`

Preparation owns only `.csdlc/issues/117` and `.csdlc/prepared/issues/117`. Issue
#83, umbrella #110, dependencies #111 through #116, deferred #122, and all product
paths remain read-only during this setup.

## Terminal Gates And Handoff

Do not bind or edit product code until #83 and every implementation child #111,
#112, #113, #114, #115, and #116 are terminal, their merged revisions are
ancestors of the selected execution base where applicable, and their shared-path
ownership is released. Re-read #117 and reconciled #110, inspect the exact handed-
off contracts, then update SPP and VPP through typed edits before binding if any
path, schema, test, or budget changed. #122 is expressly excluded from this gate.

## Validation Plan

- Existing exact Runtime control, Observatory, and OpenAPI targets provide the
  preparation-time nonzero denominator.
- Execution adds an issue-owned live browser validator for integrated roster,
  chat, history, rooms, inbox, accessibility, responsive, adversarial, reconnect,
  restart, scale, backpressure, offline, and redaction proof.
- Exact security assertions cover XSS/content rendering, origin and TLS posture,
  token handling, replay, stale authority, confused deputy, denial, and artifact
  hygiene.
- Exact accessibility assertions cover keyboard, focus, semantics, announcements,
  contrast, zoom/reflow, reduced motion, screen-reader exposure, and mobile views.
- Strict focused Clippy, diff hygiene, deterministic screenshots, clean console,
  bounded resource metrics, and independent exact-head product, architecture, and
  security review all apply to one candidate revision.

Product lanes remain deferred during preparation. Typed card validation and doctor
prove design-time readiness only, never product behavior.

## Failure Policy And Non-Goals

Fail closed on any nonterminal gate, ownership collision, fixture substitution,
browser-owned authority, unsafe rendering, secret or private-state leakage,
inaccessible core workflow, hidden degradation, duplicate action, stale-state
confusion, unbounded resource growth, missing real-browser proof, failed exact
lane, or unresolved exact-head finding.

No public deployment, cloud provisioning, AWS work, Unity work, new Runtime or
governance authority, product implementation during preparation, push, PR,
publication, merge, closeout, #110 mutation, dependency mutation, or #122 work is
in scope.

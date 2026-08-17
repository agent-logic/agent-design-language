# Structured Task Prompt

Template: 1.0.0

Issue: 271

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later implement only #271 Observatory presentation/adapter and focused proof; #278 history restoration remains downstream.

## Deliverables

- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- adl/tools/validate_layer8_authority_observatory_ui.sh
- .csdlc/evidence/271/validate_exact_three_path_scope.py
- .csdlc/evidence/271/authentic-handler-output.json
- .csdlc/prepared/issues/271
- .csdlc/issues/271
- .csdlc/evidence/271

## Acceptance

1. AC-1: #112/#265/#270 terminal caches validate canonical and their merges are ancestral to the execution base.
2. AC-2: Observatory consumes only the served #270 response schema and never treats request acceptance as delivery.
3. AC-3: Delivered, refused, failed, revoked, and recovery states are distinct, visible, accessible, and action-safe.
4. AC-4: Raw correlation identifiers, signed messages, private keys, proof hashes, provider payloads, policy details, and opaque signed request/acknowledgement material do not render, log, cache, or persist.
5. AC-5: Post-bind scope validation compares the bound branch from its exact execution base to current main and accepts exactly three product/test paths: demos/html-observatory/app.js, demos/html-observatory/styles.css, and adl/tools/validate_layer8_authority_observatory_ui.sh, plus issue-local lifecycle/evidence surfaces.
6. AC-6: The browser wrapper exercises the actual Observatory assets through the literal eight-case exact set: delivered, signed refusal, malformed response failure, unavailable Runtime recovery, revoked demotion, action release, keyboard/live-region accessibility, and forbidden-field non-disclosure; every case must be nonzero and zero/ignored/skipped/missing/duplicated cases fail closed.
7. AC-7: Existing real adl-runtime-kernel recipient_ack handler proof parses a nonzero denominator and covers delivered acknowledgement, signed refusal, generation binding, malformed/unknown status rejection, and raw-correlation redaction without adding a new non-issue-local script.
8. AC-8: Browser fixtures consume a source-grounded authentic handler-output artifact handoff stored under .csdlc/evidence/271 from the runtime proof; loopback-only or mocked conversation-frame evidence is not accepted.
9. AC-9: Fresh design review and fresh exact-head implementation review have no actionable findings before publication.

## Dependencies

- #112 terminal canonical and ancestral
- #265 terminal canonical and ancestral
- #270 terminal canonical and ancestral
- #278 remains downstream and blocked until #271 terminal

## Inputs

- agent-logic/agent-design-language#271
- agent-logic/agent-design-language#112
- agent-logic/agent-design-language#265
- agent-logic/agent-design-language#270
- agent-logic/agent-design-language#278
- docs/api/runtime-v3/v1/openapi.json
- adl-runtime-kernel/src/control.rs
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- historical nonpublication reference e0fd2364

## Non Goals

- Runtime authority or acknowledgement protocol changes
- Kernel ingress enforcement
- Durable transcript/history/receipt persistence
- #278 transcript restoration
- Browser signing, policy, local signature verification, or private-key storage
- Rendering, logging, caching, or persisting opaque signed request/acknowledgement material
- Cloud/public exposure
- Mutating or binding #114
- Mutating #112 worktrees or replaying historical e0fd mock evidence

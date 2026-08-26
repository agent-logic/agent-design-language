# Issue #271 Design — Layer 8 Authority and Delivery State in Observatory

## Outcome

Observatory consumes the terminal #270 served recipient-acknowledgement protocol and presents `delivered`, `refused`, `failed`, and recovery-safe states without becoming an authority source. The browser never creates or holds private keys. Runtime-provided signed request/acknowledgement artifacts may exist only ephemerally as opaque POST inputs to the served verifier; they are never rendered, logged, cached, or persisted. Raw correlation identifiers, proof hashes, provider payloads, and private policy detail likewise never render or persist.

## Authority boundary

Runtime remains the sole verifier and authority. The browser forwards only a Runtime-provided, already-signed request/acknowledgement pair through the configured Runtime API base and renders the public response projection. It does not originate either signed message, verify signatures locally, infer delivery from request acceptance, invent acknowledgements, cache authorization as authority, or convert refusal into delivery. The opaque POST input is discarded after the request settles.

## Product scope

- `demos/html-observatory/app.js`: add a bounded client adapter for `/v1/layer8/recipient-acknowledgement`, validate the public response schema/status, map delivery/refusal/failure distinctly, and retain only the redacted correlation hash.
- `demos/html-observatory/styles.css`: ensure the communication surface and refusal/failure states remain visible and accessible.
- `adl/tools/validate_layer8_authority_observatory_ui.sh`: real-browser deterministic proof over the actual Observatory assets. Any loopback transport is permitted only as a transport harness fed by authentic #270 handler-output artifacts retained under `.csdlc/evidence/271`; loopback-only or mocked conversation-frame proof fails closed.
- Issue-local C-SDLC cards, evidence, design, diagram, and validator surfaces.

`demos/html-observatory/index.html`, Runtime protocol implementation, OpenAPI, durable history, and #278 transcript restoration remain read-only inputs unless a fresh reviewed replan explicitly widens scope.

## Runtime contract

The adapter uses `POST /v1/layer8/recipient-acknowledgement` and accepts only schema `adl.runtime_v3.layer8.recipient_acknowledgement_response.v1`. A response is delivery only when Runtime returns `status=delivered`; `refused` remains refusal and `failed` remains failure. The UI may show the public `correlation_hash`, credential generations, message identifiers, and bounded error classification only where the served schema declares them public. Raw correlation IDs and opaque signed request/acknowledgement material never enter DOM text, logs, URL/query state, session/local storage, IndexedDB, or other durable browser storage.

## Proof

The proving browser test loads the repository's actual HTML, JavaScript, and CSS through repo-local ephemeral HTTPS. It consumes authentic #270 handler-derived public response fixtures from `.csdlc/evidence/271` and rejects loopback-only or mocked conversation-frame evidence. It exercises exactly eight nonzero cases: delivered, signed refusal, malformed response failure, unavailable Runtime recovery, revoked demotion, action release, keyboard/live-region accessibility, and forbidden-field non-disclosure; zero, ignored, skipped, missing, or duplicated cases fail closed. Static contract checks bind the exact #270 route/schema and reject browser-side signing or policy logic. Diff scope proves only the three product/test paths plus issue-local lifecycle surfaces changed.

## Dependencies

#112, #265, and #270 must have canonical terminal caches whose merge commits are ancestral to the execution base. #278 remains downstream and must not be implemented here. The historical `e0fd2364` candidate is inspection-only input and grants no review or publication authority.

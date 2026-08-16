# #270 design: trusted recipient acknowledgement Runtime API protocol

## Scope

#270 owns the trusted recipient-acknowledgement Runtime API protocol after #112 defines shared Layer 8 authority and #265 enforces that authority at Runtime kernel conversation ingress. This child defines and serves a production Runtime route that verifies acknowledgement provenance before side effects, binds credential-generation signatures to the acknowledged message context, redacts correlation safely, and distinguishes refusal from delivery.

This design packet was originally bootstrap/design-only. #112 and #265 are now terminal and ancestral, so #270 may bind and implement only the trusted recipient-acknowledgement Runtime API/protocol slice described here.

## Dependency posture

Execution is gated on terminal and ancestral #112 core-authority truth plus terminal and ancestral #265 Runtime ingress enforcement truth. Current observations show #112 terminal at merge SHA `6172bfb067bd45ec231fbc2635e7efbb718ef415` and #265 terminal at merge SHA `301080a40c91c6882f34fead3c742524467c056d`, both ancestral to current `origin/main`. #270 is bind-ready after refreshed design/card review approves this updated packet.

## Boundary

#270 consumes #112 authority semantics and #265 ingress enforcement. It does not redefine Layer 8 authority, implement kernel ingress enforcement, implement #115 room/UI behavior, implement Observatory/UI state, own durable transcript storage or acknowledgement-watermark persistence, or create cloud/public exposure.

## Proposed implementation shape

1. Define a versioned recipient-acknowledgement Runtime API request/response contract.
2. Serve the acknowledgement contract through a production Runtime route, not a fixture-only adapter.
3. Verify server-authenticated acknowledgement provenance before side effects.
4. Bind credential-generation signatures to the message, authority, recipient, replay, and conversation context.
5. Preserve refusal and delivery as distinct visible outcomes without leaking secrets, private cognition, raw provider payloads, or unsafe correlation.
6. Prove correlation redaction and production-route API behavior with focused tests.

## Validation plan

Design/bind-readiness validation for this packet:

- `python3 .csdlc/prepared/issues/270/validate_preparation_bundle.py`
- `csdlc-doctor --repo <repo> --issue 270`
- `csdlc-validate --root <repo> issue --issue 270`
- fresh design/card review before refreshed design approval and bind

Implementation validation after bind:

- focused Runtime API route tests
- acknowledgement provenance and verify-before-side-effects tests
- credential-generation binding tests
- refusal-versus-delivery and correlation-redaction tests
- strict relevant Clippy
- exact-head independent review before publication

## Stop conditions

- #112 or #265 terminal cache fails validation or is not ancestral to the execution base.
- Proposed scope redefines #112 authority or #265 ingress enforcement instead of consuming them.
- Proposed scope absorbs #115 room/UI behavior, Observatory/UI, durable transcript storage, acknowledgement-watermark persistence, or cloud exposure.
- Bind creates a branch/worktree outside the typed #270 FastWork route or mutates #112/#265.
- Design review finds unresolved actionable issues.

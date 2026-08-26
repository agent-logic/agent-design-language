# Structured Intent Prompt

Template: 1.0.0

Issue: 270

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Design and later implement the trusted recipient-acknowledgement Runtime API protocol after #112 and #265 are terminal and ancestral.

## Required Outcome

Runtime exposes a production served route for recipient acknowledgements that verifies acknowledgement provenance before side effects, binds credential-generation signatures to message and authority context, redacts correlation safely, and distinguishes delivery from refusal.

## Scope

- Versioned trusted recipient-acknowledgement Runtime API protocol
- Production Runtime served route for acknowledgements
- Verify acknowledgement provenance before side effects
- Credential-generation signature binding to message and authority context
- Refusal-versus-delivery distinction and correlation redaction proof after dependencies clear

## Authority

- #112 owns shared Layer 8 signed authority primitives and identity-message contract
- #265 owns Runtime kernel conversation ingress enforcement before #270 served route behavior
- #270 owns recipient acknowledgement Runtime API/protocol only
- #270 does not own #115 room/UI behavior, Observatory/UI, durable transcript storage, acknowledgement-watermark persistence, or cloud exposure

## Assumptions

- #112 is terminal and ancestral to current main through merge SHA 6172bfb067bd45ec231fbc2635e7efbb718ef415.
- #265 is terminal and ancestral to current main through merge SHA 301080a40c91c6882f34fead3c742524467c056d.
- #270 remains open, ready, unbound, and scoped to trusted recipient-acknowledgement Runtime API/protocol work.

## Operator Constraints

- Bind and implement #270 only after validating #112 and #265 terminal caches and ancestry against current main.
- Use the typed v2 lifecycle route and a FastWork worktree; no raw GitHub lifecycle writes.
- Do not mutate #112 parent/prep, #265, #271, #115, #114, #276, #277, or #278.
- Do not absorb Observatory/UI, durable transcript storage, acknowledgement-watermark persistence, multi-agent room/UI behavior, cloud/public exposure, or Runtime kernel ingress enforcement.
- Runtime product/test/docs changes must remain limited to the trusted recipient-acknowledgement Runtime API/protocol scope.

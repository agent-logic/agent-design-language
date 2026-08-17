# #276 design: durable conversation journal foundation

## Scope

#276 owns the durable storage foundation for conversation history after Layer 8 authority and acknowledgement prerequisites are available. The child defines the journal schema, storage boundary, versioning, migrations, corruption/partial-write recovery, and bounded retention/deletion primitives. It does not own acknowledgement-watermark semantics, replay reconciliation, public history APIs, Observatory restoration, or parent-level integrated proof.

## Dependency posture

This packet is refreshed after all declared prerequisites became terminal and ancestral to the execution base:

- #112 provides the shared Layer 8 signed authority core.
- #265 enforces Layer 8 authority at Runtime kernel conversation ingress.
- #270 defines and serves trusted recipient-acknowledgement Runtime API protocol.

Because all three prerequisite issues now have canonical derived-terminal cache entries whose merge SHAs are ancestral to current `origin/main`, #276 may proceed to typed bind after fresh design/readiness review PASS and typed design approval. Implementation remains limited to #276 durable journal foundation scope.

## Boundary

The journal foundation may store only authority-validated conversation events supplied by upstream Runtime authority surfaces. It must not create an alternate authority model, acknowledgement protocol, replay policy, public API contract, or Observatory state machine.

## Proposed implementation shape after gates clear

1. Define a versioned durable journal record model for authorized conversation events, receipt references, retention/deletion markers, migration metadata, and corruption-recovery evidence.
2. Add a storage boundary that supports atomic append/commit semantics and deterministic restart loading without exposing partial writes as committed history.
3. Add forward-only schema migration support with exact version gates and typed corruption outcomes.
4. Add bounded retention/deletion primitives with auditable outcomes while preserving authority and receipt coherence requirements for downstream #277/#278.
5. Prove restart, migration, corruption, retention, and deletion foundation behavior with focused deterministic tests.

## Validation plan

Design/bootstrap validation for this packet:

- `csdlc-doctor --repo <repo> --issue 276`
- `csdlc-validate --root <repo> issue --issue 276`
- fresh design/card review before design approval

Future implementation validation, deferred until dependency gates are terminal:

- focused durable journal schema/storage tests
- migration/corruption/partial-write tests
- retention/deletion outcome tests
- strict relevant Clippy
- exact-head independent review before publication

## Stop conditions

- #112, #265, or #270 lacks canonical terminal cache authority or is not ancestral to the execution base.
- Proposed changes redefine Layer 8 authority, acknowledgement trust, replay/watermark policy, public history APIs, or Observatory behavior.
- Bootstrap creates a branch/worktree or mutates #114/#277/#278.
- Design review finds unresolved actionable issues.

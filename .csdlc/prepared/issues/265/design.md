# #265 design: Runtime kernel Layer 8 authority ingress enforcement

## Scope

#265 owns the Runtime kernel conversation-ingress enforcement layer after #112 defines the shared Layer 8 signed authority core. This child wires #112 authority checks into production kernel assembly/control/ingress so unauthorized, revoked, replayed, or scope-escalated conversation attempts are refused before delivery or any conversation side effects.

This design packet is bootstrap/design-only. It does not implement Runtime product code while #112 remains open.

## Dependency posture

Execution is gated on #112 terminal and ancestral core-authority truth. Current typed reads show #112 open and no local `.git/csdlc-v2/derived-terminal/112.json`, so #265 is not bind-ready or implementation-ready.

## Boundary

#265 consumes the #112 authority primitives; it does not define them. It also does not own the #270 recipient acknowledgement served API/protocol, #115 multi-agent room behavior, Observatory/UI behavior, durable transcript storage, cloud exposure, or new browser-held signing authority.

## Proposed implementation shape after #112 terminal

1. Identify the production Runtime kernel conversation ingress boundary and the earliest control/assembly point before delivery or provider side effects.
2. Insert #112 authority verification at that boundary, preserving per-principal identity, signed authority semantics, revocation/refusal/replay outcomes, and audit redaction rules.
3. Ensure unauthorized, revoked, replayed, stale-generation, or scope-escalated attempts fail closed before side effects.
4. Record refusal/audit outcomes without secrets, private cognition, or raw provider payloads.
5. Prove the production conversation boundary rather than a fixture-only path.

## Validation plan

Design/bootstrap validation for this packet:

- `python3 .csdlc/prepared/issues/265/validate_preparation_bundle.py`
- `csdlc-doctor --repo <repo> --issue 265`
- `csdlc-validate --root <repo> issue --issue 265`
- fresh design/card review before design approval

Future implementation validation, deferred until #112 terminal:

- focused Runtime kernel ingress authorization/refusal/replay tests
- production conversation-boundary proof
- strict relevant Clippy
- exact-head independent review before publication

## Stop conditions

- #112 remains open, lacks derived-terminal cache, or is not ancestral to the execution base.
- Proposed scope redefines #112 authority primitives instead of consuming them.
- Proposed scope absorbs #270 served API/acknowledgement protocol, #115 room/UI behavior, Observatory/UI, durable transcript storage, or cloud exposure.
- Bootstrap creates a branch/worktree or mutates #112/#270.
- Design review finds unresolved actionable issues.

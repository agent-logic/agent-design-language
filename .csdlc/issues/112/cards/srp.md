# Structured Review Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src
adl-runtime/src/layer8_authority.rs
adl-runtime/tests/layer8_authority.rs
adl/src/csm_runtime_api.rs
adl/tests/layer8_authority_runtime_api.rs
adl/tools/validate_layer8_authority_observatory_ui.sh
demos/html-observatory
docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md
.csdlc/evidence/112
.csdlc/issues/112

## Prompts

- Can authentication, browser state, caller claims, content, provider output, or agent self-report authorize any action without an exact current capability and policy intersection?
- Can contact, continuation, attachment, or single-recipient authority widen into another action, conversation, recipient set, room, broadcast, or Polis?
- Do replay, expiry, rotation, revocation, restart, audit corruption, and concurrent duplicate requests fail before sequence reservation or provider execution?
- Do all projections omit content, attachment bytes, secrets, private policy, provider payloads, and private cognition?
- Does the dedicated real-browser Observatory contract render authorized, refused, and stale or revoked states truthfully, keep refused actions unavailable, and disclose only audience-approved projection fields?
- Is #111 the sole serial gate, is #83 retained only as preserved source, and are all three exact product targets deferred without preparation surrogate proof?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Recipient acknowledgement uses the in-process agent execution boundary and filesystem-backed keys, not external transport or HSM proof.
- Browser evidence is local deterministic presentation proof; the fresh review rerun was sandbox-limited and relied on retained exact-head PASS evidence.

## Review Result

Revision: Some("git-blake3:3b0b5db556e79a9d114f694a2bf31c2fe678b801:648acd88a09c73d198f6eac2bca61df250c6acdcee9ecdd9c474bc9f90b01aff")

Reviewer: Some("fresh-subagent-112-final-pass")

Result: pass

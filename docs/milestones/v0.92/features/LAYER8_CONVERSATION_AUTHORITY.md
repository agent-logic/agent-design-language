# Layer 8 Conversation Authority

- Milestone: `v0.92`
- Work package: `WP-18C.02`
- Issue: `#112`
- Dependency contract: merged `#111` Runtime conversation session and HTML
  Observatory feed/UI contracts
- Focused proof: `bash adl/tools/validate_layer8_authority_observatory_ui.sh`

## Product boundary

The Runtime owns Layer 8 conversation authority. The Observatory presents
Runtime decisions; browser login state, selected agents, typed content,
provider output, and agent self-report do not grant or widen authority.

The existing `#111` conversation schemas remain the carrier contract:

- `adl.runtime_v3.observatory_conversation_intent.v1` submits one bounded turn.
- `adl.runtime_v3.observatory_conversation_result.v1` reports its Runtime-owned
  acceptance, terminal outcome, and audience-approved reply.
- `adl.runtime_v3.observatory_conversation_cancel.v1` requests cancellation of
  that exact accepted turn.
- `adl.runtime_v3.observatory_ws_control_result.v1` reports authentication and
  revocation state for the write channel.

This issue-owned presentation contract does not add browser authority, change
the Runtime schemas, or modify the `#111` UI implementation.

## Production identity and policy inputs

The live Runtime fails closed unless both `ADL_LAYER8_AUTHORITY_PROFILE` and
`ADL_LAYER8_SIGNING_PROFILE` name readable JSON profiles outside repository
state. The authority profile supplies current authenticated identity evidence,
pre-existing capabilities, and separate agent and polis policies. A request is
matched against those grants; it cannot manufacture a request-shaped grant or
declare its own credential generation.

The signing profile maps the sender and each recipient principal to a key id
and an external Ed25519 private-key file. Key files contain one hex-encoded
32-byte secret and are read only during Runtime initialization. They must not
be stored in the repository, browser storage, Observatory payloads, audit
records, or rendered output. The profile polis id must match the live Runtime
instance.

For every authorized delivery the Runtime constructs and verifies the shared
signed ACIP request before dispatch. It reports `delivered` only after a
recipient-key acknowledgement verifies against the request's exact sender,
recipient, polis, conversation, correlation, and causation identity. Missing,
expired, malformed, revoked, substituted, or mismatched evidence refuses or
fails the conversation without exposing signing material.

## Authorized presentation

An operator turn becomes visible only after a matching Runtime result proves
acceptance. A terminal result with a positive `turn_sequence` may carry the
same acceptance proof when the accepted frame and terminal frame are coalesced.
The match is exact over conversation, turn, recipient, and correlation IDs.

`accepted` keeps the turn pending and exposes cancellation for that turn.
`delivered` terminates the turn and may present only the bounded `reply` field.
Neither browser submission nor a WebSocket send is displayed as acceptance.

## Refused presentation

`refused` is terminal and does not prove acceptance. The Observatory presents
a Runtime-owned outcome and a bounded refusal code, does not reveal the
operator message as an accepted turn, does not render an agent reply, removes
the turn from pending state, and leaves no action available for that refused
turn.

The same terminal and no-reply posture applies to `failed`, `timed_out`, and
`cancelled`. A malformed, mismatched, or unknown result frame is ignored rather
than being presented under another turn.

## Stale or revoked presentation

A disconnected pending turn may replay only against the same proved Runtime
incarnation. When the incarnation changes, the Observatory presents
`restart_unavailable`, marks the old turn terminal, removes its cancellation
control, and never sends the retained intent to the successor incarnation.

`credential_revoked`, `authentication_failed`, and
`write_authentication_required` remove conversation write access. The
Observatory returns to `public read`, disables conversation submission, and
requires fresh Runtime authentication before another governed action can be
sent. Browser-retained credentials never preserve authority after Runtime
revocation.

## Disclosure boundary

The operator audience may see only the selected recipient label, accepted
operator text, bounded status or refusal code, correlation-bound turn state,
and an audience-approved delivered reply. Presentation must not expose:

- bearer tokens, credentials, signing material, or private keys;
- capability or private-policy contents;
- provider requests, responses, or internal result hashes;
- attachment bytes or undisclosed message content;
- private cognition, audit internals, or cross-audience projections.

Transcript content is assigned through DOM `textContent`, so audience-approved
text is presented as text rather than interpreted markup. Result hashes and
other proof internals are not transcript fields. Refusal codes are bounded
Runtime vocabulary, not arbitrary private error payloads.

## Deterministic proof

`validate_layer8_authority_observatory_ui.sh` is the sole focused proof for
this worker slice. It uses no network, provider, cloud, live Runtime, or soak
surface. The validator:

1. Executes the merged `#111` transition helpers with fixed authorized,
   refused, and stale fixtures.
2. Verifies acceptance-gated transcript rendering and terminal cleanup in the
   actual Observatory source.
3. Verifies changed-incarnation refusal and credential-revocation demotion.
4. Verifies refused actions remain unavailable and internal proof or secret
   fields do not reach transcript rendering.
5. Fails when the issue-owned product contract or required conversation UI
   elements are absent.

This proof establishes deterministic browser/static presentation behavior. It
does not claim live Runtime authorization, provider execution, cloud delivery,
or the separate Rust and Runtime API authority proofs owned by the rest of
issue `#112`.

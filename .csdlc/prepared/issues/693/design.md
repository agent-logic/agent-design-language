# Issue 693 design — reliable model-backed A2A action selection

Status: reviewed design candidate.

## Problem

The governed A2A primitive is already authoritative. The reliability defect is
one layer earlier: production provider output is interpreted as an action only
when the model reproduces an exact JSON envelope. Ordinary provider prose falls
through to the operator reply path, so a model can describe contacting a peer
without any A2A work being created.

## Decision

Use a Runtime-owned, typed action channel in the provider conversation result.
The provider adapter returns an explicit response object whose action variant is
selected by the provider contract, rather than scanning arbitrary reply prose.
The Runtime translates only that typed variant into the existing governed
initiation intent. Free-form text remains an operator-facing reply and never
authorizes dispatch.

Where an HTTP/model provider supports native tool or function calls, normalize
that provider-native structured action into the typed result. The deterministic
test provider must exercise the same normalization boundary with ordinary
assistant prose plus a separate provider-native action signal; tests must not
inject the final Runtime initiation envelope.

## Execution flow

1. Production conversation ingress resolves the admitted sender and provider.
2. The provider receives the user message and bounded roster/action context.
3. The provider adapter returns either `Reply(text)` or
   `Action::InitiateAgent { recipient, message, idempotency_key }`.
4. The Runtime resolves canonical identities and constructs the existing signed
   or authenticated initiation intent; provider output cannot supply authority.
5. Existing admission, eligibility, Layer8, replay, cancellation, and failure
   gates accept or reject the request.
6. Accepted work executes through the recipient's configured provider route.
7. Initiation, acceptance, recipient result, and terminal outcome retain their
   distinct identities and appear in the authoritative feed.
8. An operator-facing reply is emitted independently; it cannot stand in for
   dispatch or recipient completion.

## Compatibility and failure behavior

- Keep valid explicit action-envelope compatibility if currently public, but
  route it through the same typed normalization boundary.
- Never infer an action from prose, markdown fences, or a claim of delivery.
- Invalid recipient aliases fail before dispatch and do not create false
  completion events.
- Existing idempotency keys and cancellation paths remain Runtime-owned.
- Provider failure before action selection is an ordinary provider failure;
  recipient failure after accepted dispatch is an A2A terminal failure.

## Proof strategy

Build an isolated end-to-end test that enters through production conversation
ingress, uses a deterministic provider response containing natural assistant
prose and a distinct provider-native action signal, dispatches Beacon to Ember,
executes Ember through its configured deterministic provider, and observes
correlated A2A activity and terminal result. Run it repeatedly. Retain focused
tests for authorization, missing/stale recipient, replay, cancellation, and
provider failure, plus the pre-existing #662 primitive tests.

## Explicit boundaries

No live Wuji restart, cloud call, paid provider, transcript-history work,
unbounded autonomy, broadcast, recursive fan-out, or changes to #686/#689.

# Issue 694 design — complete conversation history restoration

## Decision

Runtime-owned conversation history is the only reload authority. Each accepted
human-to-agent turn is retained as two ordered, role-explicit history entries:
the operator submission and the terminal agent reply. Both carry stable
conversation and turn identity; the reply additionally retains its work and
recipient identity. `ingress.completed` remains completion telemetry and is not
used to reconstruct missing operator text.

The authenticated Observatory connection requests bounded
`conversation_history.v1` after its initial snapshot. A fresh client invokes
the existing `restoreConversationTranscriptFromRuntimeHistory` path, merges
history by stable entry identity, and then accepts live frames through the same
deduplication boundary. This avoids a race-dependent second transcript source.

## Authority and privacy

- History is emitted only after the existing Observatory authentication and
  conversation visibility checks succeed.
- Only operator-visible roles and public reply content enter the response;
  prompts, provider payloads, private memory, tool arguments, credentials, and
  internal reasoning remain excluded.
- Page limits and replay bounds are enforced by Runtime before serialization.
- Unknown roles, malformed identifiers, revoked access, and ambiguous ordering
  fail closed.

## Proof strategy

Focused Runtime tests prove ordered complete records, authorization, redaction,
bounds, and replay identity. Observatory tests prove fresh-load invocation and
history/live deduplication. One isolated end-to-end acceptance uses production
history serialization and restoration code to submit operator text, receive a
generated reply, discard UI state, restore history, and observe both ordered
halves exactly once. It uses isolated state and never touches the permanent
Wuji Runtime.

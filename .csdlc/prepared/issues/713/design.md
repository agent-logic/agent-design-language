# #713 — Complete A2A conversation history

## Design

Treat an agent-to-agent exchange as one durable causal conversation record, not as unrelated delivery assertions. The Runtime-authoritative record binds sender and recipient identities, outbound content, recipient reply, status, timestamps, and conversation/turn/work/correlation identifiers. The same projection serves authenticated API recovery and Observatory reconstruction after reconnect or restart.

The path is symmetric for every admitted agent and remains governed by signed ACIP identity, replay protection, authorization, redaction, and audit policy. Checkpoint and rehydration preserve the same history without duplication or reordering.

## Boundaries

- Extend existing Runtime conversation/A2A history; do not create a parallel transcript authority.
- Do not add Shepherd-specific behavior.
- Do not expose secrets, hidden prompts, credentials, or provider-private data.
- Preserve #707 delivery behavior and keep #713 changes independently reviewable.


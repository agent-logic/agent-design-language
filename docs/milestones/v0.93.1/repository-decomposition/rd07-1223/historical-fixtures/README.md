# Retained durable-history audit fixture

This is the byte-identical historical Runtime integration test read by ADL's
WP-18C issue #114 terminal-chain audit. It is retained as text for its ten
original marker checks. It is not a compiled test target or evidence of current
Runtime behavior.

Original path: `adl-runtime-kernel/tests/durable_conversation_history_integration.rs`.
Original revision: `248f00e359e412f6bc0061ac88be9ff374e3a212`.
Git blob: `4de3e9ddaecf3513bb1a406ae9f9090c8bd0cccd`.

The audit still requires its original ADL derived terminal records for children
#276, #277, and #278 and their merged ancestry. This fixture supplies neither
those records nor current native authority. No records or assertion outcomes
are synthesized. The original Runtime test's execution obligation remains with
its Runtime successor, separately from this historical text audit.

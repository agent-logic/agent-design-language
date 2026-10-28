# WP-12 ACIP WebSocket Transport Proof (#4659)

This retained packet proves the bounded v0.91.7 ACIP WebSocket transport path for #4659. It exercises `tokio-tungstenite` server/client mechanics, validates ACIP JSON envelopes at the transport boundary, applies the WP-12 fail-closed access policy, and records malformed, denied, close-before-response, and timeout failure behavior.

Non-claims: this packet does not claim production TLS termination, production authentication, cross-polis networking, or protobuf wire encoding.

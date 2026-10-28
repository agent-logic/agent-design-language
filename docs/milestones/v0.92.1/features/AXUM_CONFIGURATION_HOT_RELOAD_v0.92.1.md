# Axum Configuration Hot Reload — v0.92.1

## Outcome

Apply validated configuration changes to new Axum requests without restarting the process.

## Design

A watcher debounces file events, loads and validates a complete candidate, and atomically swaps immutable state. Readers remain nonblocking. Invalid or partial files preserve the last-known-good configuration and emit bounded diagnostics.

## Initial scope

Strings, flags, limits, and template configuration only. Database pools, credentials, listeners, and authority-bearing resources require separate lifecycle designs.

## Proof

Concurrent reads, atomic replacement, malformed input, partial writes, rapid event bursts, deletion, permission failure, recovery, observability, and shutdown.

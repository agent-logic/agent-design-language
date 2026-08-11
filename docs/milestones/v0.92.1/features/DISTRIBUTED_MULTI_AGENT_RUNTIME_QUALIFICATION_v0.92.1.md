# Distributed Multi-Agent Runtime Qualification

## Outcome

Qualify the production distributed Runtime with exactly three voters in one polis, three governed agents, one non-voting shepherd, one quorum-leased Observatory, and an out-of-band fault controller.

## Live Windows

1. Three isolated Wuji voters with separate identities, ports, credentials, storage, and state roots.
2. One Wuji voter and two private AWS voters in separate availability zones.

Live qualification cannot begin until `#142` is terminal. The retained input must name the exact merged revision, prove it is ancestral to the qualification revision, and include passing production Guardian, API, WSS, and WP-04.16 receipts. A green or open `#142` PR is not sufficient.

## Required Proof

- delegation, criticism, response, correlation, causation, and exactly-once governed commit;
- explicit `3 -> 2 -> 1` voter behavior: three-voter commit, two-voter continuity, and one-voter mutation halt;
- quorum election, stale-owner fencing, old-Observatory lease expiry before successor bind, snapshot restore, independently materialized snapshot parity, and committed-index parity;
- crash, restart, latency, loss, duplication, reordering, asymmetric partition, healing, certificate failure, disk pressure, and provider stall;
- forged identity, wrong trust domain, stale authority, stale lease/fence, missing capability, copied state, cross-polis replay, malformed envelope, and plaintext/public pre-auth REST/WSS disclosure rejection;
- distinct voter, non-voting shepherd, Observatory, transport, and provider identities and keys;
- private authenticated AWS transport, Wuji isolation without loss of AWS control access, and AWS-only quorum continuity;
- deterministic replay bound to exact commands, terms, committed indexes, envelopes, receipts, source revisions, and model digests;
- coherent Observatory authority cut, redacted causal trace, bounded resources, soak, and provider-verified cleanup after success and after every failed phase.

## No Synthetic Credit

In-process services, mocks, shared state roots, direct executor calls, cached provider outputs, hand-authored receipts, hard-coded counts, skipped tests, screenshots alone, or unverified cleanup cannot satisfy this feature.

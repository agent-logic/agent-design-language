# Issue 191: secure durable three-voter OpenRaft design

## Decision

The Runtime polis consensus substrate extends the existing authoritative distributed transport rather than creating a parallel Quinn stack. One mutually authenticated Quinn connection exists per peer pair. The TLS handshake and ordinary `AuthorityCertificate` bind the trust domain, holder identity, purpose, and transport certificate generation. Before any OpenRaft RPC is decoded, a mandatory signed application handshake binds the exact polis id, sender, receiver, boot generation, protocol version, and committed membership index to that authenticated connection. The existing signed envelope is extended with the same route/session identity plus message kind and payload digest; it remains defense in depth and is not treated as encryption.

Each node owns unique transport private-key material loaded only from path-safe private files and absent from retained evidence. The endpoint API may transiently hold DER bytes in memory, but neither logs nor proof artifacts expose them. The TLS leaf SPKI must equal the public key in the `AuthorityCertificate` accepted by `TransportAuthorization`. Certificate rotation overlap is explicitly valid when the certificate store authorizes it; an unauthorized generation or a superseded generation after overlap fails closed. A deterministic connection-owner and bounded reconnect state machine prevents duplicate bidirectional sessions while allowing either node to recover a failed channel. Every channel has bounded request, response, and reconnect queues.

OpenRaft vote, log, state-machine and snapshot objects are persisted with generation-stamped write-ahead transactions: validate and serialize the candidate into bounded bytes, fsync the journal and directory, atomically replace the target, fsync the directory, then publish the candidate to memory. Atomicity is claimed per OpenRaft callback and durable object; separate OpenRaft callbacks are not represented as one global disk/network transaction. Append and state-machine callbacks return success only after their own durable fsync boundary. Snapshot bytes, metadata, and installed state are reconciled as one durable generation before publication.

Rollback detection is anchored outside the state root by an injected `ConsensusCheckpointAuthority`. It exposes monotonic compare-and-swap and restore for the last accepted durable generation, committed log id, state digest, and snapshot digest. Startup reconciles state/journal/backup against that authority: an interrupted authority update can complete only when the exact new durable digest is present; a coherently rolled-back root cannot satisfy the external checkpoint and fails closed.

Transport anti-replay and OpenRaft retry semantics are distinct. The durable session record is a bounded `(peer, certificate generation, boot generation, sequence, request digest) -> canonical response` cache. A new sequence is journaled with its response only after idempotent RPC dispatch has reached its own durable OpenRaft callback boundary. An exact retry returns the cached response without redispatch; a reused sequence with a different digest, a sequence below the retained window, or an unordered new sequence is rejected. Crash recovery exposes either the old cache with an idempotently retryable request or the new cache with the exact response, never an accepted request with neither route.

Initial routes are derived from a concrete `MembershipState` and an `AuthorityMembership` whose trust domain, committed index, voter guardians, voter identities, and control keys match exactly. Bootstrap accepts only their validated initial joint configuration. Later peer-set changes are consumed only after those authorities expose a newer committed configuration; caller-provided peer lists are routing hints and never membership authority.

## Wire contract

- Length-delimited canonical Prost frames with a hard maximum before allocation.
- TLS-authenticated peer identity and SPKI must equal the authorized certificate, application handshake, envelope sender/receiver and authority-derived route.
- Envelope binds schema, polis, domain, sender, receiver, boot generation, sequence, message kind and payload digest.
- Exact duplicate requests return the bounded durable cached response; conflicting duplicate, reordered, cross-session, or evicted sequences fail closed without redispatch.
- Wrong polis/domain/node/receiver/boot generation, unauthorized certificate generation, oversized/truncated frame and unexpected message kind close the connection without Raft dispatch.

## Storage contract

- State roots and every existing ancestor are lstat-checked; symlinks and nonordinary leaves are rejected.
- Every durable object has a bounded metadata check before read/allocation and exact canonical re-encode validation.
- Injected failures before journal fsync, before rename, after rename and before directory fsync recover to one complete generation.
- Snapshot installation does not publish a new state machine until both snapshot bytes and corresponding metadata are durable.
- Torn, corrupt, or partial writes fail closed; coherent rollback or a lower generation/committed index fails the external `ConsensusCheckpointAuthority` comparison.

## Proof topology

The focused test starts three real Quinn/rustls voters whose route set comes from matching `MembershipState` and `AuthorityMembership` authorities. It uses distinct generated identities and external in-test checkpoint authorities; proves one committed write at 3/3 and 2/3; denies 1/3; proves exact retry response caching and conflicting replay rejection; restarts a voter from its durable prefix; materializes/restores a canonical snapshot; and exercises the complete transport, path, rollback and persistence negative denominator. Strict Clippy and an issue-owned exact receipt bind the source and test output. Higher-level lease/fence/migration behavior remains outside #191 and is delivered serially by #192-#194 before #142 integration.

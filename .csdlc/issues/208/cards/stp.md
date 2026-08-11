# Structured Task Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only the private production continuity listener/client, real kernel checkpoint and isolated validation operations, persistence, cleanup, and focused proof.

## Deliverables

- adl-runtime-kernel/src/continuity_control.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime/src/kernel_continuity_client.rs
- adl-runtime/src/config.rs
- adl-runtime/src/lib.rs
- adl-runtime/tests/kernel_continuity_client.rs
- adl-runtime-kernel/Cargo.toml
- adl-runtime/Cargo.toml
- adl-runtime-kernel/Cargo.lock
- adl-runtime/Cargo.lock
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb
- .csdlc/evidence/208
- .csdlc/issues/208

## Acceptance

1. AC-1: Configuration rejects non-loopback or unsafe internal listeners, overlapping or symlinked roots, missing identities, duplicate identities, invalid TLS material, zero ports, and unbounded policy before bind.
2. AC-2: Mutual TLS plus canonical signed request binding authorizes only the exact configured Guardian identity and rejects bearer-only, agent, public control, voter, Shepherd, unknown, stale, replayed, conflicting, or wrong-kernel requests before dispatch.
3. AC-3: Quiesce/export invokes the live kernel admission gate and CheckpointCoordinator over the exact participant set and returns an opaque bounded handle to the committed signed manifest and service blobs, never synthetic checkpoint bytes.
4. AC-4: Stage/validate accepts only bounded streamed bytes under a fixed isolated root and verifies exact signature, generation/predecessor, accepted prefix, topology, configuration, service set/schema, file names, sizes, and content before possession evidence.
5. AC-5: Resume and discard are exact idempotent operations; failed, cancelled, corrupt, or incomplete work remains isolated and yields independently verifiable zero-residue cleanup evidence.
6. AC-6: Client/server accepted journals, replay/results, external checkpoints and markers reconcile every crash/reply-loss window; cache-first retry never duplicates effects and returns only after exact completion.
7. AC-7: The public Runtime/Observatory listener and OpenAPI contain no continuity route or operation, and evidence reveals no key, token, raw content, endpoint secret, caller path, or private identity.
8. AC-8: Exact thirty-six-case focused proof, strict Clippy, merge-safe immutable receipt validation, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Issue #204 remains blocked until #208 merges
- Final #142 operational integration remains downstream

## Inputs

- agent-logic/agent-design-language#208
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/control.rs and public OpenAPI as read-only denial surfaces
- adl-runtime/src/guardian.rs and RuntimeInitConfig
- adl-runtime/src/distributed/polis_runtime.rs from merged #191
- .csdlc/issues/142 operational design as read-only umbrella truth

## Non Goals

- Consensus, authority protocol, membership, certificate, lease, fencing, or serving eligibility
- Migration/recovery policy or remote transfer orchestration (#204)
- Shepherd/model execution, AWS, live Wuji/AWS qualification, final #142 delivery, merge without operator authorization, or lifecycle closeout

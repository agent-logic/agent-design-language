# Issue #142 Design — Operational Distributed Runtime and Polis Observatory

## Problem

Sprint 3 merged the distributed authority components and library-level integration, but the production Guardian/kernel entrypoints do not yet operate as a real multi-node polis. The retained proof does not launch networked processes or exercise the required API/WSS, partition, fencing, migration, recovery, Observatory, and shutdown behavior.

## Outcome

Provide one production-shaped distributed Runtime path and prove it in two strictly serial three-node topologies:

1. three Runtime/Guardian voters on Wuji;
2. only after full teardown, one voter on Wuji and two independently stateful voters in the approved Agent Logic AWS account.

Each topology owns exactly one polis-level distributed Observatory. The Observatory identifies both nodes and presents one coherent, redacted authority cut. It is not duplicated per node.

## Authority and safety boundaries

- Existing WP-04 distributed authorities remain authoritative; the launcher must invoke them rather than recreate or bypass them.
- Node identities, state roots, ports, credentials, and durable authority state are distinct and bounded.
- The Observatory exposes only the governed distributed projection and redacted operational status.
- Phase B is forbidden while any Phase A process, Observatory, port, lock, state lease, or credential remains active.
- The hybrid layout places two voters in AWS so loss of Wuji preserves a strict two-of-three majority; a two-voter layout is explicitly forbidden.
- Before failure injection, Wuji creates a quorum-committed snapshot, transfers it over the authenticated private channel, and both AWS voters verify its digest, trust domain, epoch, committed index, and restore eligibility.
- Wuji is then stopped as a real process/host-loss event. The AWS majority must fence the unavailable Wuji owner, preserve or restore the committed prefix, activate one authoritative AWS owner only after the safety window, and keep the polis Observatory available in AWS.
- AWS commands use only the verified `agent-logic-admin` business profile and bounded ephemeral infrastructure.
- Remote transport is authenticated and encrypted; remote Observatory viewing uses a private authenticated path such as SSM.
- Exact live proof and cleanup evidence are required; in-process fixtures and screenshots alone are not sufficient.

## Integration shape

The implementation must connect production Guardian and kernel entrypoints to the merged identity, certificate, networking, membership, failure-detection, lease, fencing, placement, snapshot, migration, recovery, and projection authorities. A single polis Observatory/API/WSS service reads one coherent projection cut and shows both members, health, membership epoch/index, and redacted authority summaries.

The operator runner is a serial state machine:

`idle -> wuji_three_running -> wuji_three_stopped -> hybrid_three_running -> snapshot_verified_in_aws -> wuji_down -> aws_continuity_proven -> hybrid_stopped`

Every transition is durable and fail-closed. Hybrid launch is rejected from any state other than a proven local-stopped state with released resources.

## Proof

- Focused configuration and authority-boundary tests.
- Real three-process Wuji proof with one live polis Observatory.
- Full Wuji teardown proof.
- Verified business-account identity, then real one-Wuji/two-AWS proof with one securely viewed polis Observatory.
- Quorum-committed Wuji snapshot, independently verified AWS copies, Wuji loss, AWS majority fencing/activation, continued governed mutation, and stale-Wuji rejection proof.
- Failure/degradation, fencing, migration/recovery, and shutdown evidence in each applicable topology.
- Exact argv and source-bound receipt validation.
- Fresh independently attributable exact-head review.

## Explicit exclusions

No parallel demos, per-node Observatories, public unauthenticated endpoints, plaintext transport, permanent AWS resources, Kubernetes, customer traffic, umbrella reconciliation, or async lifecycle closeout.

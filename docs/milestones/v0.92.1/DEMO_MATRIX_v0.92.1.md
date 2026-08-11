# v0.92.1 Demo Matrix

| Demo | Proof role | Required evidence | Non-proof substitutes |
| --- | --- | --- | --- |
| Corporate control readback | Show company-controlled repository, domain, cloud, deployment, billing, MFA, and recovery authority | Redacted live readback and custody receipts | Screenshots alone; personal credentials |
| C-SDLC v3 fresh lifecycle | Complete install through cleanup with one binary and no v2 mutation | Exact-revision command transcript, state, cards, GitHub readback, second-run no-op | Dispatcher over v2; hand-edited state |
| C-SDLC v3 writer-fenced cutover | Show one writer before, during, and after migration | Archived v2 state, absent writable index, durable fence, v3 authority scan, rollback rehearsal | Dual write; deleting v2 during cutover |
| Wuji distributed polis | Run three voters and three governed agents through production paths, then prove `3 -> 2 -> 1` behavior | Exact `#142` ancestry, node, agent, authority, commit, snapshot restore, old-lease expiry, Observatory, replay, and per-phase cleanup receipts | In-process graph; direct executor calls |
| Hybrid continuity | Partition Wuji from two private AWS voters and prove AWS-only continuity | Two private AZs, authenticated transport, independent snapshots, quorum, election, fencing, mutation halt, healing, model digests, and per-phase cleanup | Shared state roots; manual snapshot copying; public control path |
| Negative authority matrix | Reject forged, stale, cross-polis replayed, malformed, unauthorized, stale-lease/fence, invalid-certificate, and pre-auth disclosure operations | Producer-derived outcomes tied to exact envelopes, key roles, state revisions, commands, terms, and committed indexes | Hard-coded counts or assertion labels |

Demonstrations support proof only when their machine-readable receipts pass independent validation.

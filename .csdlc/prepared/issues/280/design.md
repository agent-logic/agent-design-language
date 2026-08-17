# Issue 280 design

## Issue identity

- Issue: #280
- Title: `[v0.92][WP-18C.07b][117.b] Prove large-Polis performance and recovery behavior`
- Repository: `agent-logic/agent-design-language`
- Current candidate base: `557dd28d85746a8dc5109dcc674f5a606b8c9890`

## Purpose

#280 is a bounded proof-and-remediation slice for the integrated production-Polis Observatory candidate. It verifies that the browser-facing path stays truthful and bounded under large rosters, long transcripts, stream pressure, reconnect/restart, offline transitions, and version mismatch.

The issue may add deterministic fixtures, metrics capture, and narrow Observatory performance/recovery fixes. It must not change Runtime authority, acknowledgement protocol, room-routing authority, durable history semantics, or final qualification assembly.

## Dependency gate

The preparation validator treats these merge commits as the current prerequisite chain:

- #111: `5dab282aa6b730efd057f0502dacd462d30cc1d0`
- #112: `6172bfb067bd45ec231fbc2635e7efbb718ef415`
- #265: `301080a40c91c6882f34fead3c742524467c056d`
- #270: `b1c38cd53573c03cdc4ad818ed5ead5eba570981`
- #271: `6b200cfee83ea36a546123de4d24a6eda191b652`
- #113: `a260e14ab4a56b95fe5b37e4ffaff3f263bc58c1`
- #114: `1d8685745b00df78f304cb03a6a559fa4e2cdec9`
- #276: `3e249f9857f392f7f569560fbd5fbfbc36b95b2f`
- #277: `3160fb8be575ba9a27748b05ea5dd911e4375deb`
- #278: `c3ecaa615fbc29c1784d4e89f4fe38a98743ff02`
- #115: `22122c6c245b1f847aabcaf168a98660a3f11972`
- #116: `557dd28d85746a8dc5109dcc674f5a606b8c9890`

Each must be ancestral to the exact implementation base before bind and before publication.

## Implementation boundary

Owned paths may include:

- `.csdlc/prepared/issues/280`
- `.csdlc/issues/280`
- `.csdlc/evidence/280`
- `demos/html-observatory/tests/large_polis_performance_recovery.test.mjs`
- `demos/html-observatory/app.js`
- `demos/html-observatory/index.html`
- `demos/html-observatory/styles.css`
- narrow helper code under `demos/html-observatory/tests` required to run deterministic large-Polis proof

Forbidden scope:

- #279 accessibility/responsive proof
- #281 security/privacy/adversarial proof
- #282 final qualification assembly
- #117/#110 parent closeout
- Runtime authority, signing, acknowledgement, room routing, durable history, receipt, or key lifecycle semantics
- credentials, cloud/public deployment, Unity live host, or paid/optional jobs

## Proof strategy

1. Generate deterministic large-roster and long-transcript fixtures without credentials or network access.
2. Exercise Observatory state reducers/render helpers for roster growth, transcript windows, stream/backpressure flags, reconnect/restart/offline/version-mismatch transitions, and duplicate-action prevention.
3. Emit machine-readable metrics under `.csdlc/evidence/280`, including fixture sizes, rendered/retained counts, bounded timing, and explicit degradation/recovery state.
4. Run focused proof lanes and diff hygiene before exact-head fresh review.

## Stop conditions

- Any prerequisite merge is not ancestral to the implementation base.
- The proof requires credentials, cloud, Unity live host, or paid/optional jobs.
- Remediation would require Runtime contract or authority changes.
- The proof uncovers #279/#281/#282-owned findings that cannot be fixed narrowly inside #280.
- Exact-head review, publication linkage, required CI, or typed finish fails.

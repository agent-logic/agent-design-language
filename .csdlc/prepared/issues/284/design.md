# Issue #284 design: ADR 0066 Guardian authority evidence reconciliation

## Intent

Reconcile, without changing shared ADR documents, the evidence that can currently support ADR 0066's distributed Guardian membership, authority, fencing, live operation, recovery, migration, and bounded-shutdown claims.

## Evidence boundary

Issue #284 consumes retained evidence from the #142 implementation graph. It does not re-run distributed runtime implementation, alter #142 acceptance, or assert ADR acceptance. The output is issue-local truth for #207: what is terminal, what is partial, and what residual gaps must remain visible to #288 before final ADR serialization.

## Current source observations

- #284 is open and scoped to ADR 0066 reconciliation under #207.
- #5878 has a merged terminal cache for PR #140, with retained WP-04.16 execution proof for distributed Guardian module registration, native-platform receipts, authority replay rejection, oversized-frame rejection, and wrong-domain rejection.
- #194 is closed by merged PR #397 and retains private Wuji-AWS qualification evidence, but its local issue index remains non-terminal and its live summary explicitly records partial qualification and remaining quota/serial-hybrid gaps.
- #142 is closed but has no local derived-terminal cache and no direct closed-by PR reference in the live observation. It is therefore coordination context, not a sole terminal proof source.

## Implementation map

1. Bootstrap and bind #284 in a FastWork issue worktree.
2. Add only issue-local evidence under `.csdlc/evidence/284/`.
3. Add a deterministic validator that checks:
   - terminal cache identity for #5878 / PR #140;
   - retained execution-proof schema, commands, negative cases, and artifact hashes;
   - retained #194 private qualification summary and receipt existence;
   - explicit residual-gap/non-claim classification;
   - live-observation packet consistency for #142, #194/#397, and #5878/#140.
4. Record SOR truth that distinguishes proof, partial proof, stale local cards, and residual gaps.
5. Review, publish, and finish through typed C-SDLC v2 if gates pass.

## Non-goals

- No production/runtime implementation.
- No cloud rerun.
- No changes to shared ADR docs, the ADR index, the final ADR plan, or milestone manifest; #288 owns serialization.
- No weakening of #142/#194/#5878 acceptance truth.

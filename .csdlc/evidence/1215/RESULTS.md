# Issue 1215: idle resident readiness

The previous five-minute inference age rule conflicted with the event-driven monitoring contract. Successful current-binding evidence now remains last-known ready while metadata is fresh, until failure or explicit invalidation. No recurring inference is introduced. Missing evidence remains unverified.

Historical age-only incidents retire as `idle_policy_reconciled` with retained history and alert receipts; this is not fabricated new inference or provider recovery. Actual failures still require successful post-incident inference. Pending outbox delivery remains at-least-once. Observatory identifies retired incidents as historical.

## Proof

- Focused kernel `--lib resident_health`: 23 passed, zero failed.
- Observatory `agent_orientation.test.mjs`: 4 passed.
- Independent review by `/root/review1215`: two findings fixed (old expectation and retired-incident wording); re-review found no remaining actionable findings.
- PVF: required Runtime and UI regression; deterministic local CPU and temporary journal files, no provider/AWS calls in tests. Hosted CI is separate pending integration proof.

## Live diagnosis

On the preceding deployed generation, one explicit operator conversation per resident succeeded for Beacon, Ember, Delta, Quill, Harbor and Nova. Afterward the full live roster reported all six healthy/ready/available and zero open incidents. This proves bounded provider execution, not the new idle policy. Post-deployment verification must observe readiness beyond five minutes without extra model probes. Live replies and credentials are not retained.

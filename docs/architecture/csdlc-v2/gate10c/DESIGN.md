# Gate 10C reversible cutover

`csdlc-cutover` is a typed transaction over the tracked generation selector. It requires current green Phase B evidence, starts from v1, switches to v2 for a complete lifecycle smoke, restores v1 for a runnable v1 smoke, then switches back to v2 and repeats the lifecycle smoke. Any failed or ambiguous step restores the original v1 selector and returns failure.

The only permitted tracked mutation during execution is the selector. All v1 code, commands, tests, installer paths, and recovery surfaces remain present. `csdlc-install resolve` makes that tracked selector the sole routing authority and preserves an explicit v1 override after default v2 selection. Every post-mutation error restores the exact original selector bytes; restoration failure is a reconciliation-required terminal result.

The reviewed request declares only 14-day rollback and 30-day importer durations. The cutover binary records their RFC3339 timestamps from the actual successful switch instant, so authored future timestamps cannot masquerade as elapsed operational evidence. Evidence always hard-codes `deletion_authorized: false`; v1 deletion remains a separately reviewed and explicitly approved Gate 10D action.

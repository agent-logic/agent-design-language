# Gate 10C reversible cutover

`csdlc-cutover` is a typed transaction over the tracked generation selector. It requires current green Phase B evidence, starts from v1, switches to v2 for a complete lifecycle smoke, restores v1 for a runnable v1 smoke, then switches back to v2 and repeats the lifecycle smoke. Any failed or ambiguous step restores the original v1 selector and returns failure.

The only permitted tracked mutation during execution is the selector. All v1 code, commands, tests, installer paths, and recovery surfaces remain present. An explicit v1 override remains supported after default v2 selection. The evidence hard-codes `deletion_authorized: false` and records exact 14-day executable rollback and 30-day importer windows using parsed RFC3339 timestamps.

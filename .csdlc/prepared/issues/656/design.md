# #656 — Atomic Runtime v3 generation installation

Install `csm`, `adl-runtime-guardian`, and `adl-runtime-kernel` as one immutable generation. Stage and verify the complete set against one receipt, then atomically change one canonical `current` reference. launchd and Runtime-init must resolve the same generation. Reject incomplete, mixed, or incompatible sets before service mutation, retain the prior verified generation, and support bounded rollback. This slice excludes the live Runtime, timeout policy, providers, identity, Observatory, Caddy, cloud, and Runtime v2.

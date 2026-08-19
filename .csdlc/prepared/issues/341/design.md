# #341 Provider-neutral multi-agent proof design

Issue #341 proves the WP-18B provider-neutral multi-agent birthday scenario over the landed v0.92 Runtime prerequisites. It consumes #256 and #414; it does not own their implementation paths.

The owned proof surface is intentionally small:

- one provider-neutral scenario harness;
- one proof-matrix validator;
- one focused validator/negative-case test runner;
- redacted issue evidence under `.csdlc/evidence/341`;
- one feature projection under `docs/milestones/v0.92/features/`;
- a private, non-production Observatory packet showing several agents running.

Live positive proof must use at least two approved real provider credentials. Local reference proof may be used only for deterministic validator tests and must not be recorded as the final live-provider acceptance claim.

The Observatory demo remains private. Public Observatory exposure, production ingress, GPU quota work, and new Runtime recovery/snapshot behavior are non-goals.

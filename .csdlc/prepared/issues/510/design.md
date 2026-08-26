# Issue 510 design: HOT-01

## Outcome

Produce one production-ready Axum hot-reload implementation.

## Authority and scope

This issue owns only the declared paths below. It does not authorize adjacent sprint work,
cloud/provider mutation, credential disclosure, legal advice, or lifecycle work for another issue.

- `adl-runtime/src/config_reload.rs`
- `adl-runtime/tests/config_reload.rs`
- `docs/runtime/config-hot-reload.md`
- `adl-runtime/src/lib.rs`
- `.csdlc/prepared/issues/510`

## Execution shape

1. Reconcile dependencies and freeze the exact issue-local denominator.
2. Produce one production-ready axum hot-reload implementation with last-known-good retention.
3. Run the planned PVF lanes and retain bounded, redacted evidence.
4. Obtain exact-head review and stop before publication unless separately authorized.

## Invariants

- Issue completion is exactly one production-ready hot-reload implementation; behavioral cases are proof inputs.
- Valid reload, invalid retention, debounce, concurrent-read, and watcher-shutdown tests pass.
- Private credentials, legal instruments, auth codes, recovery factors, and provider secrets stay outside Git.
- Any operator-only mutation requires explicit bounded authorization at execution time.

## Stop conditions

- Reload requires process restart
- Partial configuration can become visible

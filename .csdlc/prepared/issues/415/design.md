# Issue #415 — Exact builder preflight diagnostics

## Authority and boundary

Issue #415 repairs diagnostics for the immutable AWS builder path before any
further #268 paid attempt. It must not launch AWS resources, mutate #268 or
#269 lifecycle state, add provider fallback, or weaken image and source
immutability.

## Design

Replace the compound builder-toolchain shell block with an ordered set of
individually named checks. Each check captures its exact combined stdout/stderr
in one bounded private temporary file, records its exit status, redacts that
capture into the cumulative durable log, and removes the raw temporary file.
The script emits a concise diagnostic envelope naming the failed check. Checks
cover architecture, Rust,
Cargo, nextest, sccache, linker, AWS CLI, Ruby, Ruby smoke, and receipt-validator
self-test.

The builder script owns the durable toolchain diagnostic. Raw capture is never
a portable artifact: it is removed on success and failure by an EXIT trap. The
script writes through a second temporary file and atomically publishes only a
redacted `builder-toolchain.log`, so early failure cannot leave only a remote
temporary path or a partial durable file. The failure line identifies the exact
check and exit code without guessing the missing executable.

The remote runner must presence-gate and emit the retained builder-toolchain
log on its normal post-command path after it captures `COMMAND_EXIT`, because
builder failures deliberately run under `set +e` and do not trigger ERR. The
ERR trap repeats the same best-effort emission only as secondary protection.
Missing or unreadable diagnostics never override the command status, summary,
or cleanup path. This keeps early builder failure visible in command stderr and
portable attempt artifacts while leaving exact-owner cleanup unchanged.

## Proof

Focused tests simulate one missing executable, assert the precise check name,
exit status, retained redacted output, and raw-capture removal, and prove the
validation command was not reached. Existing success, Ruby, architecture,
cache, validation-failure, and summary-failure cases remain green. A dynamic
runner fixture executes the captured nonzero-command path and proves the
diagnostic is emitted after `COMMAND_EXIT`; a missing/unreadable diagnostic
case proves summary and cleanup processing remain reachable. Exact-scope proof
rejects changes outside the declared issue and lifecycle paths.

## Stop conditions

- A proposed fix requires an AWS launch or provider mutation.
- Exact output cannot be retained without exposing secrets or host-specific
  absolute paths.
- Cleanup ownership or immutable-image/source checks would be weakened.
- The implementation would touch #268 or #269 lifecycle state.

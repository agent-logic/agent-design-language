# Runtime v3 Parity-A Complete Implementation Design

Issue #5591 owns the first Runtime v3 parity implementation lane under #5361.
This packet describes the complete outcome and contains no partial-runtime
implementation. Architecture authority #5336 integration is the sole current
stop condition. Once that exact authority is on `main`, the claim must be
amended through `csdlc-bind` before any product file changes.

## Canonical ownership and boundaries

- `adl-runtime-kernel` is the independent canonical Runtime v3 kernel.
- `adl-runtime` may provide guardian/supervision integration only where the
  accepted #5336 architecture assigns it; Runtime v2 source is never imported,
  linked, copied, edited, or deleted by this issue.
- Maintained COTS crates own transport, TLS, serialization, checksums, bounded
  channels, and signal handling wherever they meet the required contracts.
- No AWS, default switch, hard-coded IP address, HTTP-only access, provider
  deployment, Runtime v2 deletion, or unrelated product scope is authorized.

## Full execution outcome

The guardian launches one configured Runtime v3 process and exposes one secure,
typed canonical ingress. Representative domain work crosses bounded channels,
executes production components, and emits deterministic retained evidence plus
Observatory-readable lifecycle, queue, checkpoint, replay, resume, pressure,
shutdown, and access events.

Checkpoint records are versioned, checksummed, atomically committed, and tied to
the accepted ingress sequence. Replay rejects corrupt, truncated, incompatible,
out-of-order, duplicate, or unauthorized state. Resume reconstructs the same
observable state and next-work result as uninterrupted execution.

Normal stop and configured resource-pressure thresholds both stop admission,
drain or explicitly serialize bounded work, commit the final checkpoint, emit
the terminal Observatory event, and shut down within a tested bound. Restart
resumes from that checkpoint without double execution.

Local and remote listeners are configuration-driven and TLS-authenticated.
Loopback and non-loopback addresses are supplied by configuration or ephemeral
test allocation, never literals in product code. Plain HTTP, missing or invalid
credentials, authority escalation, malformed ingress, unknown message types,
oversized frames, and replay tampering fail closed.

## Future implementation claim-scope proposal

After #5336 integrates, amend the preparation claim with the smallest verified
subset of these paths before implementation:

- `adl-runtime-kernel/`
- `adl-runtime/` only for guardian-owned launch/supervision integration
- `infra/runtime-v3/` only for configuration required by local/remote access
- focused Runtime v3 test, proof, and owner-script paths
- `.csdlc/evidence/5591/`

The amendment must first prove no collision with active child claims and must
exclude every Runtime v2 implementation path.

## Exact-revision evidence contract

Acceptance requires one committed revision with guardian-launched live ingress,
deterministic continuity, pressure shutdown, secure configured access,
Observatory output, negative authority proof, strict lint, focused tests,
dependency inventory, and the current Runtime v3 LoC/test budget report. Fixture,
library-only, mocked-process, degraded, skipped, pending, or prose-only evidence
does not satisfy any criterion.

## Current stop condition

`#5336` is not yet integrated into `main`. This is the sole current stop. The
packet may be reviewed and committed, but implementation binding, product edits,
validation execution, acceptance, publication, and readiness advancement must
wait for the typed PR state to show #5336 green and merged.

# #1127 validation contract

PVF lane: runtime. Proof role: deterministic local regression. Resource profile:
local filesystem and in-process/mock provider, no paid inference or AWS access.
Release gate: focused admission greeting regression plus required CI and independent review.

Command: `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib admission_greeting`.

Coverage includes real `shepherd` identity versus `beacon` display-name mismatch,
missing sender reason persistence, bounded failed-series retry, stale request
replay and completed-delivery protection, legacy unknown reasons, authorization,
write-failure rollback and existing restart/concurrent greeting regressions.
Refusal reasons derive only from static control-plane error codes. The authenticated
status projection excludes message parts, reply text, tokens and raw provider errors.
Structured tracing stays separate from HTTP JSON; no compatibility logging override
is introduced. Live delivery/deployment is separate from this local proof.

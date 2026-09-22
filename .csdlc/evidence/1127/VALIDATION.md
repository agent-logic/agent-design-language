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

## Hosted CI dependency-guard repair

Run 35670604408 failed in `runtime_kernel_manifest_has_no_repo_local_path_dependencies`:
`adl-provider-core must use crates.io or std, not a repo-local path dependency`.
The existing manifest and #855 commit 3734d45d4bec5db04406d7be00a421feae8db01e
introduced the canonical provider extraction before this issue. The welcome
change did not modify that manifest. The contract repair permits only its exact
existing sibling path, rejects package aliases, other local dependencies and
workspace inheritance, and leaves dev/build/target dependency checks intact.
This is local deterministic policy regression proof, no external resources.

Additional command: `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test contracts`.

The same hosted lane's local reproduction exposed three pre-existing strict
Clippy findings (two long reload helper signatures and one deliberately held
commit gate in a two-worker race fixture) and an outdated Observatory exact-key
assertion missing the already-shipped `resident_incidents` field from #1114.
Narrow function-local lint reasons document those existing designs without
weakening the lane, and the schema assertion includes the actual field.
No runtime behavior is changed by these CI compatibility repairs.

After repair, the complete runtime command passed: 841 tests, 0 failed,
1 ignored across 43 reported suites (including doc tests). Exact hosted Clippy
command passed: `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml
--all-targets -- -D warnings`. Hosted Linux CI still supplies integration proof.

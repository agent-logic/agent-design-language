# Session ledger locking

Session occupancy is advisory coordination, not C-SDLC lifecycle authority.

The public CLI keeps a persistent `<ledger-name>.lock` file with the marker
`adl.session-ledger.os-lock.v1`. Each mutator holds an exclusive OS file lock
through its read, modification and save. Dropping the guard explicitly unlocks
before closing its descriptor, so a duplicate descriptor cannot extend the
guard's critical section. Process death releases ownership when the last
inherited or duplicated descriptor closes. If explicit unlock fails, guard
destruction still closes its descriptor as a best-effort fallback; outstanding
duplicates can delay release in that error case. The file remains in place so all
waiters and later callers use the same inode. **Do not delete a current lock
file**, even when no process appears to hold it.

Initialization writes and syncs the complete marker in a temporary file in the
same directory, then publishes it without replacing any existing destination.
An interrupted initialization may leave a temporary file, but cannot publish an
empty lock. Filesystems must support exclusive advisory locks and atomic
no-clobber publication; unsupported operations return errors. This protocol
assumes a trusted ledger directory that other actors do not replace while used.

Old CLI clients used file existence as ownership. They cannot operate against the
persistent marker. An old-format or invalid lock is refused, without PID/age
heuristics or automatic deletion. To migrate, stop every old client using that
ledger, preserve any needed diagnostic evidence, explicitly remove only the
confirmed legacy lock, then use the new CLI exclusively. Never remove a legacy
lock while its old client may still be running. Missing lock files require no
migration action.

## Validation classification

The `session_ledger` Rust tests and the native `session` contract in `tools/public_contracts`
are PVF `local_contract`: deterministic owned temporary files and child
processes, bounded local CPU/disk, no network or providers, required public
source regression gates. Tests cover live contention, killed holders,
concurrent read-modify-write updates, duplicate-descriptor lifetime and stale
duplicate closure, stable-inode waiters, legacy refusal and
initialization failure. Public CLI checks verify repeated commands and persistent
marker truth. Run the public CLI lane on macOS and Linux before claiming both
platforms; local results alone do not establish CI or release acceptance.

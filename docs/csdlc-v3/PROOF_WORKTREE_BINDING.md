# Proof and install worktree ownership

Issue #762 repairs A520-ARCH-005 and C520-CODE-005. The location of the
stable `csdlc` executable is provenance, not the destination of issue work.
Invoke the installed binary from the registered issue worktree or a directory
beneath it. The primary checkout is inspection-only.

| Route | Classification | Mutation boundary |
| --- | --- | --- |
| `proof` | Operational | Authenticated native bound issue; receipt beneath its evidence directory |
| `install` | Operational | Same ownership checks; existing exact artifact and cutover approval checks remain required |
| `shadow` | Historical | Execution disabled; returns `historical_route_disabled` without running commands or writing receipts |
| `soak` | Historical | Execution disabled; retained transition evidence remains available for inspection |

Operational request JSON adds a required `binding` object. Its fields are
`worktree` (absolute path), `branch`, `exact_head` (full Git SHA),
`git_common_dir` (absolute path), `generation`, and `lifecycle_digest`.
`evidence_root` must resolve to that same invoking checkout. A request without
binding is denied even when it carries an old operator approval.

The adapter observes Git topology independently. It requires a linked
non-primary checkout, an attached matching issue branch, exact HEAD, matching
common Git directory, and the exact live worktree registration. The common
Git directory must also match the stable binary installation provenance; its
checkout never supplies the mutation destination. It checks the
canonical repository remote, native authority selector and authenticated
reconciliation evidence against `origin/main`, then enforces the canonical
worktree policy. Git environment overrides are rejected.

The native index must identify the requested repository and issue in `bound`
phase, with matching branch and worktree. The binding record, six card pairs,
index digest and expected generation must agree. A request supplies expected
values; it cannot grant ownership by naming a directory. Construction state or
retained v2 state does not satisfy this operational v3 contract.

All issue output paths are checked before execution, including existing
ancestors of absent paths. Symlinks and parent traversal are denied. Ownership
is rechecked before installation and before durable receipts, including after
a proof command returns. Temporary files use exclusive creation. These are
filesystem identity guards, not an operating-system sandbox for arbitrary
proof executables; callers must continue to govern the proof command itself.

Blocked proof-route reports remain nonzero exits. Both ready and blocked typed
reports use stdout; the blocked human diagnostic uses stderr. This change does
not claim OpenTelemetry or new compatibility-log behavior.

## Validation contract

`proof_worktree_binding` is a deterministic, local, medium-resource integration
lane and required issue proof. It creates an isolated primary repository and
linked issue worktree, installs one driver binary under the primary's stable
`.adl/bin/native-v3/` path, then invokes that same binary from primary, worktree
and nested worktree directories. It checks actual receipt/install bytes and
snapshots issue mutation surfaces around rejected requests. Native bound
records are fixture data; this test does not claim to test the separate native
binder/editor repair. No credentials or network are required.

`proof_parity_install_commands` is a small required CLI denial lane for legacy
requests and historical routes. The prior primary-mutating construction
fixtures are replaced by linked-worktree operational proof. The terminal
verifier's historical cutover receipt fixture is now explicitly modeled test
data, so it does not call an operational route to manufacture historical
unbound authority. #763 continues to own the broader detached-checkout fixture
repair; #760 owns broader authority-surface alignment.

Run focused validation with:

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding --test proof_parity_install_commands --test command_manifest --test terminal_cleanup_cutover_commands
cargo test --manifest-path csdlc-v3/Cargo.toml --lib
cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings
```

Hosted C-SDLC owner CI remains integration evidence required before review-ready
publication. Local focused proof does not substitute for required hosted checks.

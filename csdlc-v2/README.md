# C-SDLC v2

Current package version: `0.92.0`.

`csdlc-shepherd --schema` and `--example <name>` are currently the complete
discovery path for a JSON-input owner binary. The same affordance is planned
for the remaining request-driven binaries as their CLI contracts are repaired;
until then, use their typed public schema bundle and request definitions.

This is the standalone clean-room C-SDLC v2 workspace. It does not depend on
ADL or Runtime crates and does not reuse their lifecycle implementation,
schemas, templates, tests, fixtures, or skills.

Gate 2 provides the typed lifecycle/card engine and whole-record transactions.
Gate 3 adds `csdlc-issue create` and `csdlc-bind` for deterministic construction
and safe Git branch/worktree binding. Git uses typed argv arrays;
the control plane contains no shell or Python lifecycle logic. Later gates add
PVF, review truth, publication, and terminal finish without widening this core's
authority.

Gate 4 adds `csdlc-validate`, `csdlc-schedule`, and `csdlc-shepherd`. Validation
manifests contain executable-plus-argv commands, deterministic dependencies,
resource costs, network/credential posture, timeouts, and bounded evidence
policy. The scheduler and shepherd are pure read-only classifiers; only
`csdlc-validate` can execute a declared proof DAG.

Gate 5 adds `csdlc-review` for issue-bound review assignment, exact-revision
review recording, finding/fix/route evidence, and a read-only publication
guard. Review has no GitHub or lifecycle publication authority.

## Focused validation

```text
cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check
cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path csdlc-v2/Cargo.toml
```

## Contracts

`csdlc-edit schema` prints the versioned JSON Schema bundle. `csdlc-edit
bootstrap --request <json>` atomically creates an issue record and all six
cards from typed values. `csdlc-edit apply --request <json>` performs one
guarded semantic operation. `csdlc-doctor --repo <path> --issue <n>` emits
stable JSON and performs no network or mutation.

Bootstrap selects a typed planning profile and automatically writes explicit
SPP time/token estimates and VPP time/token budgets. There is no follow-up
manual budget-filling stage.

Gate 10D2 is the current authority. The v1 wrappers and command surfaces are
sunset; `csdlc-install resolve` selects the independent v2 binaries and the
eleven typed operator skills cover init, GitHub, finish, review, shepherd,
doctor, validate, bind, clean, card editing, and publish. Earlier coexistence
gates remain historical evidence, not current operating instructions.

Gate 10D1 adds `csdlc-eligibility`, a non-mutating decision and proposed-
manifest binary. It derives the exact Gate 1 inventory from its pinned Git
revision, binds operator approval to Phase B, Phase C, selector, manifest, and
code-revision digests, enforces the reviewed 90/80-percent thresholds and both
mandatory sunset windows, and always reports `deletion_executed: false` on
stdout. Its `schema` subcommand publishes the versioned JSON contracts. Actual
removal belongs to a separate approval-gated issue.

Markdown files are generated projections. The engine renders deterministic
Markdown from typed values, parses it with `markdown.rs`, validates semantic
anchors, and records values/rendered/AST digests. Direct Markdown edits fail
doctor as corruption.

Historical terminal records and retained receipts remain deserializable and
available through read-only compatibility inspection. They are not writable
delivery authority. New terminal authority is the minimal derived envelope
produced by `csdlc-finish` from live GitHub state.

The ADL primary checkout is inspection-only. Native `csdlc-issue create`
bootstrap for `agent-logic/agent-design-language` must run from an isolated
staging checkout, never from root `main`; the bootstrap guard fails closed when
Git topology cannot prove a non-primary checkout. Issue-local bootstrap remains
supported when all six cards and the approved design already live in the target
worktree. Run `csdlc-bind` from that worktree; the binder verifies the issue,
branch, worktree, and GitHub state in place without writing to the primary
checkout. Branch/worktree topology and live issue/PR state are the coordination
authority.

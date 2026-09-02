# C-SDLC v3 Package Boundary

`csdlc-v3/**` is the construction surface for the planned clean replacement of
C-SDLC v2. It is not the live lifecycle authority yet. Root `AGENTS.md`,
`csdlc-v2/AGENTS.md`, and the typed v2 operator skills remain authoritative
until the explicit V3-F/#505 cutover decision approves the transition.

## Current construction state

- V3-A/#500 established the v3 contract and predecessor map as construction
  evidence. Corrective follow-up #571 repaired the predecessor owner/proof-lane
  and lifecycle-gate consistency gaps; it remains historical corrective
  evidence, not authority cutover.
- V3-B/#501 added foundation import/projection surfaces for v2 compatibility
  exploration. Those surfaces are read-only construction evidence, not live
  import or migration authority.
- V3-C/#502 added the lifecycle-kernel construction slice and
  `csdlc-v3/AGENTS.md` package-local guardrails. It does not replace v2
  lifecycle commands.
- V3-D/#503 adds the local preparation command model exposed through the single
  `csdlc local` proof surface. It must remain non-authoritative until V3-F.
- V3-E/#504 adds remote delivery, review, publication, finish, and cleanup
  models. They are cutover-readiness proof surfaces only until V3-F.
- V3-G/#570 repaired v2-first documentation and skill guidance for advance
  notice. The live route still remains typed v2 until V3-F/#505.

## Clean replacement target

The target is a clean v3 replacement line, not permanent v2/v3 coexistence.
V3 work should make issue start, review, publication, finish, and cleanup easier
to operate while preserving typed contracts, exact topology, exact-head review,
closing-linkage proof, terminal truth, and safe cleanup boundaries.

A prepared v3 issue should be inspectable, bindable, and ready for first useful
work in three minutes or less once dependencies are satisfied. That time target
removes ceremony and ambiguity; it does not remove typed authority, review, or
validation gates.

## Non-goals before V3-F/#505

Before V3-F/#505, v3 must not:

- bind worktrees for live issues;
- mutate `.csdlc/issues/**` as lifecycle authority;
- publish pull requests or mutate GitHub;
- finish issues, close issues, or derive terminal truth;
- clean worktrees or retire v2;
- claim compatibility, migration, rollback, or authority cutover as complete.

## Focused proof commands

Use the issue-owned focused proof for the slice you are working on. Current
local construction checks include:

```sh
cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check
cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets
cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings
```

The v3 crate builds one operator-facing binary named `csdlc`. Current
construction subcommands are:

```sh
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- foundation --repo-root .
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- local --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
```

For docs and cutover-readiness work, use the issue-owned validators declared by
the active issue, such as the #570 stale-route and skill-guidance scans. Passing
v3 construction checks is evidence for the v3 package only; live lifecycle work
still routes through typed C-SDLC v2 until V3-F/#505.

Operators preparing for the one-binary replacement should read
`docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md`. That notice is advance guidance,
not authority cutover.

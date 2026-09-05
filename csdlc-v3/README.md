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
- V3-F/#505 now includes a read-only `csdlc sprint` readiness verifier for
  testing upcoming sprint umbrellas against typed issue readback evidence before
  cutover. It is planning evidence only and does not start child execution.
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
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- bind --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- doctor --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- edit --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- eligibility --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- issue --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- schedule --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- shepherd --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- validate --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- github --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- github-issue --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- github-pr --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- pr-state --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- publish --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- review --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- remote --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- finish --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- clean --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- cutover --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- install --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- proof --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- shadow --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- soak --help
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- sprint --repo-root . --request <request.json>
```

Those advertised commands are still construction interfaces. They do not grant
live lifecycle, GitHub, publication, finish, cleanup, install, or cutover
authority before #505.

The Sprint 8/9 pre-cutover canary is:

```sh
bash .csdlc/prepared/issues/505/run-v3-sprint-8-9-readiness-trial.sh
```

That canary reads live issue state through typed C-SDLC v2 issue transport,
parses the current umbrella membership for #536 and #537, and then verifies the
result through non-authoritative v3 sprint readiness.

For docs and cutover-readiness work, use the issue-owned validators declared by
the active issue, such as the #570 stale-route and skill-guidance scans. Passing
v3 construction checks is evidence for the v3 package only; live lifecycle work
still routes through typed C-SDLC v2 until V3-F/#505.

Issue #505 is the pending V3-F authority-transition decision. Until #505 is
explicitly approved, merged, and terminally reconciled, v3 remains
non-authoritative construction and cutover evidence. Operators must receive the
pre-change notice in `docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md` before any
default route changes from v2 to v3.

Operators preparing for the one-binary replacement should read
`docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md`. That notice is advance guidance,
not authority cutover.

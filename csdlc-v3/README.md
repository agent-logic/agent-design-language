# C-SDLC v3 Package Boundary

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

## Historical construction state

The following milestones describe pre-cutover construction. Their restrictions
apply to those historical stages; current operation follows the native authority
contract and root `AGENTS.md`.


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

## Native post-cutover authority

After the #505 / PR #591 cutover, operational authority is rooted in
`csdlc-v3/operator/authority-selector.json` and its digest-bound
`native-authority-receipt.json`. Both files must be byte-identical to their
copies on canonical `origin/main`. The receipt retains the reviewed cutover
head, merge commit, migrated selector digest, and the authenticated-readback
reconciliation policy. Missing, stale, malformed, or digest-mismatched
authority evidence denies operational work.

The migration is one-way for normal operation: the native receipt records the
last v2 selector as historical provenance, while v3 no longer compiles against
or reads from the v2 source tree. A rollback requires an explicit canonical
native selector with `generation: rollback` and suspended authority; absence
or corruption of the v3 selector is never rollback authorization.
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

## Preparing local issue state with v3

The v3 `issue` route initializes or updates local lifecycle state for an
already-numbered issue. It does not create a GitHub issue.

The local route needs two small JSON files:

1. `request.json` describes the numbered issue and requested local commands.
2. `registrations.json` lists the allowed worktree registration for that branch.

Example local `request.json`:

```json
{
  "issue": 503,
  "title": "[v0.92.1][V3-D] C-SDLC v3 local preparation workflow",
  "repository": "agent-logic/agent-design-language",
  "branch": "codex/503-v3-d-local-preparation-workflow-exec",
  "worktree": "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec",
  "registry_version": "1.0.3",
  "expected_lifecycle_digest": null,
  "commands": [
    "prepare_issue",
    "bind_worktree",
    "edit_cards",
    "plan_pvf",
    "doctor",
    "schedule",
    "shepherd",
    "eligibility"
  ]
}
```

Example `registrations.json`:

```json
[
  {
    "branch": "codex/503-v3-d-local-preparation-workflow-exec",
    "worktree": "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec",
    "primary": false
  }
]
```

Run the local route from the repository root:

```sh
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- \
  issue \
  --request request.json \
  --registry docs/templates/prompts/current.json \
  --registrations registrations.json \
  --repo-root .
```

To write and re-read local v3 lifecycle state for a canary, use an ignored
state root and keep the same request digest on later writes:

```sh
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- \
  issue \
  --request request.json \
  --registry docs/templates/prompts/current.json \
  --registrations registrations.json \
  --repo-root . \
  --v3-state-root .git/csdlc-v3/local-state
```

If v3 reports stale existing lifecycle state, do not delete or bypass the
guard. Read the reported digest, put it in `expected_lifecycle_digest`, and run
the command again only if that existing state is the state you meant to update.

## Creating a GitHub issue with v3

Use the `github-issue --execute` route for real GitHub issue creation after v3
operational authority is active. The mutation follows the GitHub issue-create
model: title and body are required; labels, assignees, and milestone are
optional. GitHub assigns the issue number, and v3 records that assigned number
in its authenticated mutation receipt.

Example operational request:

```json
{
  "expected_lifecycle_digest": "<current-selector-digest>",
  "exact_review_sha": "<approved-v3-cutover-review-sha>",
  "operation": {
    "kind": "github_mutation",
    "request": {
      "repository": "agent-logic/agent-design-language",
      "cutover_issue": 505,
      "operator_approval": "operator approved creating this issue with v3",
      "expected_head_sha": "<approved-v3-cutover-review-sha>",
      "credential_names": ["GITHUB_TOKEN"],
      "mutation": {
        "action": "issue_create",
        "title": "[v0.92.1] Short issue title",
        "body": "Issue body written in the same style you would pass to gh issue create --body.",
        "labels": ["v0.92.1"],
        "assignees": [],
        "milestone": null
      }
    }
  }
}
```

Execute it from the repository root:

```sh
cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- \
  github-issue \
  --request create-issue.json \
  --execute
```

The route writes a durable intent before the external mutation, posts to
`repos/<owner>/<repo>/issues`, validates GitHub's create response, re-reads the
created issue by the v3 operation marker, and then writes an authenticated
receipt under the Git control directory. If the mutation result is uncertain,
the durable intent prevents blind replay until authenticated reconciliation
succeeds.

Before cutover, the same command fails closed instead of mutating GitHub.

After the canonical evidence-bound v2 selector activates v3 authority for the
exact reviewed #505 head, named local routes automatically enter native
operational mode from `--repo-root`, the canonical selector, retained cutover
approval, typed request, and requested worktree. There is no caller-controlled
authority switch. Operational `schedule` requests must provide all six
`schedule_readiness` dimensions. Operational `shepherd` requests must provide
`shepherd_routing`, which deterministically classifies waiting, retryable,
repair-required, operator-required, or ready state.

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

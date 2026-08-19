# ADL C-SDLC GitHub Client Boundary

This document records the current C-SDLC v2 ownership boundary for GitHub issue
and pull-request operations.

## Canonical Entry Points

GitHub work for C-SDLC v2 is owned by repo-native Rust binaries and the shared
token resolver. Do not use the ChatGPT GitHub connector, legacy wrappers,
shell/Python lifecycle mutation, or AWS for covered lifecycle writes. Raw `gh`
is also prohibited except for the separately audited break-glass transport
defined below.

The current command surface is split by responsibility:

Covered C-SDLC GitHub route owners: issue actions =
`csdlc-github-issue`; PR state = `csdlc-github-pr`; publication =
`csdlc-publish`; terminal delivery = `csdlc-finish`.

Route rule: the ChatGPT GitHub connector and raw `gh` are prohibited for
covered lifecycle writes except for the audited break-glass transport below. A
missing binary, unfamiliar error, timeout, or operator preference is not by
itself break-glass authority.

- `csdlc-github-issue` owns GitHub issue lifecycle actions:
  `issue_create`, `issue_update`, `issue_comment`, `issue_close`, and
  `issue_read`.
- `csdlc-github-pr` owns GitHub PR observation through `pr_state`.
- `csdlc-pr-state` remains the dedicated low-level PR-state observer used by
  other v2 binaries.
- `csdlc-publish` is the sole PR-publication owner.
- `csdlc-finish` is the sole terminal-delivery owner, including exact-head
  merge and derived-terminal authority.
- `csdlc-github` remains a compatibility facade while callers migrate to the
  narrower owner binaries.
  It also owns the read-only organization larger-runner preflight because that
  observation spans hosted runners, runner groups, selected repositories,
  workflow refs, and Actions job dispatch rather than issue or PR lifecycle.

Every issue/comment mutation must carry an `operation_key`. The GitHub command
surface renders it as a stable marker, reads back remote state, and fails closed
on missing, duplicated, or mismatched reconciliation.

## Audited Break-Glass Boundary

Typed C-SDLC v2 remains the default and final lifecycle authority. Raw `gh` is
only emergency transport for a confirmed reproducible regression in the exact
typed owner, after a durable tooling-regression issue and explicit
operation-scoped operator authorization. It does not become lifecycle
authority.

The sole allowed canonical argv shapes, exact identity requirements,
terminal/destructive denylist, mode-0600 body-file rule, three create-only
receipt events, redaction contract, and mandatory typed reconciliation freeze
are defined together in:

- root `AGENTS.md`
- `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`

No merge, issue close, finish, cleanup, deletion, release, administration,
secret/variable, workflow, force, bulk, API, alias, extension, or issue-create
operation is in the exception. After a transported write, readiness, review,
publication, merge-ready, terminal, and finish claims remain denied until the
typed owner reconciles exact remote state and the immutable reconciliation
event records success.

## Machine Output Termination

The split issue and PR binaries route schema, success, and typed error JSON
through the shared C-SDLC stdout writer. Machine-readable JSON remains on
stdout and human diagnostics remain on stderr. If a downstream reader closes
stdout early, the writer treats `BrokenPipe` as normal termination and emits no
panic or backtrace text. Serialization failures and every other stdout I/O
error remain fail-closed.

## Shared Client Ownership

Shared GitHub behavior belongs in the C-SDLC v2 GitHub library code, not in
individual command wrappers:

- token-source selection through the shared resolver
- marker rendering and exact-marker checks
- issue readback and idempotent mutation reconciliation
- PR state normalization and readiness classification
- retry/backoff behavior through the shared `adl-resilience` crate where a
  bounded retry policy is appropriate

The GitHub app connector is read-only for this repository and is not a write
fallback. The operator-approved token file may be supplied through
`token_file`/`ADL_GITHUB_TOKEN_FILE`; token contents must never be printed,
copied, persisted into tracked artifacts, or committed.

A connector `403 Resource not accessible by integration` is an integration
authorization failure. It is not evidence that the shared token resolver or
operator-approved token failed, and it does not authorize connector retry or
the audited raw-`gh` exception.

## Split Issue And Code Repository Authority

The typed issue record's `repository` remains the issue-tracker authority. A
publication request may additionally set `code_repository` when the reviewed
code, branch, and pull request belong to a different canonical repository.
The resulting publication intent and evidence retain both identities:

- `issue_repository` identifies the issue that GitHub must close.
- `repository` identifies the code repository that owns the Git remote,
  branch, pull request, checks, and merge.

Split-authority publication requires a qualified closing keyword such as
`Closes danielbaustin/agent-design-language#5844`. An unqualified `Closes
#5844` is invalid because GitHub would resolve it against the code repository.

Before any push, `csdlc-publish` verifies every configured effective fetch URL
and push URL for the selected remote against the code repository. It then
requires the observed PR base and head repositories, refs, and exact head SHA
to match, exhaustively reconciles all pages of matching open PRs, and rejects
ambiguity. `csdlc-finish` uses the publication repository for PR authority but
does not derive terminal success until the separately identified issue is
observed closed through the exact qualified relationship.

Typed bind records `code_repository` before execution when code and issue
authority differ. Doctor compares that explicit identity with the effective
`origin`; it does not infer split authority from remote names. Publication must
reuse the recorded code repository. Omitting `code_repository` is the
backward-compatible same-repository mode; it retains the same exact remote,
linkage, review, and reconciliation checks.

## Install Contract

The v2 install/coexistence manifests must require every operational GitHub
owner binary:

- `csdlc-github`
- `csdlc-github-issue`
- `csdlc-github-pr`
- `csdlc-pr-state`
- `csdlc-publish`
- `csdlc-finish`

`csdlc-install install` must build and install the reviewed binary set into the
dedicated `.adl/bin/csdlc-v2/` generation directory. `csdlc-install verify`
must fail closed when any required binary is missing, non-executable, symlinked,
or built from stale provenance.

## Migration Rules

- Prefer `csdlc-github-issue` for issue actions.
- Prefer `csdlc-github-pr state --request <request.json>` or `csdlc-pr-state`
  for PR observation.
- Keep `csdlc-github run --request <request.json>` only as compatibility during
  migration.
- Use `csdlc-github runner-preflight --request <request.json>` for the bounded,
  read-only larger-runner eligibility and dispatch diagnostic documented in
  `docs/tooling/GITHUB_LARGER_RUNNER_PREFLIGHT.md`.
- Do not add new issue actions to `csdlc-github-pr`.
- Do not add new PR actions to `csdlc-github-issue`.
- Do not route publication or terminal operations through connector actions;
  keep publication under `csdlc-publish` and exact-head merge plus derived
  terminal retention under `csdlc-finish`.
- Unsupported GitHub workflow operations must fail closed until a repo-native
  Rust implementation exists.

## Proof Hooks

Focused proof for this boundary lives in:

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_route_policy`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `csdlc-install install --repo <repo> --destination <repo>/.adl/bin/csdlc-v2`
- `csdlc-install verify --repo <repo> --bin-dir <repo>/.adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json`

These checks prove that issue and PR actions are split, marker reconciliation is
exact, and the stable installed binary set cannot omit required GitHub owner
binaries.

# Typed PR-create install/provenance defect

Date: 2026-08-31
Actor: worker-6
Repository: agent-logic/agent-design-language

## Defect observed

While creating the #503 terminal-materialization PR through the typed v2
`csdlc-github-pr` route, the installed stable binary set under
`.adl/bin/csdlc-v2/` was stale relative to current source.

The first typed PR-create attempt failed with:

```json
{
  "code": "corrupt_record",
  "message": "unknown variant `pr_create`, expected one of `issue_create`, `issue_update`, `issue_comment`, `issue_close`, `issue_read`, `pr_state` at line 2 column 23",
  "schema": "csdlc.error.v1"
}
```

That proved the installed `csdlc-github-pr` binary did not yet support the
source-declared `pr_create` action.

An attempted reinstall using the installed stale `csdlc-install` then produced a
receipt with `schema: csdlc.install_receipt.v1`. Current source-side owner
provenance expects `schema: csdlc.install_receipt.v2`, so the refreshed
`csdlc-github-pr` failed closed with:

```json
{
  "code": "validation_failed",
  "message": "stale owner-binary provenance: installed receipt is malformed; run csdlc-install resolve, then reinstall the selected generation",
  "schema": "csdlc.error.v1"
}
```

## Recovery used

The stable v2 generation directory was reinstalled from current source with an
external FastWork Cargo target:

```sh
CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/csdlc-v2-install-refresh \
  cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml \
  --bin csdlc-install -- install \
  --repo /Users/daniel/git/agent-design-language \
  --destination /Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2
```

The resulting receipt had:

- `schema: csdlc.install_receipt.v2`
- `source_revision: git:22da530df4cbc2c085b4bdd949943a720bb78d36`
- `source_set_schema: csdlc.owner_source_set.v1`

Coexistence verification then passed:

```json
{
  "schema": "csdlc.coexistence_report.v2",
  "pass": true,
  "default_generation": "v2",
  "missing_v2_binaries": [],
  "present_forbidden_v2_binaries": [],
  "skill_count": 11
}
```

The same typed PR-create request then succeeded and opened PR #601.

## Follow-up need

Before V3-F cutover, the one-command path must make this class of mismatch
hard to hit:

- `csdlc-install resolve` should clearly select and execute a source-compatible
  installer when the installed generation is stale.
- A stale installed installer must not be able to refresh the generation into a
  receipt shape that the refreshed owner binaries immediately reject.
- Operator-facing output should remain compact; full Cargo build logs should be
  retained as evidence only when needed.

## Non-claims

This defect did not require raw `gh`, did not change GitHub through any
non-typed route, and did not affect the live issue reopen sweep.

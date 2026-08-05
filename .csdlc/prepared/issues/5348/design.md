# #5348 WP-23 Release Ceremony And Lifecycle Closeout Design

## Status

Execution packet for the final v0.91.8 release ceremony. It changes only
release documentation, bounded release evidence, and issue-local lifecycle
state before the post-merge tag and GitHub release actions.

## Objective

Publish an evidence-bound v0.91.8 release after WP-22, then close the milestone
sprint umbrella. The ceremony contains no product implementation, hidden
remediation, or v0.92 execution.

## Authority Boundary

The active claim names the exact release documents, #5809 supplemental
evidence, and #5348 issue-local lifecycle paths. Root main remains inspection
only; tracked changes occur in the bound FastWork worktree.

## Entry Gate

WP-22 PR #5811 merge `703ee31f2c02bb6c8fda7d6bc51ff7963075132e`
must be ancestral to the exact #5348 revision. Before release mutation, the
remote `v0.91.8` tag and GitHub release must both be absent.

## Ceremony Flow

1. Finalize release notes, plan, checklist, ceremony packet, and #5809 evidence.
2. Run focused docs/evidence validation and the release script preflight.
3. Obtain one exact-head review and merge a PR closing #5348 and #5809.
4. At the merge commit, use `adl/tools/release_ceremony.sh` to create and push
   the annotated tag, create the draft release, and publish it.
5. Verify tag/release identity and close #5595 with exact release references.

## Validation

This docs-only lane uses Markdown/path checks, JSON/YAML parsing,
`git diff --check`, exact Git identity, release-script preflight, and live
tag/release verification. It does not run Rust builds, Clippy, coverage, or a
broad test matrix.

## Circular Closeout Boundary

The release script's local typed-closeout gate includes #5348 itself, which
cannot be closed_out before its merge and release. The post-merge release
mutation therefore uses the script's explicit `--skip-sor-gate` option only for
that circular boundary. Typed #5348 closeout follows the release and does not
serve as evidence for an earlier gate.

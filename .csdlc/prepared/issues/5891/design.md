# Canonical ADL Repository Cutover Design

## Decision

Promote `agent-logic/agent-design-language` to canonical code authority while
retaining `danielbaustin/agent-design-language` as a public legacy repository
and active v0.92 issue tracker. New branches and pull requests target the Agent
Logic repository after cutover. Existing issues remain on the legacy repository
and may be closed by qualified cross-repository references.

## Cutover Boundary

The cutover has two independently reviewed content changes:

1. A destination change set updates current clone, badge, and remote execution
   defaults to the Agent Logic repository and retains a complete redacted
   continuity inventory.
2. A source-only README notice identifies the Agent Logic repository as the
   canonical development location. No other source content or setting changes.

The live cutover window begins only after Sprint 1 is terminal, source and
destination refs have an exact explained match, all open pull requests have a
disposition, every active worktree branch is preserved, and authenticated
destination push authority is confirmed.

The retained inventory is provisional while Sprint 1 remains active. It must
be refreshed immediately before activation; the current counts and refs are
planning evidence, not activation authority.

## Declared Scope

The destination content change owns exactly these 13 operational references:

- `README.md`
- `adl/README.md`
- `adl/src/aws_remote_validation.rs`
- `adl/src/bin/adl_aws_remote_validation.rs`
- `adl/tools/publish_adl_builder_image_codebuild.sh`
- `adl/tools/run_aws_spot_remote_validation_lane.sh`
- `adl/tools/run_nessus_remote_validation.sh`
- `adl/tools/run_v0915_multi_agent_quality_comparison.py`
- `adl/tools/setup_aws_codefriend_build_resources.sh`
- `adl/tools/setup_aws_spot_remote_validation_github_resources.sh`
- `docs/tooling/CI_LOG_ARCHIVE_S3.md`
- `tools/aws_remote_validation/src/aws_remote_validation.rs`
- `tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs`

The remaining owned paths are the two cutover documents, the issue-5891
evidence manifests, and the four prepared design/validation files. The exact
file allowlist is machine-readable in
`docs/repository-cutover/ADL_CANONICAL_REPOSITORY_CUTOVER_INVENTORY.json`.
Schema files, session records, lifecycle cards, `.csdlc/preparation/**`, source
code beyond the 13 references, workflow files,
remotes, GitHub settings, and repositories are outside this remediation.

## Continuity Model

- Code, clone, push, and new pull-request authority: Agent Logic repository.
- Existing v0.92 issue authority: legacy personal repository until each issue
  closes or is explicitly transferred.
- Existing worktrees: retain paths and branches; shared `origin` changes to the
  Agent Logic destination, with `legacy-origin` preserving the old URL.
- Historical links, issue citations, release links, evidence, and stable schema
  identities that are intentionally immutable remain unchanged and are listed
  by classification in the reference disposition manifest.
- Secret-dependent and OIDC-dependent workflows remain disabled or explicitly
  deferred until their named credentials and trust policies are recreated on
  the destination. Secret values are never inventoried.

## Execution Order

1. Freeze and record source/destination refs and repository settings.
2. Produce the complete active-work and operational-reference inventory.
3. Finish Sprint 1 using the existing legacy issue authority.
4. Prepare destination and source-notice branches from the common base.
5. Run focused validation and independent exact-head review.
6. Enable destination Actions and open the destination pull request.
7. Merge the destination change, then merge only the reviewed source notice.
8. Change the shared local `origin` to Agent Logic and add `legacy-origin`.
9. Verify destination push authority, focused CI, active-work continuity,
   canonical links, and non-destructive rollback.
10. After the destination and source-notice merges, generate distinct final
    source and destination full-ref manifests and obtain independent review of
    that terminal evidence before issue closeout.

## Rollback

Rollback changes only local remote names and URLs: set `origin` back to the
recorded legacy URL and retain the Agent Logic URL as `canonical`. It never
force-pushes, deletes refs, or rewrites history. Published pull-request merges
are not silently reverted; any content rollback requires a separately reviewed
PR.

## Stop Conditions

- Unexplained source/destination ref divergence before the cutover window.
- Any open pull request or active worktree without a continuity disposition.
- Destination push authentication or focused CI cannot be proven.
- A required action would delete, privatize, rewrite, or force-push the
  personal repository.
- Secret material would need to be printed or retained.

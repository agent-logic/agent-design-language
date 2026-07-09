# Finish Validation Path Ownership Registry

`pr finish` still fails closed for unknown changed paths.

That behavior is intentional. What changed in `#4418` is where known path
ownership is declared.

## Purpose

The finish-validation selector needs one bounded place to answer four questions
for a changed path:

- which owner binary or owner lane owns the surface
- which focused validation lane applies
- which proof role the path triggers
- whether the known classification is publication-sufficient

## Current Registry Surface

The registry currently lives in
`adl/src/cli/pr_cmd/finish_support.rs` as `FINISH_PATH_OWNERSHIP_RULES`.

Each rule declares:

- exact paths and optional path prefixes
- `owner_binary`
- `validation_lane`
- `proof_role`
- `publication_sufficient`

Matching is additive for registry queries. If more than one
`FINISH_PATH_OWNERSHIP_RULES` entry matches the same path, every matching
publication-sufficient rule contributes its owner/proof-role classification.
Shared paths must therefore either agree on the validation lane or carry an
explicit regression test proving the intended combined behavior.

## How To Declare A New Command Surface

When a new command or control-plane surface should be recognized by
`pr finish`:

1. Add the new exact path or prefix to `FINISH_PATH_OWNERSHIP_RULES`.
2. Set the owner binary or owner lane that is responsible for the surface.
3. Set the focused validation lane that should run when the path changes.
4. Set the proof role that should be triggered by the path.
5. Add or update a regression test in
   `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`.

If a path does not belong to any known rule, leave it unclassified and let
finish fail closed.

## Broad Runtime Owner-Lane Disposition

When validation-manager escalation reports the Rust fast lane as too broad for
ordinary PR-fast proof, `pr finish` may consume a tracked disposition only for
the broad runtime owner-lane case. The disposition must be repo-relative,
tracked or staged, and current for every `matched_paths` entry reported by the
active validation profile.

The executable proof command must be one of:

- `bash adl/tools/run_owner_validation_lane.sh runtime`
- `bash adl/tools/run_owner_validation_lane.sh runtime --build`
- `bash adl/tools/run_owner_validation_lane.sh all --build`

Minimal shape:

```yaml
schema_version: adl.release_gate_disposition.v1
issue: 5042
disposition: approved_with_runtime_owner_lane_proof
changed_release_gate_surfaces:
  - adl/src/long_lived_agent.rs
  - adl/src/execute/mod.rs
reviewer_or_review_mode: bounded runtime owner-lane review
focused_validation_run: bash adl/tools/run_owner_validation_lane.sh runtime --build
residual_ci_proof_required_before_merge: required
```

Finish rejects stale dispositions, missing paths, prose-only proof text, and
non-runtime proof commands for this escalation.

## Non-goals

- This registry does not replace the future validation manager.
- This registry does not auto-classify unknown paths.
- This registry does not make docs-only or wider owner-lane policy optional.

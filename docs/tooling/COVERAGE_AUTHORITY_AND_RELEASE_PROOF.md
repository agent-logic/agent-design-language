# Coverage Authority And Release Proof

The repository has two intentionally different coverage surfaces:

The nightly coverage is release-authoritative when the scheduled watchdog
completes successfully.

| Surface | Purpose | Authority |
| --- | --- | --- |
| `adl/tools/run_pr_fast_coverage_lane.sh` | Fast, changed-surface feedback for a pull request | Non-authoritative advisory feedback |
| `adl/tools/run_authoritative_coverage_lane.sh` | Full workspace and companion `adl-runtime` instrumentation | Authoritative when selected by the merge/release policy |
| `.github/workflows/nightly-coverage-ratchet.yaml` | Scheduled full workspace report with the 90% workspace and 80% per-file floors | Release-authoritative nightly watchdog |

PR-fast coverage is non-authoritative and may be incomplete by design. It must not be used to claim
release coverage, and a passing PR-fast result does not waive the authoritative
merge or release lane. The authoritative runner distinguishes the full
`full_authoritative_default_features` mode from the bounded
`bounded_policy_surface_pr` mode; the latter is still not a release claim.

Mechanical compile-fallout classification is also non-authoritative. The
tracked classifier accepts only exact governed import or argument-pass-through
diffs with compile proof for every hunk and behavioral proof for every owning
API path. Its machine receipt explains why the ordinary changed-file threshold
did not apply to that exact diff; it does not lower the 80% threshold,
allowlist a path, exclude a file from nightly/full coverage, or convert PR-fast
evidence into release proof. Any malformed, semantic, unmapped, or incompletely
proved diff fails closed through the ordinary changed-source coverage gate.
An accepted receipt binds the exact base and head identities, unified-diff and
mapping digests, changed hunk content, proof-manifest digest, and the digests of
compile and behavioral results produced by commands the classifier executes
from the tracked governed mapping inside a clean archive of the exact Git
revision, with only the classified worktree diff overlaid for authoring mode.
Unrelated mutable or untracked source and test files are never proof inputs.
Caller-authored `passed` strings or result
artifacts are not inputs. The receipt retains each exact command, exit status,
result digest, and evidence-log digest; replaying or substituting any revision,
diff, mapping, result, evidence log, or hunk content is rejected. Argument
pass-through classification additionally requires the added token to occur in
the mapped governed callee invocation under a constrained Rust callsite
grammar; comments, strings, macros, and callee-shaped decoys fail closed.
Import classification binds both the token spelling and its exact configured
Rust module path. The single-file unified-diff parser validates matching file
headers, exact hunk line counts, body prefixes, and end-of-input. The classifier
and mapping are loaded from the same clean Git-object archive as the proof
commands and their control digests are verified again after execution.

The nightly workflow currently sets `EXCLUDE_FROM_FILE_FLOOR_REGEX` to `^$`.
That means the report does not silently exempt active source files from the
80% per-file floor. If a future exception is required, it must be a reviewed
policy change with a named path and retained rationale, not an ad hoc workflow
edit.

## Proof boundary

`bash adl/tools/test_coverage_authority_contract.sh` proves the routing and
claim boundary without running instrumented coverage. The actual release claim
requires the corresponding authoritative workflow result and retained
`coverage-summary.json`; this contract test is not a substitute for that run.

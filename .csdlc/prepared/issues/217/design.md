# Issue 217 design: historical retention plus current native re-proof

## Context

PR #215 proved the repaired production ACIP/WSS path on Linux and macOS at
source revision `c640066f284a915b638add377cc4b0a2e221e6f9`. The later commit
that retained its eight platform files, validation manifest, and validation log
was not included in squash merge `a77519c3fca9f64752af41c9a2ebd396468891f7`.

Those ten artifacts are valid historical evidence, but they are not sufficient
current proof. Since c640, merged #191 legitimately changed protected path
`adl-runtime/Cargo.toml`. A validator must not silently approve that drift merely
because it is expected. The default authority is therefore a fresh native run
at the final #217 implementation head. The old c640 packet is restored only as
immutable provenance.

## Decisions

### 1. Freeze the historical packet exactly

Restore the ten c640 artifact paths without rewriting their bytes. The
machine-consumed denominator
`.csdlc/prepared/issues/217/historical-c640-denominator.json` freezes exactly
ten repository-relative paths and their SHA-256 digests. Historical validation
fails if the count, path set, or any digest differs.

The historical source lane runs the unchanged #209 validator in a detached
c640 worktree. It overlays the retained eight platform files at their original
paths and supplies the original GitHub environment:

- `GITHUB_ACTIONS=true`
- `GITHUB_REPOSITORY=agent-logic/agent-design-language`
- `GITHUB_WORKFLOW_REF=agent-logic/agent-design-language/.github/workflows/wp14-production-acip-repair.yml@refs/heads/codex/209-acip-authority-repair`
- `GITHUB_RUN_ID=31453636709`
- `GITHUB_RUN_ATTEMPT=1`

The wrapper cleans its detached worktree and reports the exact validated source
revision. It never treats historical success as current qualification.

### 2. Require a fresh current-head native packet

Issue #217 owns a narrow producer/workflow that runs the same two production
ACIP/WSS tests on Linux and macOS against the final implementation head and
writes a separate ten-file packet beneath `.csdlc/evidence/217`. A generated
current denominator freezes the exact ten current paths and digests after both
native jobs and aggregate validation pass.

No rebaseline is implicit. If a fresh run cannot be obtained, execution stops.
Any later proposal to approve a scoped rebaseline requires explicit operator
approval and a new reviewed design revision.

### 3. Validate retained proof at later heads

The issue-owned retained validator accepts a denominator as its entry point and:

1. requires exactly ten unique, repository-relative, evidence-confined paths;
2. verifies every denominator digest before parsing referenced artifacts;
3. verifies manifest identity, repository, workflow, run attempt, job IDs,
   successful job states, artifact identity, and every recorded digest;
4. verifies both receipt envelopes, canonical payload digests, platforms,
   producer digest, exact test inventory, command/log/semantic/source-manifest
   digests, path hygiene, assertion inventory, and cross-platform semantic
   equivalence;
5. derives one proved source revision from both receipts and the validation
   manifest;
6. accepts the source relationship only when the proved revision is ancestral
   to the validating HEAD or when the complete protected source manifest is
   digest-equivalent to the validating checkout; and
7. in both modes, requires every current protected path to equal the proved
   source manifest so post-proof protected drift always fails closed.

The equivalence mode is merge/squash/rebase-safe because it does not require the
source commit object to remain locally available. It is not a waiver: complete
protected-byte equality remains mandatory. The validator reports which source
relationship passed.

## Proof design

Focused fixtures cover:

- exact historical ten-path/digest denominator validation;
- executable detached-c640 source validation with the evidence overlay and
  original GitHub environment;
- fresh Linux/macOS current-head packet generation and semantic equivalence;
- retained validation at an ancestral later head;
- squash-equivalent history with no source commit ancestry;
- missing/extra/duplicate path and digest tampering;
- receipt runner, job, workflow, source, or manifest mismatch;
- semantic projection or assertion mismatch;
- protected-source drift; and
- unrelated/non-equivalent source failure.

The final #217 VPP/SOR command invokes the retained validator on the fresh #217
denominator at the reviewed PR head. The historical c640 lane is separately
reported as provenance-only proof.

## Boundaries

- No production Rust, ACIP, replay, API, Guardian, kernel, or runtime behavior
  changes.
- No modification of terminal #209 cards or derived terminal state.
- No AWS or cloud resources.
- No edits to #142 beyond typed linkage/commentary if required.
- Publication requires independent design review and later independent
  exact-head implementation review. Merge remains operator-controlled.

## Rollback

Revert the #217 repair PR. The immutable #209 GitHub Actions run remains
historical evidence; no production rollback is needed because this issue
changes only proof tooling, workflows, and retained evidence.

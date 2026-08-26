# Issue 217 design: historical retention plus current native re-proof

## Context

PR #215 proved the repaired production ACIP/WSS path on Linux and macOS at
source revision `c640066f284a915b638add377cc4b0a2e221e6f9`. Merge
`a77519c3fca9f64752af41c9a2ebd396468891f7` is a two-parent merge whose second
parent is c640, so c640 is ancestral to merged `main`. The later evidence
retention commit `b27b61597b7e6bc6563d6a7fef6f13ec9c6d3e98` is not ancestral
and its eight platform files, validation manifest, and validation log were not
merged.

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

The fresh proof has an exact two-head sequence:

1. `H` is the reviewed producer/workflow/validator implementation head.
2. GitHub Actions runs against exact `H` and uploads the Linux, macOS, and
   aggregate artifacts without changing the branch.
3. `H2` adds only the downloaded current evidence, its exact ten-path
   denominator, an H2 retained-surface manifest, and the explicitly named
   issue-lifecycle paths frozen by
   `.csdlc/prepared/issues/217/h2-retention-allowlist.json`. It changes none of
   the protected, unprotected-source, proof-contract, design, or workflow
   paths.
4. The workflow path filter includes proof tooling/workflow and the protected
   source paths, while a successful classifier evaluates the latest pushed
   commit range. The native matrix and aggregate run only when that delta
   touches a protected or proof path. Evidence/lifecycle-only H2/H3 synchronize
   pushes therefore retain a truthful classifier check without recursively
   launching another native run; opened, reopened, manual, or unverifiable
   ranges fail safe by running the native proof.
5. Before parsing proof content, the retained validator compares
   `git diff --name-status H..H2` with the machine allowlist. The current
   evidence denominator, its ten unique paths, and the retained-surface manifest
   are required; every changed lifecycle path must be one of the fourteen exact
   paths named by the allowlist. Deletions, renames, copies, unmerged states,
   duplicate paths, or any other changed path fail closed.
6. The eight paths in
   `.csdlc/prepared/issues/217/proof-contract-paths.json` are digest-bound at H
   and must be byte-identical at H2. This independently freezes the historical
   wrapper, producer, validator, workflow, both denominators, the proof-path
   denominator itself, and the H2 allowlist.
7. The H2 retained-surface manifest records exactly nineteen unique
   repository-relative path/digest entries: the current denominator, its ten
   evidence paths, and all eight proof-contract paths. It deliberately does not
   digest itself. Lifecycle paths are also excluded because typed truth advances
   after review.
8. An independent reviewer reviews exact H2. A later `H3` (or later head)
   retains a review receipt, because a receipt cannot truthfully authenticate
   the review of the same commit that contains it. The receipt binds H, H2,
   both tree identities, the canonical `name-status` diff digest, both
   evidence/source denominators, all eight proof-contract path digests, the H2
   retained-surface manifest digest, reviewer identity, reviewed scope, result,
   and no-drift verdict. `H2..H3` may add only that receipt and named lifecycle
   changes.
9. The receipt is not trusted merely because its contents are internally
   consistent. The validator finds the unique commit on current-HEAD ancestry
   whose tree contains the receipt path while every existing parent lacks it.
   A normal merge with one receipt-bearing parent is therefore not a second
   addition. The anchor is H3 for retained branch ancestry or the
   squash/integration commit when branch commits are not retained. The anchor
   commit and receipt blob must remain available, and the current receipt must
   equal the anchored bytes, Git blob identity, and SHA-256 digest. A later
   coherent receipt-plus-manifest rewrite therefore fails even if all rewritten
   fields agree with one another.
10. At H3 and later the validator authenticates the anchored reviewed-H2
   receipt and compares the current checkout with every path and digest in the
   H2 manifest.
   This retained-surface equality is reconstructible even when the H2 commit and
   tree objects are unavailable; only those H2 objects may be missing. The H3
   or integration anchor objects are mandatory. H2 ancestry, when available, is
   additional evidence rather than a prerequisite. Independent current equality
   for all seventeen protected paths remains mandatory.

### 3. Validate retained proof at later heads

The independent machine-consumed denominator
`.csdlc/prepared/issues/217/protected-source-denominator.json` freezes the exact
17 protected paths. The producer must emit exactly that set into each source
manifest; the validator compares each platform manifest and the current tree
against the same reviewed denominator and rejects a missing, extra, or duplicate
path before checking content digests.

The issue-owned retained validator accepts an evidence denominator as its entry
point and:

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
   source manifest so post-proof protected drift always fails closed;
8. validates the exact H-to-H2 changed-path/status set, the H-to-H2
   byte-identity of every proof-contract path, and the exact nineteen-entry H2
   retained-surface manifest;
9. validates that H2-to-H3 adds only the review receipt and named lifecycle
   paths; and
10. locates exactly one ancestral introduction commit for the receipt path,
   requires its commit/blob objects, and compares current receipt bytes, Git
   blob identity, and SHA-256 with that anchor; and
11. at H3 or later, validates the anchored reviewed-H2 receipt, its
   H/H2/tree/diff/manifest/denominator/proof-digest/reviewer bindings, and every
   retained-surface path/digest without requiring H2 commit or tree objects.

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
- protected-source drift;
- arbitrary unprotected-source drift such as a README, lockfile, or unrelated
  source file;
- historical-wrapper, producer, validator, proof-denominator, allowlist, or
  workflow drift;
- H-to-H2 deletion, rename, copy, unexpected status, or unlisted path;
- missing, extra, duplicate, or tampered H2 retained-surface entries;
- missing, stale, forged, or self-referential H2 review receipt;
- coherent later receipt-plus-manifest rewrite against the retained ancestral
  receipt blob;
- missing, ambiguous, non-ancestral, or blob-missing receipt anchor;
- disallowed H2-to-H3 additions or modifications;
- a later-head success fixture with H2 refs and objects unavailable while the
  H3 or integration anchor remains; and
- unrelated/non-equivalent source failure.

The final #217 VPP/SOR command invokes the retained validator on the fresh #217
denominator at the reviewed PR head. The historical c640 lane is separately
reported as provenance-only proof.

### 4. Tracked stop gate

The preparation lifecycle is deliberately serial:

1. independent review passes this authored design, diagram, historical
   ten-path denominator, and protected 17-path denominator;
2. typed design approval is recorded;
3. typed binding occurs only to unlock the lifecycle-authorized STP acceptance,
   SPP step, and VPP lane corrections identified by the first review;
4. those card repairs are applied without product, workflow, validator,
   evidence, publication, or PR changes;
5. a second independent reviewer passes the complete bound six-card package;
6. only then may implementation begin, followed by exact-head implementation
   review, publication, the `H` native run, exact-allowlist `H2` retention with
   the retained-surface manifest, independent exact-H2 review, allowlisted `H3`
   receipt retention, and final object-independent later-head validation.

Any missing review or unresolved finding stops the sequence. In particular,
there is no implementation or publication between binding and the second
full-package review.

## Boundaries

- No production Rust, ACIP, replay, API, Guardian, kernel, or runtime behavior
  changes.
- No modification of terminal #209 cards or derived terminal state.
- No AWS or cloud resources.
- No edits to #142 beyond typed linkage/commentary if required.
- No implementation or publication before the second independent full-package
  review passes after typed card repair.
- Publication requires independent design review and later independent
  exact-head implementation review. Merge remains operator-controlled.

## Rollback

Revert the #217 repair PR. The immutable #209 GitHub Actions run remains
historical evidence; no production rollback is needed because this issue
changes only proof tooling, workflows, and retained evidence.

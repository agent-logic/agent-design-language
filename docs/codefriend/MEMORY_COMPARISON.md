# Compatible review comparison

`adl codefriend memory` retains and compares shared CodeFriend `ReviewRecord`
artifacts against live admitted evidence. It does not run a new source review,
call a provider, execute repository code, or implement Memory Palace.

Retain two producer review artifacts and save the returned references:

```sh
adl codefriend memory retain --store evidence-store --baselines review-baselines --input first-review.json > first-ref.json
adl codefriend memory retain --store evidence-store --baselines review-baselines --input second-review.json > second-ref.json
adl codefriend memory compare --store evidence-store --baselines review-baselines --baseline first-ref.json --current second-ref.json --out delta.json
adl codefriend memory delta-read --store evidence-store --baselines review-baselines --input delta.json
```

`memory read --store evidence-store --baselines review-baselines --reference
first-ref.json` reconstructs the validated shared review. `memory delete` with
those same flags deletes a baseline and writes a deletion marker. Deletion is
idempotent and prevents that run from being retained again in this baseline
store. Missing inputs, altered references, incompatible artifact schemas and
provenance failures are errors; commands do not bootstrap an absent admission
store. Baseline and admission directories must be separate. Delta outputs must
be outside both managed directories and are created without overwriting files.

## Matching and compatibility

Finding identity remains the shared repository, perspective, rule and semantic
anchor identity. Titles, prose, source locations and evidence IDs never create
new identities. For matched IDs, assessment equality compares title, severity,
rationale, confidence, inference and limitations. A prose change is `changed`
under the same identity. Revision-bound evidence IDs and scope IDs are excluded
from assessment equality; unchanged assessments at moved locations can remain
`unchanged`, with before/after revision and evidence references retained.

The shared `Comparison` validator uses this same assessment equality. Matching
is deterministic and ordered by finding ID. Finding disappearance is `resolved`
only when both reviews have complete, matching coverage. Different repositories,
scopes/included/excluded paths, lane or policy versions, provider routes, or
incomplete review states produce `not_comparable` with explicit reasons and no
resolved findings. Invalid shared schemas fail validation before comparison.
An empty but compatible comparison is explicitly comparable with zero changes.

## Baseline adapter contract for Memory Palace

`BaselineAccess::load(BaselineRef)` returns a validated `ReviewRecord` matching
the reference's run ID, packet ID and immutable record digest. The comparison
consumer independently checks the returned reference identity, even for custom
backends. The current `AdmittedBaselines` implementation stores sorted Run and
Finding data and reconstructs the Admission through `Store::get` on every load.
It never retains another copy of admitted source. A changed finding set cannot
replace a retained run under the same run ID; duplicate finding IDs are errors.

A future Memory Palace adapter must preserve these exact identity, compatibility,
retention and deletion checks and call the same comparison function. It must not
supply stale cached admission or implement a separate matcher. This contract is
bounded single-store retrieval, not organizational or cross-user memory.

Artifacts are limited to 16 MiB and the baseline directory to 128 run identities,
including deletion markers. Files are created privately and cannot overwrite
existing output. Paths reject parent traversal and symlinks. Baseline access is
serialized with an exclusive lock. Deleting or expiring an admission immediately
prevents baseline and saved-delta reads. Baseline deletion also invalidates saved
deltas. Previously exported review/delta copies are separate files; deleting a
store record does not erase those copies. Finding prose and exported reviews
must be handled under the same privacy policy as admitted evidence.

## Proof

`MEMORY_COMPARISON_PROOF_INVENTORY.json` declares the runtime proof lane.
`codefriend_cf_memory` exercises the production adapter, matcher and CLI.
The `codefriend_memory_fixture` example constructs inert fixture reviews using
production acquisition, admission and shared Run/Finding constructors. It feeds
`adl/tools/codefriend_memory_installed_proof.py`, which requires an isolated
installed binary with installation provenance. Fixture assessments are known
inputs to this comparison consumer, not claims of automated source analysis.

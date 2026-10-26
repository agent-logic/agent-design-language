# Remaining utility migration (#1351)

Status: **in progress; not accepted or ready for publication**. Native issue
preparation and FastWork binding succeeded. The issue remains responsible for all
60 baseline Python paths, the later #1275 adapter checker, and executable inline
payloads. No exception or completion-by-omission is accepted.

## Completed source slice: adapter report consistency

`tools/public_contracts/src/adapter_evidence.rs` replaces the #1275 Python report
checker. Run:

```sh
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- adapter-evidence
cargo test --locked --manifest-path tools/public_contracts/Cargo.toml --test adapter_evidence
```

This is offline report consistency, not authenticated lifecycle or product
acceptance. The original checker passed before removal. Native tests cover the
current report, truthful incomplete reporting, mismatched identity/package/stage
claims, false aggregate success, missing evidence and duplicate repositories.
Missing identity fields cannot accidentally compare equal. Package-binding
ambiguity is rejected rather than silently overwritten.

`utility-dispositions.json` retains exact Git mode/blob/SHA256 custody for each
completed removal. No source payload is archived or embedded. Original #1275
reports remain unchanged. Remaining utilities, callers, final guard overlays,
independent review and CI are still required before this issue is delivered.

## Completed source slice: dependency cache warming

The Rust cache helper replaces the implementation and five Python regression
cases. The original tests passed before removal. Ten native tests preserve
artifact selection, multiline workspace manifests, dep-info/build exclusions,
explicit replacement, rollback/restoration, wrapper defaults, JSON/output-file
behavior and the optional cross-device skip. Direct CLI tests run with empty
`PATH` and verify successful JSON, runtime failure exit 2 and setup failure exit 1.
The cross-device case ran on this host using the available second filesystem.

The same-filesystem wrapper and direct dry-run interface remain available at
`adl/tools/rust_validation_warm_cache.sh` and
`adl/tools/warm_rust_dependency_cache.sh`. They invoke Rust through Cargo; the
first use may compile the helper. Warmup remains acceleration, never proof.
Current lane selectors and documentation point to the native implementation.

Safety corrections are explicit: source artifact/profile symlinks and destination
parent escapes are refused, wildcard workspace membership requires explicit
classification, and a failed rollback retains its original backup instead of
silently deleting it. Full TOML parsing replaces the old Python-version fallback.

## Completed source slice: planning draft templates

Native `planning-fill` and `planning-validate` commands replace both Python
helpers, with shell entrypoints preserving their explicit registry/template/
values/input/output options. All 11 active templates produced byte-identical
original/native drafts and passed both structural validators from outside the
repository before source removal. Six native test groups cover scalar/list/object
rendering, sorted missing keys, non-overwrite on unresolved values, explicit
partial drafts, required headings, malformed/inactive/version-mismatched
registries, missing inputs, absolute paths and symlink escapes. CLI tests run
with empty `PATH`. Draft generation and structural success retain their explicit
non-approval/non-lifecycle claim boundaries. Current documentation and registry
callers are switched; historical review assignments remain documentary evidence.

## Completed source slice: supported architecture and showcase demos

Both supported v0.90 demos retain their generators and packet contents, with
native validators behind new shell entrypoints. The architecture packet retains
its structural `proving` label; the showcase remains `non_proving`, with
publication disabled and the original staged quality-evaluator boundary. These
labels do not confer private owner or release acceptance. Current commands and
generated validation instructions now name Rust/shell tooling.

Both original demo suites passed before migration. Four native test groups run
the actual demos with failing Python/Ruby traps and verify packet existence,
manifest identities, ordered sections/evidence, publication and staged-lane
truth, leak refusal, existing-output preservation and dangling-symlink refusal.
No trap was invoked. The complete original required-file/diagram/section lists
remain native constants. Safety checks now reject ambiguous duplicate skill
lanes and single-backslash Windows host paths; leak diagnostics identify the
category without echoing potentially sensitive matched content.

## Completed source slice: prompt schema artifact smoke checks

The native `prompt-schemas` command and shell entrypoint replace the Python smoke
checker and the inline active-registry assertions in the retained validation
wrapper. Both wrappers retain their sunset-source absence guards and explicit
non-lifecycle/non-equivalence claims. The native default checks preserve schema
identity, version, card kind, template path, parser provenance, array shapes and
nonempty headings. `--active-registry` also requires active status, matching
registry versions, exactly six lifecycle cards and existing template/schema files.

The original checker passed all six versions (1.0.0–1.0.5) before removal.
Three native test groups cover those versions, malformed/missing schemas,
identity/shape mutations, invalid options, active-registry mutations and CLI
execution from outside the checkout with empty `PATH`. An independent Node JSON
parser read all 33 tracked schemas successfully; Python readability is no longer
claimed as an executed native check. Current instructions use the JSON artifact
smoke check. Schema artifacts and historical review assignments remain unchanged.

## Completed source slice: source tool registry

The Rust `tool-registry` generator replaces the Python implementation and its
two directory-inventory tests. Before removal, both original tests passed and
the complete original/native rendering matched byte-for-byte after substituting
only the generation-command spelling. Three native groups cover empty nested
directories, populated directory assets, classification precedence, explicit
binary-only inventory, counts, skill ownership notes, stale-output refusal and
historical-file preservation. Strict Clippy and the current CLI owner-guidance
guard passed. Cargo TOML is now parsed structurally; invalid manifests fail.

The live registry is regenerated from current source, including the new shell
entrypoint and deleted Python tests. Prior hand-edited rows are reconciled to the
same generator rules; Rust tests outside `adl/tools` are not mislabeled as tool
directory entries. The immutable v0.91.5 registry is untouched. Current wrappers
use the native command; source inventory still grants no installed admission.

## Completed source slice: three retained packet validators

Native `evidence-bundle`, `transition-dag` and `csdlc-demo-proof-contract`
commands replace the corresponding Python validators. Their shell entrypoints
accept the same packet-root argument and retain success markers and usage exit
codes. The native checker reports all missing requirements, preserving the
proof-contract validator's aggregate diagnostics. It validates packet structure,
not the truth of historical execution or authenticated lifecycle acceptance.

All three original shell suites passed before removal. Three native groups
exercise the unchanged tracked packets, independently remove each required file
and snippet, reject absent roots, and run the CLI with empty `PATH`. The proof
contract's two additional shell assertions remain explicit native assertions.
Inline Python fixture mutation is removed from both old negative-test wrappers.
Strict Clippy passed. Current proof-lane and quality-gate callers use the native
shell entrypoints; those larger mixed-language lanes remain outside this slice's
execution claim. Historical packets and their original command records remain
unchanged. Replays now use `bash adl/tools/validate_<name>_packet.sh <packet-root>`.

## Completed source slice: packet claim and citation boundaries

Native sprint-console, first-proof-readiness and ObsMem handoff validators keep
the original claim boundaries. The console requires an estimated timebox and
explicitly partial literal five-minute claim. Readiness requires every upstream
proof file and all three canonical manifest path identities, without claiming
execution. ObsMem checks source-record kinds, truth labels, tracked relative
paths, suffixes, actual source files, citations and nonempty outside-memory data.
Supporting artifact path validation retains its original syntactic scope.

All three original shell suites passed before removal. Three native groups
preserve their negative cases and diagnostic substrings, add truth/identity/path
mutations, and run successful CLIs with empty `PATH`. The console's original
HTML/CSS/JavaScript assertions remain explicit tests. Strict Clippy passed.
Current caller commands are updated; broader mixed-language lanes remain
unexecuted by this slice. Historical proof documents are unchanged.

## Completed source slice: retained merge-readiness packet

The native merge-readiness checker preserves the required file/section inventory,
fixed historical snapshot identities, safe relative path checks, evidence/review
file existence and Markdown/snapshot consistency. SOR existence remains outside
this historical validator's original contract. This never authorizes a live
merge. Integer aliases for JSON Boolean truth fields are now rejected.

The original shell suite passed. Two native groups cover missing files/markers,
every required snapshot key, stale PR state, malformed JSON, Markdown drift,
unsafe paths, missing proof files and interpreter-free CLI success/usage failure.
Strict Clippy passed. Current shell callers use the native replacement; historical
records retain original command text. The adjacent quality-review validator has
a pre-existing demo-map route mismatch (v2 doctor expected, v1 wrapper recorded);
that observed failure is not claimed as successful validation.

## Completed source slice: quality review surface checker

The native checker retains all three surface inventories, linked-file checks,
feature/route assertions and missing-feature verdict. Historical command strings
are documentary comparisons and are never executed. Two original surfaces pass;
the original demo-coverage check fails because it expects a v2 doctor route while
the immutable map names the older v1 wrapper. The port preserves that refusal.
Two native groups verify these observed outcomes plus synthetic positive and
negative markers, links, routes and verdicts. Strict Clippy passed. No successful
current demo-coverage validation or availability of those old routes is claimed.

## Completed source slice: HTML game packet

The native HTML game packet checker preserves partial browser/gameplay proof,
explicit not-run interactive QA, passed helper-URL evidence, required artifacts
and rejection of both original playable-artifact overclaims. Original field
parsing accepts a quoted first value or an expected value followed by a comma.
The original shell suite passed. Native packet-truth tests retain its result
mutation, HTML/CSS/JavaScript checks and add required-file/snippet, claim-field
and overclaim negatives. All four packet-truth groups and strict Clippy passed.
No browser execution or gameplay acceptance is inferred from these checks.

## Dependency integration: merged historical retirement

#1347 merged through PR #1360 at `d1f3626762f7c222f38dba048319b47c802ad7ac`.
The utility branch incorporates that delivery. Its historical preservation test
now pins this reachable merged snapshot: the pre-merge `7a0b018b` commit is not
an ancestor after squash merging. The exact original-source custody and current
HEAD absence checks remain unchanged. This corrects the merge-topology assumption
without weakening preservation or requiring an abandoned PR commit to remain
fetchable. Full #1351 transition overlays and acceptance remain pending.

## Completed source slice: retained PVF manifest validation

The native PVF checker preserves the v0.91.4 manifest schema, exact allowed keys,
required lane fields, enum domains, lane identifiers, nonempty/unique declared
classes and equality to actually used lane classes. It accepts multiple files,
retains usage/failure statuses and never executes declared commands. Timeout
values must be positive unsigned integers; Boolean aliases are rejected.

Two native groups cover malformed shapes, missing/unknown fields, class mismatch,
invalid identifiers, optional-field types and interpreter-free CLI operation.
Strict Clippy passed. The runner fixture suite passed before and after the validator migration;
its inline Python was subsequently replaced in the retained PVF runner slice below. The modern
`docs/validation/pvf_lanes.json` uses a different schema and remains outside this
retained validator's contract; its rejection is not a regression or passing proof.

## Retired historical generators

Removed the v0.91.7 outlier-analysis builder, RD-01 split-inventory builder and
#4645 live-state snapshot builder after the retirement-first usage assessment.
Their historical reports and snapshots remain unchanged. Exact source bytes,
Git mode/blob and SHA256 are recorded in `utility-dispositions.json`; no source
archive or replacement generator was added. Generation options were configurable,
but the discovered consumers were historical records, not current public lanes.
No execution frequency is inferred from the absence of current lexical callers.
The full retirement transition overlay and issue acceptance remain pending.

## Retired historical provider packet replay

Removed the three v0.91.5 OpenRouter/Gemma/comparison Python validators and their
three shell tests together. Their executable references form a historical replay
family; neither current CI nor either lane-selector registry selects these tests.
The tooling README now records retirement, exact source/test custody is retained,
and the original review packets remain unchanged. This deliberately removes old
replay capability rather than maintaining new native implementations of completed
experiments. Owner qualification obligations remain separate and unfulfilled by
this removal. Full transition admission and issue acceptance remain pending.

## Current public release-version check

`check_release_version_surfaces.sh` is selected by public CLI validation. Its
three Python TOML/JSON readers are replaced by native `release-version-source`
and `release-version-metadata` helpers. Shell retains Cargo execution, temporary
metadata cleanup and the exact CLI-version comparison. Source/lock disagreement
still stops before metadata and CLI execution. Missing/malformed data refuses;
versions must be nonempty strings in the manifest. No dependency was added.

The original shell check passed before editing. The updated actual check passed
with failing Python/Python3/Ruby commands ahead of PATH, reporting 0.92.1 for
manifest, lock and CLI. Two focused native test groups cover missing/malformed
inputs, manifest/lock/metadata drift and empty-PATH helper execution; strict
Clippy passed. This is version consistency, not a deployed-release claim.

## Retired historical suitability and Polis replay

Removed the v0.91.6 DeepSeek suitability validator, its import alias and shell
test, plus the v0.91.4 Software Development Polis packet validator and shell
test. Outside historical/inventory records, references were confined to each
family. Current CI and lane selectors contain no registrations, and a scan of
tracked `.csdlc` shell scripts found no stem references. The import alias was
removed with its implementation, not left as a broken wrapper. Historical
packets, current authority rules and separate owner qualification obligations
remain unchanged. Exact source/test custody is recorded; no replay replacement
is added. Full guard admission and issue acceptance remain pending.

## Retained QTS and Windows status-source checks

The two registered status-source validators only search for required literal
fragments. Shell successors preserve those lists, success messages and aggregate
missing-fragment diagnostics without adding Rust code or executing host/SSM
operations. The conditional lane registry now names the shell entrypoints.
Both original Python checks passed. Successors passed two positive checks, all
21 single-fragment deletion mutations and two missing-file refusals with failing
Python/Ruby commands ahead of PATH. These are source-shape checks only; they do
not prove correct JSON escaping, PowerShell execution or actual host status.

The same positive, per-fragment and missing-file cases are retained in
`tools/public_contracts/tests/status_source.rs`, which invokes the actual shell
scripts. Its Cargo regression passed and is admitted by the native proof plan.
Direct Bash validator declarations were refused by the installed allowlist;
that attempt is preserved in local receipts, not claimed as successful admission.

## Retained PVF runner data processing

The retained PVF runner remains registered in the conditional v0.91.6 lane
selector. Its inline Python plan construction, changed-path matching and JSON
report encoding now use native helpers. Lane execution stays in the existing
shell scheduler. Lane declaration order, glob/prefix matching, credential and
worktree gates, status precedence and report fields are preserved. Newline/tab/
carriage-return fields that cannot safely fit its TSV plan are explicitly refused.
JSON whitespace/key order may change; report field values and scheduling declaration order do
not. The manifest validator still targets its retained v0.91.4 schema.

The original shell fixture suite passed against the new runner before replacement
of that suite. Two native groups preserve the original fixture cases, parallel
starts, plan-only nonexecution, credential refusal/explicit allowance, release
gate precedence, skipped nonmatching lanes, invalid-manifest refusal, JSON escaping
and path matching. The actual runner test uses failing interpreter traps. Both
groups and strict Clippy passed; the shell test entrypoint now runs these native
regressions. This does not assert execution frequency or qualify private lanes.

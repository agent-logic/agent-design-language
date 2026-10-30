# Python necessity assessment for #1351

Assessment baseline: `d050007654dacdbfc78b20ec0354ad2c73ecfac3`, with the
unfinished PVF runner changes present. There are 41 Python files remaining in
this worktree; main still contains 61. Twenty replacements on this branch are
not merged. This assessment does not declare #1351 complete.

## Decision rule

Do not port a utility merely because it exists, has tests, or is mentioned in a
historical record. Retire obsolete capabilities with exact Git custody and
preserved reports. Replace only a capability with a demonstrated current public
consumer or requirement. Owner-only functionality should remain with its owner;
a public adapter needs a separately justified public contract.

Before further ports, trace current entrypoints, distinguish assertions about a
filename from actual execution, and follow indirect callers. Reconsider already
ported historical packet checkers by the same rule; previous implementation
effort is not evidence that a feature is needed.

## Frequency evidence and limits

The current `.github/workflows/ci.yaml` routes ordinary PR validation through
Rust contract suites and `tools/public_adl/run_ci.sh`. That script invokes the
public CLI, package checks and native public-contract tests. It does not directly
invoke these 41 Python files. CI still has a Python version preflight; removal
requires completing its actual dependency audit, not merely deleting that line.

The old lane selector registers some scripts for conditional/manual use. Such
registration proves discoverability, not execution frequency. No CI log sample
or developer invocation telemetry has been collected. Therefore this assessment
does not assert that a script has never run, or give an invented run count.

`python-usage-assessment.json` records literal basename references across all
tracked files. References are candidate edges, not a call graph. Historical and
inventory references are separated. Dynamically constructed names/imports may
not be found; identically named scaffold scripts share lexical matches.

## Immediate retirement candidates

| Script | Evidence | Disposition before source removal |
| --- | --- | --- |
| `adl/tools/build_v0917_execution_outlier_analysis.py` | Defaults to v0.91.6 metrics input and v0.91.7 report outputs, with configurable input/output arguments; no current reference candidates outside historical/inventory records | Retire generator; preserve reports and exact source identity |
| `adl/tools/rd01_repository_audit.py` | RD-01 split inventory with configurable generation baseline/output and a pinned historical verification baseline; no current reference candidates outside historical/inventory records | Retire one-off builder; preserve split inventory and custody |
| `docs/reviews/v0.91.7/internal-review-4645/live-state/build_internal_review_state.py` | Writes old #4645 review snapshots and queries local lifecycle/GitHub state; no current reference candidates | Retire snapshot generator; preserve recorded snapshots unchanged |

These are proposed dispositions, not completed removals. Check dynamic callers,
record Git mode/blob/hash and update the exact retirement guard before acceptance.
Do not execute the old live-state generator merely to validate removal: it writes
historical outputs and queries live remote state.

## Required next decisions

- Historical packet/demo validators: inspect their shell wrappers and whether
  those wrappers still belong to a supported public lane. A test calling only
  its own validator does not establish ongoing need. This includes v0.91.5
  comparison/OpenRouter/Gemma, v0.91.6 suitability, old Polis, first-proof and
  v0.92 coverage packets.
- `revision_validation_inventory.py` and its test: inspect imports; filename
  references alone omit Python module imports. Do not retire based on the sparse
  lexical reference result.
- Validation manager/inventory/selector: current tooling docs, shell wrappers and
  lane registrations demonstrate consumers. Determine the minimal retained
  public behavior and any native successor before deciding replacement scope.
- Cache/planning/schema/current command-guidance utilities: retain only documented
  current requirements; their completed replacements still need final review.
- Owner adapters and qualification tests: establish public consumer obligations
  separately from owner qualification. `tools/groupd_validation/src/lib.rs`
  invokes Python adapters, so these are not orphan files. Replacing an adapter
  with a native wrapper that still executes owner Python does not meet a claim
  of an interpreter-free lane.
- Aptitude demo fixtures: named by a public demo; decide supported demo status
  before removing its protocol or validator.
- Repository scaffolds: their own CI and Node tests reference the Python checker.
  Treat these as executable examples, not inert Markdown; inspect nested policy.
- Skill analyzers: inspect installed native owner equivalents and current skill
  entrypoints; documentation alone is not proof of execution frequency.

## Acceptance for this assessment

All 41 current paths are represented in the reference inventory. No source has
been deleted by this assessment. No claim is made that all 41 need replacement,
that the optional lanes run regularly, or that existing ports are all necessary.
The next implementation decision must cite a current requirement or a supported
retirement rationale, and preserve the original issue's full accounting scope.

Independent read-only assessment review found one wording issue: historical
defaults were described as fixed inputs. Corrected to distinguish configurable
generation from pinned verification. No other assessment findings were reported.

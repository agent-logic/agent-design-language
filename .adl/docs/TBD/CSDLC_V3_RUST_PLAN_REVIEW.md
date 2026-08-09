# C-SDLC v3 Rust Plan Review Record

Status: Pre-PR findings incorporated; final exact-revision verification passed

Issue: #73

Initial reviewed revision: `13aad5fd8039661f1bbbcaff703ee8d50f17c330`

Reviewed paths:

- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`

## Reviewer Identity And Evidence

| Reviewer | Provider-asserted model | Initial result | Evidence |
| --- | --- | --- | --- |
| Gemini | `gemini-3.1-pro-preview` | Request changes | `.csdlc/evidence/73/provider-reviews/initial-gemini-result.json` |
| Claude | `claude-sonnet-4-6` | Request changes | `.csdlc/evidence/73/provider-reviews/initial-claude-sonnet-result.json` |
| Claude diagnostic | `claude-opus-5` | HTTP 200 with empty text; not accepted as review | `.csdlc/evidence/73/provider-reviews/initial-claude-opus-result.json` |
| Claude diagnostic retry | `claude-opus-5` | HTTP 200 with empty text; not accepted as review | `.csdlc/evidence/73/provider-reviews/initial-claude-opus-r2-result.json` |

Model identity is provider-asserted. Provider reachability and successful text
extraction do not make model findings lifecycle authority.

## Gemini Review

### G-01: V2 writer revocation is missing

- Severity: P0
- Disposition: Incorporated
- Change: The migration now archives exact v2 state, removes the canonical v2
  index, writes a durable `migrated_to_v3` writer fence, requires v3 to observe
  the fence before mutation, updates supported v2 tools to reject fenced writes,
  and makes CI reject reintroduced v2 authority.

### G-02: Remote intent commit is absent from the transaction sequence

- Severity: P1
- Disposition: Incorporated
- Change: The plan now separates durable pre-network intent commit from
  post-readback state reconciliation and requires crash-resumable intents.

### G-03: Remote and terminal issue is overloaded

- Severity: P1
- Disposition: Incorporated
- Change: The former V3-13 is split into GitHub read-only observation, PR
  mutation/foreground watch, and finish/cleanup issues.

### G-04: OS signals do not reach structured cancellation

- Severity: P2
- Disposition: Incorporated
- Change: Root signal handling, cancellation-token propagation, task joining,
  OS-child termination, and bounded output drain are explicit.

### G-05: Cancellation lacks a distinct exit code

- Severity: P2
- Disposition: Incorporated
- Change: Exit 130 is reserved for interrupted/cancelled invocation outcomes.

### G-06: Cleanup path identity is underspecified

- Severity: P3
- Disposition: Incorporated
- Change: Cleanup requires canonical path equality with the verified Git
  worktree root and rejects prefix or relative-path matching.

## Claude Review

### C-01: Dependency graph lacks repository-to-adapter stabilization

- Severity: P0
- Disposition: Incorporated
- Change: V3-04 has a reviewed adapter-interface checkpoint; V3-05 consumes it
  through fakes; V3-09 waits for the V3-05 repository observation contract.

### C-02: Windows commit guarantees are unresolved

- Severity: P0
- Disposition: Incorporated
- Change: V3-08 must prove a per-platform synchronization and replacement
  matrix. Windows mutation remains fail-closed read-only if equivalent
  durability is not proven; the plan does not silently weaken its claim.

### C-03: Lazy application fields have ambiguous cell types

- Severity: P1
- Disposition: Incorporated
- Change: The `App` design now classifies sync and async lazy fields explicitly,
  caches typed initialization results, and makes a sync-to-async change an
  architecture revision.

### C-04: Cancellation propagation is underdefined

- Severity: P1
- Disposition: Incorporated
- Change: One root `CancellationToken`, signal wiring, `JoinSet` drain order,
  cancellation-aware waits, and OS-child termination are now specified.

### C-05: Importer schema, retention, and unsupported-field behavior are vague

- Severity: P1
- Disposition: Incorporated
- Change: V3-01 owns a versioned normalized import schema and retention policy;
  unsupported fields block v3 mutation pending reviewed field dispositions.

### C-06: Quantified size and effort claims lack methodology

- Severity: P2
- Disposition: Incorporated
- Change: Source counting excludes generated expansion under a declared tool
  profile, spike extrapolation must be explicit, confidence is lowered, and
  every proposed issue has an engineer-week planning range.

### C-07: Octocrab capability gaps are deferred

- Severity: P2
- Disposition: Incorporated
- Change: V3-02 inventories every required GitHub operation and reopens the
  dependency decision if more than three require raw requests. Every raw
  endpoint requires typed structures, an API reference, and fixtures.

### C-08: Local-command, PVF, and remote-operation issues are too broad

- Severity: P2
- Disposition: Incorporated
- Change: Local issue/bind work is separated from card/doctor work; PVF planning
  is separated from execution/evidence; remote work is split into three issues.
  The plan now contains 18 implementation issues plus deferred retirement.

## Final Verification

### Verification Round 1

Reviewed revision: `b9187722888b1b1d8e3e09a33f11a4f65e1940d3`

Evidence:

- `.csdlc/evidence/73/provider-reviews/verification-r1-gemini-result.json`
- `.csdlc/evidence/73/provider-reviews/verification-r1-claude-sonnet-result.json`

Gemini returned `REQUEST_CHANGES`. Claude returned a complete finding register,
but its response reached the configured output limit before emitting a terminal
decision, so it is retained as findings evidence rather than accepted final
verification.

The following new findings were incorporated:

- Projections now replace only after canonical `state.json` commits; a
  post-commit projection failure is repair-required, not ambiguous authority.
- Repository and issue context use async lazy cells consistently with their
  Git/process I/O and call sites.
- Root parsing declares `--jq` and `--template`, and the output envelope,
  schema evolution, and in-process formatter boundaries are explicit.
- Cleanup requires committed `closed_out` state and a terminal receipt, not
  GitHub merge observation alone.
- Sync and async lazy accessor, error-caching, cancellation, and retry behavior
  are explicit and test-owned.
- Operator Decision 11 is a hard dependency between the construction spike and
  transaction-store implementation.
- Reviewer principals and the enforceable independence/policy-only boundary are
  explicit.
- Foreground watch defaults to 30 minutes, caps at 24 hours, polls every 15
  seconds by default, and reports progress on stderr.
- V3-02 and V3-03 own preliminary and production dependency-policy checks.
- Per-card/per-phase required and optional field behavior is contract-owned.
- Canonical state embeds typed audit events; `audit.jsonl` is only a generated
  projection, with no initial pruning or co-primary authority.

### Verification Round 2

Reviewed revision: `29aed43ab0dfc2914e5a9dba0877039b12de52ec`

Evidence:

- `.csdlc/evidence/73/provider-reviews/verification-r2-gemini-result.json`
- `.csdlc/evidence/73/provider-reviews/verification-r2-claude-sonnet-result.json`

Gemini returned `PASS`. Claude returned `REQUEST_CHANGES` with two P1 findings,
both incorporated:

- The plan now spells out that `SyncInit<T>` stores the full
  `Result<T, Arc<AppError>>` inside `OnceLock`, and that its accessor
  pattern-matches that stored result without a panic path.
- V3-01 now owns the PVF subprocess command-allowance policy; V3-09 implements
  and enforces that contract rather than inventing it.

The final plan must receive fresh Claude and Gemini reviews over the same exact
revision. Verification is complete only when both return no undispositioned
P0/P1 findings and this record names that revision and evidence.

### Final Result

Reviewed revision: `3d9bb25a01ad704722bae4e383d648a4264c9574`

Reviewed scope:

- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`

Evidence:

- `.csdlc/evidence/73/provider-reviews/final-gemini-result.json`
- `.csdlc/evidence/73/provider-reviews/final-claude-sonnet-result.json`

Gemini 3.1 Pro Preview: `PASS`, no unresolved P0/P1 findings.

Claude Sonnet 4.6: `PASS`, no unresolved P0/P1 findings.

The review scope was unchanged after the reviewed revision. This record and the
provider receipts are evidence-only additions and do not alter the reviewed
architecture or diagram.

## Pre-PR Subagent Review

Reviewed target: `5b4d4cd6d2f40455f6590007535a4551034a0c37`

Evidence: `.csdlc/evidence/73/pre-pr-review.md`

Initial decision: `REQUEST_CHANGES`

Final reviewed target: `64b6360bfb3da05af4af4a149775d894e13cadeb`

Final decision: `PASS`, no remaining P0-P3 findings

Two P1 architecture findings were incorporated: the reviewer-identity adapter
dependency is now acyclic, and intent journals are explicitly separated from
lifecycle/card-state authority. At that review revision, one P2
validation-evidence finding still required exact retained commands and the STP
still carried the initial fourteen-issue wording because the typed v2 editor
then rejected that collection mutation in `implemented`.

Those two statements are preserved as historical review truth and are now
superseded. The later typed lifecycle correction updated the STP to the final
eighteen implementation specifications plus V3-R01 denominator, and the final
VPP/SOR evidence records the executable exact-object source readback and all
other final proof lanes. No card was hand-edited and no lifecycle gate was
bypassed.

The final re-review also confirmed the exact AC-6 proof: repository-local link
existence, all ten pinned upstream source paths, and a retained Mermaid render.

### Post-Pre-PR Exact Verification

Reviewed revision: `17041ed7da93d2b4f9c6978053daedeb3b8c1c27`

Reviewed scope:

- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`

Evidence:

- `.csdlc/evidence/73/provider-reviews/post-pre-pr-final-gemini-result.json`
- `.csdlc/evidence/73/provider-reviews/post-pre-pr-final-claude-sonnet-result.json`

Gemini 3.1 Pro Preview: `PASS`, no unresolved P0/P1 findings.

Claude Sonnet 4.6: `PASS`, no unresolved P0/P1 findings.

Both reviewers confirmed the identity-interface dependency is acyclic, pending
intent journals have explicit authority and precedence boundaries, no earlier
safety correction regressed, and the complete 18-plus-one issue graph remains
ordered.

## PR 77 Architecture Correction Wave

Final reviewed revision: `7c488b9eea47cd642128fb0d0b38618083c2693d`

Final reviewed scope:

- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`

Final evidence:

- `.csdlc/evidence/73/provider-reviews/final-v3-ultimate-claude-result.json`
- `.csdlc/evidence/73/provider-reviews/final-v3-ultimate-r2-gemini-result.json`
- `.csdlc/evidence/73/final-exact-head-review.md`

Claude Sonnet 4.6: `PASS`, no remaining P0-P2 findings.

Gemini 3.1 Pro Preview: `PASS`, no remaining P0-P2 findings.

Independent Codex subagent Planck: `PASS`, no remaining P0-P2 findings.

The correction wave incorporated every actionable review finding, including:

- typed `closing | part_of` linkage through review, publication, readback,
  checkpoint continuation, finish, and tests;
- a concrete `AsyncInit` wrapper and exhaustive state/event contract;
- a closed field/outcome capability matrix with executable recovery and
  operator-disposition rows;
- an explicit `checkpoint_completed` transition from successful `PartOf`
  publication back to executable `implemented` state;
- a portable official `cli/cli` baseline and reproducible v2 state-size and
  worst-case audit-growth contracts;
- normative jq EBNF and negative grammar, fixed Cargo package/binary identity,
  binding V3-02 thresholds, and exact diagram/text dependency parity.

Earlier request-changes and truncated receipts remain retained as review
history. The three evidence additions above are the terminal approval set. Any
later architecture or diagram change invalidates this result and requires fresh
exact-head review.

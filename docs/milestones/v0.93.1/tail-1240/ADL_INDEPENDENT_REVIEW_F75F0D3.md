## Metadata

- Review Type: repo_review
- Subject: public ADL at `f75f0d3f9c4df85743638b956f2c2a80f3424f38`
- Reviewer: OpenAI Codex independent agent `/root/adl_external_reviewer`; sole substantive reviewer for this packet. No implementation, findings-disposition, or #1238 internal-review authorship. This is same-session agent independence, not third-party organizational independence.
- Date: 2026-10-15
- Input Surfaces: `ADL_REVIEW_ENTRYPOINT.md` initially at packet `b282e79796e7016bd0d163bf7b1f4a816f63c0c4`; entrypoint delta inspected through `cf395fea7bcf87bbab9d901df6bf7250d4ec2997`. Product freeze unchanged. Exact product Git blobs exported into a read-only review snapshot; no checkout or shared source edits.
- Output Location: `ADL_INDEPENDENT_REVIEW_F75F0D3.md` with `ADL_REVIEW_COVERAGE.json`, `source-inventory.json`, and isolated reproduction evidence beside it.

## Scope

- Reviewed: public dispatcher and owner selectors; portable workflow parsing/presentation; provider/model declarations; UTS validation; language parsing, validation and canonicalization; compiler graph/lowering and limits; sampled legacy loader/resolution/prompt contracts; session/process operations; five-package manifests, dependency boundaries, CI routing/coverage/installed-consumer construction; current root/onboarding/operator/spec entrypoints; retained #1223 proof and selected split-interface contracts.
- Not Reviewed: exhaustive historical documentation/demo corpus; complete legacy execution-plan/trace implementation; every test or lockfile row; unselected retained engine/records/adapters/CLI compatibility implementations; private product implementations, deployments, credential stores, current archive integrity, or live installed execution. The inventory covers 591 exported paths; it is not a claim that all 591 were reviewed.
- Review Mode: mixed, source-first with focused contract/test and documentation sampling.
- Gate: release-tail, bounded public ADL review only.
- Validation: one std-only Rust probe compiled from the exact prompt assembly function and executed. No repository build, package suite, network, provider call, archive download, archive hashing, or CI dispatch. Existing proof inspected and reused with original identities/limits, not relabeled as fresh execution.

## Findings

1. **[P2] ADL-1240-EXT-01 — Prompt replacement is nondeterministic for nested placeholder text.**

Location: `adl-legacy-contracts/src/prompt.rs:24-27` (public caller at `:80`).

Trigger: prompt `{{a}}`, input `a = {{b}}`, input `b = DONE`. The function walks a randomized `HashMap` and applies replacements to the progressively changed string. Visiting `a` before `b` produces `DONE`; visiting `b` before `a` leaves `{{b}}`.

Impact: identical declared prompts and inputs yield different presented prompt text and downstream prompt hashes. This is a current public behavior, not merely an unused historical path: `public_workflow.rs:38-52` keeps explicit presentation local when no Runtime expansion is required; `workflow_display.rs:13-14` calls `print_prompts`, and `resolve.rs` copies step inputs unchanged. Public library callers can also call `trace_prompt_assembly`. No provider-execution or private Runtime defect is inferred.

Evidence: `prompt-determinism-probe.rs` copies the exact function with an inert field-compatible prompt type; 256 freshly seeded maps with identical values produced both outputs. `prompt-determinism-result.json` records successful probe compilation/execution and two unique outputs. Existing tests use independent literal substitutions and do not cover replacement values containing another token. The source itself labels this operation deterministic; the language draft likewise requires deterministic assembly.

Fix Direction: explicitly define interpolation semantics, preferably substitute tokens in the original template once so values remain literal; alternatively choose and document deterministic recursive semantics. Add a focused regression across independently created maps, cross-token values, and literal braces. Do not merely rely on a stable insertion sequence in `HashMap`.

2. **[P2] ADL-1240-EXT-02 — Compiler resource limits run after unbounded recursive validation.**

Location: `adl-v2/crates/adl-compiler/src/lib.rs:147-151`, with input-limit checks at `:162-175`; recursive callees in `adl-v2/crates/adl-language/src/validate.rs:300-346`.

Trigger: call the public `compile_with_limits` API with a typed document containing a very deep nested step input or a long state-dependency chain, even with a small configured depth/node limit. `adl_language::validate` first visits all declared workflows, recursively walks input values, and recursively traverses dependencies. Only after it returns does the compiler inspect its node/depth/count budgets.

Impact: these budgets do not protect the validation phase: an oversized typed input consumes traversal/allocation work and can exhaust the call stack before returning the advertised `LimitExceeded` diagnostic. Parsing limits cannot protect callers constructing the public model directly. This is a boundedness defect in the public library; no externally exposed service exploit is claimed.

Evidence: static call-order and recursive-traversal trace. Existing `limits_and_diagnostics.rs` exercises small inputs; it does not test oversized/deep inputs reaching validation before limits. No stack-overflow stress test was executed, so no exact failure-depth threshold is claimed.

Fix Direction: perform an iterative bounded structural preflight before recursive semantic validation, or thread explicit budgets through validation and make dependency traversal iterative. Cover deep typed inputs and over-limit dependency chains without crashing the test process.

3. **[P3] ADL-1240-EXT-03 — Large positive session TTLs panic instead of returning a validation error.**

Location: `adl/src/session_ledger.rs:278` and `:315`; CLI accepts the full signed integer range at `adl/src/cli/session_cmd.rs:269-271`.

Trigger: a claim with `--ttl-secs 9223372036854775807`, or the same TTL on an existing live claim heartbeat. The only range check rejects values at or below zero; `Duration::seconds` is then called directly. Lesser large values can also overflow the resulting timestamp addition.

Impact: malformed numeric input escapes the public command's normal error contract and panics. The library heartbeat updates `heartbeat_at` before calculating the expiration, so a caller catching the panic can also retain a partially changed in-memory record. No durable corruption is claimed: the CLI saves only after the method returns.

Evidence: static trace plus the locally available locked dependency `chrono 0.4.44` implementation: `TimeDelta::seconds` calls an expect on `try_seconds`, whose documented bound is `i64::MAX / 1000`. Existing session tests cover zero TTL, not upper bounds. The panic was not dynamically executed.

Fix Direction: use checked duration construction and checked timestamp addition, reject out-of-range values as ordinary errors, and calculate expiration before mutating the claim. Test both claim and heartbeat rejection with unchanged ledger state.

4. **[P3] ADL-1240-EXT-04 — Current specification entrypoint still describes the retired repository layout and milestone.**

Location: `adl-spec/README.md:6`, `:19-24`, and `:31-35`.

Trigger: a reader follows the refreshed root README or onboarding link to the language specification. The page identifies `../adl/` as the Rust runtime and labels v0.87.1 the current active implementation/review milestone.

Impact: this current entrypoint contradicts the selected v0.93.1 split and routes language/runtime understanding back to a retired monorepo layout. The passage is not labeled as a dated historical snapshot. This is narrower than reopening the accepted #1234 root-document review: it concerns the separately linked spec entrypoint.

Evidence: exact frozen source; root README and `docs/REPOSITORIES.md` describe public CLI/contracts here and separate Runtime ownership.

Fix Direction: update present-tense ownership/navigation and either remove milestone status or label the old milestone discussion as historical. Preserve the draft ADL 1.0 normative text and historical evidence rather than declaring it shipped.

## System-Level Assessment

The split is represented coherently in the reviewed executable dependency graph and selector contracts: public planning and local coordination stay local, while execution routes to explicitly selected owners. Selector tests meaningfully cover identity mismatch, aliases, missing receipts, literal arguments and refusal; their synthetic installations are correctly labeled as non-qualification. CI separately requires selected public validation and coverage lanes, and its coverage inventory names the measured owners and declaration exemptions.

The retained #1223 result preserves five public packages, seven installed consumer cases, 21 UTS fixtures and five negatives at `ad8204be76528a2c13ffabf5ee1e2672df475c8b`. Its correspondence records distinguish the original 185-input comparison from 182 unchanged final inputs plus three validated guard changes. Existing coverage is explicitly reused from an earlier measured source, not claimed as a new final-head percentage. Git path comparison from that qualification source to the frozen product found the removed uncompiled `adl/src/cli/run.rs`, UTS CI test correction, and public coverage/package-lane changes; the prompt/compiler/session production findings predate those packaging changes. Prior split/guard review does not cover these newly exposed behavioral cases.

The private split-interface records and exact contracts already verified during preflight support read access only here. I did not re-review the private implementations or turn recovered historical receipts into present installed-byte qualification. Final #1236/#1238/#1241 heads and final Sprint 4 synthesis remain separate.

## Recommended Action Plan

- Fix now: route EXT-01 and EXT-02 to bounded public-contract repairs with focused regression proof and independent exact-head review.
- Fix before milestone closeout: resolve EXT-03 and EXT-04, or obtain explicit evidence-backed disposition rather than silently omitting them.
- Defer: broader private product review, historical-doc cleanup, archive integrity refresh, and new installed qualification unless a changed candidate makes those specifically necessary.

## Follow-ups / Deferred Work

The report is not exhaustive proof of absence of defects. Full parser stress/fuzz testing, concurrency/crash recovery of the session lock, all legacy trace/plan branches, external dependency vulnerability review, and every historical spec/schema/example correspondence were not executed or exhaustively analyzed. No actionable conclusion is invented for those unexamined areas. Owner selectors intentionally rely on the continuing trust of an owner-qualified installation; absence of a second payload hash is not reported as a vulnerability against that declared model.

Existing access receipts remain separate, unchanged historical evidence of what was readable at each preflight. Fresh candidate metadata must be read back before final synthesis. Review independence is maintained: no fixes or disposition records were authored by this reviewer.

## Final Assessment

**CHANGES_REQUIRED for the bounded public ADL source at `f75f0d3f9c4df85743638b956f2c2a80f3424f38`: two P2 and two P3 findings.** The report supplies independent ADL coverage, not full Sprint 4 release acceptance. Reuse unaffected proof, inspect the repair delta and its focused regression results, and reconcile findings before advancing a review-clean claim.

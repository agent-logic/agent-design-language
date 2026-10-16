## Metadata

- Review Type: repo_review
- Subject: repair deltas for ADL-1240-EXT-01 through EXT-04 against `f75f0d3f9c4df85743638b956f2c2a80f3424f38`
- Reviewer: OpenAI Codex independent agent `/root/adl_external_reviewer`, original independent reviewer; no repair authorship or findings-disposition authorship. Same-session agent independence, not organizational third-party independence.
- Date: 2026-10-16
- Input Surfaces: immutable Git repair heads listed below; original `ADL_INDEPENDENT_REVIEW_F75F0D3.md`; retained native proof/review receipts and coordinator validation report.
- Output Location: `ADL_REPAIR_DELTA_REVIEW.md`

## Scope

- Reviewed: exact changed production, test and specification files for the four original findings. Existing public-source coverage is reused; no full repository review repeated.
- Not Reviewed: lifecycle cards, unrelated product implementations, final combined release candidate, current remote CI/PR state, or final #1236/#1238/#1241 acceptance.
- Review Mode: code, tests and docs delta assessment.
- Gate: release-tail repair review, separate from final Sprint 4 synthesis.
- Execution: no new tests, builds, downloads, hashes, providers or network calls. Source inspected through local immutable Git objects; prior validation receipts read without rerunning their commands.

## Findings

**No new material findings in the examined repair deltas.** The four original adverse findings remain preserved in the original report; the assessment below applies only to their named repair heads and does not edit the findings register or declare final release acceptance.

| Original finding | Exact assessed repair | Assessment |
| --- | --- | --- |
| EXT-01, P2 prompt nondeterminism | `ba221958e90a85de74e01600dc33d7efa5d7cfef` | Original trigger corrected. |
| EXT-02, P2 validation before compiler limits | `5e41adf2c63c073df56781ee2b5deba5254fc6d0` | Original trigger corrected within the declared compiler-limit scope. |
| EXT-03, P3 TTL panic and mutation | `1a979c83df1a862a42024a4f80e257a99aae89b3` | Original trigger corrected. |
| EXT-04, P3 stale specification entrypoint | `1a979c83df1a862a42024a4f80e257a99aae89b3` | Original misleading current-status/ownership claims corrected. |

**EXT-01:** `adl-legacy-contracts/src/prompt.rs` scans the original template left to right and looks up each complete placeholder once. Replacement text is appended literally and never scanned, eliminating randomized map-order and cascading substitutions. Unknown and unclosed placeholders are retained; Unicode and empty values have regression coverage. The cross-token regression is effective regardless of map iteration order, so its success does not depend on probabilistically encountering a particular order. The current public presentation caller remains unchanged. Retained native proof is bound to this exact head and reports nine passed tests. Coordinator reports eight green CI checks. The retained native merge receipt maps this head to merge `15485055270e7c1376638010e520b29734abb8ef` and closed issue state. That merge object was absent from the checked local object database, so merged-tree equivalence was not independently recomputed; the assessment is of the exact reviewed head plus retained merge evidence.

**EXT-02:** `adl-v2/crates/adl-compiler/src/lib.rs` calls `check_document_limits` before language validation. It checks run inputs, selected and unselected workflow inputs, and per-workflow node counts. The value walk is iterative and checks pending child fanout before allocating it, so an extremely wide array does not create an unbounded pending stack before the configured value limit is enforced. `adl-language/src/validate.rs` now uses an explicit DFS work stack for dependency cycles; active versus completed nodes remain distinct, preserving cycle detection while removing graph-depth call-stack recursion. Tests cover deep typed input with an invalid state reference (so preflight ordering is observable), wide fanout, unselected workflow inputs, an over-limit chain, and an in-limit 2,048-node chain. Retained native proof at the exact head reports ten passing compiler tests; coordinator separately reports nine language tests passing. No new execution was performed here. Limits are caller-configurable; this does not claim bounded direct standalone language validation or every arbitrary declaration field, nor does it convert local proof into the pending CI result.

**EXT-03:** `adl/src/session_ledger.rs` computes expiration through `Duration::try_seconds` and `DateTime::checked_add_signed` before acquiring a mutable claim reference or changing ledger state. Both duration overflow and timestamp overflow become ordinary errors. Regression tests assert complete ledger equality for both failure classes; the heartbeat uses an advanced observation time, so the original premature `heartbeat_at` mutation would be detected. `test_public_session.py` checks ordinary command failure and unchanged persisted bytes. Retained native proof at the exact head reports fourteen passing Rust tests. The retained root review judgment additionally records thirty successful CLI proof commands and the two strengthened overflow regressions passing after the test-only correction. This review reuses that evidence rather than executing it again.

**EXT-04:** `adl-spec/README.md` now distinguishes public CLI/contracts from the separate Runtime repository, removes obsolete current milestone status, explicitly labels prior milestones historical, and corrects related navigation. The ADL 1.0 draft status and normative specification text are preserved; the repair does not invent a new shipped-language or release claim.

## System-Level Assessment

The changes are confined to the original findings' behavior and regression surfaces. Product/test deltas comprise one prompt file, three compiler/language files, the session implementation and CLI test, and the specification README. Additional changed files are issue lifecycle records, which are outside this source-delta assessment. No manifest or dependency change is needed by these repairs.

All four findings have a source-grounded satisfactory repair at their individual assessed heads. This is not yet a statement about a single integrated release commit. The original frozen-source CHANGES_REQUIRED report remains true of `f75f0d3...`; its findings must be reconciled against the final composed candidate by the owning register and synthesis lanes.

## Recommended Action Plan

- Fix now: none newly identified in these repair deltas.
- Fix before milestone closeout: finish required CI/publication/merge gates, select the combined immutable candidate, and verify it retains these exact repairs before final synthesis.
- Defer: unchanged broader coverage and proof remain as limited in the original report; no new blanket build/review campaign is indicated.

## Follow-ups / Deferred Work

The compiler CI was pending and TTL/spec publication in progress at assignment. Those states were not polled or upgraded by this review. The final #1236 inventory, #1238 internal assessment and #1241 dispositions still require exact-head consumption. The original adverse report, coverage ledger and reproduction remain unmodified.

## Final Assessment

**PASS for the bounded repair deltas at the three exact heads above.** No new material findings. This supports independent repair review for EXT-01 through EXT-04; it is not a final findings-register disposition, integrated-candidate approval, or Sprint 4 release acceptance.

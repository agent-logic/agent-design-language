# Methods, model specification, and limitations

Promotion note: this is the retained statistical study, not a new census or independent recomputation. Detailed study datasets and logs remain local; their hashes are recorded in `../source-promotion-manifest.json`. The tracked plan is self-contained for implementation requirements; reproducing the historical statistics requires the retained study data.

## Study design

This is a retrospective observational study of the accessible ADL issue population, with a census of GitHub records and retained local Codex logs. It is not a randomized comparison, an acceptance audit of every implementation, or a reconstruction of every issue that ever existed. Its strongest evidence is the combination of a broad denominator, explicit operational failure examples, and repeatable extraction.

Units of analysis are deliberately separate:

- **Issue record:** `(repository, issue number)`, with GitHub node identity available in the raw census. Used for creation cohorts, closure, and linked-merge intervals.
- **Pull request:** `(repository, PR number)`. Used for merge times and coarse code-change covariates. Several issues can close through one PR.
- **Command bundle:** one original Codex tool-call identity containing one or more recognized lifecycle shell invocations. Used for observable outcome and repeat analyses.
- **Retained issue audit:** one issue index/audit pair in the inspected main checkout. Used for operation counts and recovery recurrence.

These units must not be divided into each other to create an unsupported “commands per delivered issue” or “cost per bug.” The logs do not provide a reliable one-to-one issue assignment across all long-running tasks, changing worktrees, and repository migration.

## Collection and completeness

`collect_github.py` enumerates `GET /repos/{owner}/agent-design-language/issues?state=all`, with 100 records per page until a short final page, for both repositories. PR objects are retained and separately identified. It also enumerates repository issue comments. The collection manifests retain page counts, SHA-256 hashes, and collection timestamps.

`collect_detail.py` enumerates every issue timeline, including additional pages where needed. It separately enumerates PRs through GraphQL, retaining timestamps, state, draft/base/head metadata, changed-file/line counts, commit counts, review summaries, and declared closing-issue relationships. All requested review and closing-reference subcollections were within their requested limits; no such connection was truncated. The population is defined by accessible repository records, not by assuming contiguous issue numbers.

There is no atomic cross-endpoint snapshot. The initial issue census, timelines, PR details, and local log extraction each have their own observation time. Activity during collection can produce small discrepancies. Reported issue state and censoring use the initial census; detailed events and PR metadata come from the subsequent collection. No claim depends on sub-minute ordering across these requests.

GitHub comments and titles are mutable. Timelines preserve some changes, but this study does not reconstruct every historical body version. Closed issues without a verified PR closing relationship may have direct commits, unrecognized linkage, or administrative closure. They are not automatically classified as undelivered or incorrectly closed.

The 22 explicit migration routes are extracted from legacy issue comments. Direct successor declarations take precedence over incidental references. Only a single existing canonical issue target is accepted. Source records remain in the census and are marked `closed_migrated`; successor records remain separate. Other duplicate/decomposition relationships are not assumed to be one-to-one work items.

## Period definitions

| Period | UTC boundary | Interpretation |
| --- | --- | --- |
| `pre_formal_csdlc` | Before 2026-05-14 22:35:55 | Before the first explicit C-SDLC title in the issue census, legacy #3063 |
| `csdlc_v1_period` | Until 2026-07-13 02:15:02 | Before merged Gate 10C PR #5304 |
| `typed_v2_period` | Until 2026-09-07 03:34:56 | Before canonical cutover PR #591 merged |
| `post_v3_cutover` | Thereafter | Very short follow-up, unsuitable for claims of v3 effectiveness |

The first boundary is a reproducible historical marker, not proof of the day all C-SDLC behavior began. Wrappers and cards existed earlier. Issue cohorts use issue creation time; command cohorts use tool-call time. An issue can cross several periods. Neither variable establishes exact installed binary provenance for every operation.

## Issue screening and linkage

The exact regexes are versioned in `scripts/analyze.py`.

**Control-plane title signal** uses explicit C-SDLC, card/prompt lifecycle, named card, PR-wrapper, or lifecycle-tooling terms in the title, or `area:csdlc`. Body boilerplate is not used to classify every issue as C-SDLC. **Corrective title signal** includes repair, fix, recovery, regression, defect, stale/drift, hardening, and reconciliation. This includes intended improvements and administrative repair, not only bugs. **Bug labels** are kept independently.

Mechanism themes are overlapping title-based screens. They prioritize records for inspection, not mutually exclusive root-cause verdicts. A new defect can be missed by the vocabulary; a product issue with similar vocabulary can be included. The CSV retains the original title, labels, and exact classification so readers can challenge individual rows. The study does not claim a statistically validated precision/recall for title screening.

PR linkage uses GitHub `closingIssuesReferences` and explicit closing directives at the start of unfenced PR-body lines, with fully qualified cross-repository targets where present. Arbitrary mentions and cross-referenced timeline events are not enough. A candidate merge predating issue creation is excluded from duration estimates. An explicitly migrated source is not assigned a delivered endpoint merely because it has a PR reference.

## Codex extraction

The scanner reads every retained rollout file under `sessions/` and `archived_sessions/`. It uses `session_meta` for repository/cwd identity and `response_item` for original calls/results. It skips `event_msg` duplicates, prose messages, reasoning, and summaries when counting attempts. It reads a bounded prefix corresponding to the file size observed at scan time; active sessions can subsequently grow.

Recognized shell commands include `pr.sh`, `codex_pr.sh`, and `csdlc` owner binaries in direct tool arguments or JavaScript orchestration `cmd` fields. The parser is conservative and is not a full shell or JavaScript interpreter. Calls buried in unsupported dynamic constructions, subsequent async waits, indirect helper scripts, and some `cargo run` variants can be missed or have unknown outcomes. This is why the extraction is described as recognized command bundles, not every subprocess executed.

Duplicate `call_id` records inherited through forks are counted once. Independent repeated calls with different identities are retained. The present Planning #6 task is excluded from the historical bundle population; its observed CLI mismatch is a separate current case.

Results are classified using current top-level result envelopes, explicit wrapper exit codes, and anchored runtime error lines. The parser does **not** recurse into old audit events, plan prose, or quoted status fields. Only outer output envelopes are unwrapped. A returned issue index with a historical `status: blocked` does not make a successful edit a new failure.

Outcome meanings:

| Outcome | Meaning |
| --- | --- |
| `nonzero_exit` | At least one recognized explicit nonzero wrapper exit in the result |
| `reported_blocker_or_error` | Current reported block/corruption/error or anchored error line, without an already classified nonzero exit |
| `zero_exit` | Recognized zero exit without a recognized current adverse signal |
| `reported_success` | Current explicit success/pass without a recognized adverse signal |
| `unknown_result` | A result exists, but the conservative rules do not establish the outcome |
| `unobserved` | No matched result in the scanned trace |

An adverse result is the union of the first two rows. It includes expected denials and pending external work. A zero exit is not proof of acceptance, correctness, or actual state advancement.

Help discovery is identified separately. In particular, `pr.sh --version v0.91.x` selects a milestone and is not treated as a version/help query. Multiple-command contexts are flagged; rates on the single-context, no-help subset are reported as a sensitivity check.

The development audit inspected 75 deterministic stratified samples, including 45 initially flagged adverse results. It found a historical-audit false positive and category leakage from generic JSON words. The parser was corrected and the data reprocessed. This was a **development audit**, not a blinded independent validation sample; do not report its agreement as an out-of-sample precision estimate. The sample identities/offsets and validation assertions are retained. Recall against all nonselected tool calls is not measured.

Raw Codex commands, prompts, credentials, and tool outputs are not copied into the report or analytic CSVs. Source pointers use rollout-relative paths, call IDs, byte offsets, and hashes. They permit authorized local reinspection without treating private transcripts as publishable report content.

## Command models

For bundles with an observable result, define `Y = 1` for an adverse observation and `0` for the other observable outcomes. The conditional descriptive rate is `sum(Y) / N_observable`. Unknown outcomes remain visible in the denominator table and are not imputed as successes.

Headline uncertainty uses 2,000 bootstrap resamples of entire observed sessions, seed 60908, and percentile intervals. This preserves within-session concentration better than a binomial interval. Different tasks can still share an issue or inherited context, so session clustering does not resolve every dependency. Wilson intervals retained in intermediate tables are simple descriptive intervals; the report prefers session-cluster intervals.

The exploratory logistic model is:

```text
logit P(adverse | observable) = intercept
    + command-stage indicators
    + historical-period indicators
    + indicator(multiple recognized commands)
```

It uses IRLS and a session-cluster robust sandwich covariance. The reference is doctor in the early period with one matched command. Stages with fewer than 250 observable bundles and mixed-stage bundles are excluded from this fit. Coefficients are odds ratios conditional on being observed, not causal effects. Stage and period are associated with different tasks, logging conventions, and legitimate gate outcomes. The in-sample Brier score is a fit diagnostic, not a validation of a deployment predictor.

Sensitivity results exclude recognized CI-pending observations, restrict shell context/help, and use explicit exits alone. Extreme bounds also assign every unknown outcome first to non-adverse and then to adverse. The wide bounds demonstrate that an unconditional historical error rate cannot be identified from these logs.

Repeat bursts group an exact command-text hash within the same task. Consecutive calls must be no more than 30 minutes apart; only groups of at least two calls whose first result is adverse are included. A burst can contain state changes between identical invocations, useful polling, or unknown outcomes. It is not a proven retry loop with unchanged state. No labor/cost total is inferred from the span.

## Issue-time models

The linked duration is elapsed issue creation to earliest linked merged PR. The interval table additionally requires the PR to have opened after issue creation; seven linked observations do not meet that interval requirement. Multiple PRs and incremental deliveries mean this is an observed linkage endpoint, not necessarily final acceptance of all scope.

The exploratory duration regression is:

```text
log(1 + linked-merge hours) = intercept
    + control-plane title signal
    + corrective title signal
    + bug label
    + umbrella/sprint title signal
    + log(1 + PR additions + deletions)
    + historical-period indicators + residual
```

The post-v3 cohort is excluded. Standard errors cluster by linked PR. Exponentiated coefficients are ratios on `1 + hours`, not ordinary duration ratios. Changed lines are measured after execution and may themselves depend on process; this is an explanatory association model, not a causal adjustment set. Unmeasured complexity, dependencies, staffing, cloud/provider waits, and work prioritization remain large confounders. Low R² limits predictive use.

For 14-day outcomes, the competing-risk estimator has event 1 = linked merge occurring before another closure, event 2 = other closure, and censoring = still open at the initial census or the 14-day horizon. With risk set `n(t)` and event counts `d1(t), d2(t)`:

```text
S(t) = product over u<=t of [1 - (d1(u)+d2(u))/n(u)]
F1(t) = sum over u<=t of S(u-) * d1(u)/n(u)
F2(t) = sum over u<=t of S(u-) * d2(u)/n(u)
```

This avoids counting non-delivery closure as success or treating it as an ordinary independent censoring event. The latter is especially important for migration and retirement. Cohorts with insufficient follow-up are not extrapolated to day 14. The additional restricted mean in the JSON refers to time before **any observed endpoint**, not mean time to delivered acceptance.

## What a real state-machine model requires

The retained audits support operation frequencies and inferred phase-target sequences, not state holding times or rejection probabilities. The 524 retained records are not the complete historical issue population. Their audits predominantly describe recorded successful operations; denied attempts and waiting are elsewhere. `retained_inferred_transitions.csv` is therefore a descriptive reconstruction, not an identified transition matrix for the actual application. The inspected audit definitions are [v2 model.rs](https://github.com/agent-logic/agent-design-language/blob/bf159eb416950dfa3399933829726a7b7e71f897/csdlc-v2/src/model.rs), line 182, and [v3 storage/mod.rs](https://github.com/agent-logic/agent-design-language/blob/bf159eb416950dfa3399933829726a7b7e71f897/csdlc-v3/src/storage/mod.rs), line 45; neither defines event time. The operational diagnostic/recovery ordering is visible in [v3 commands/local/mod.rs](https://github.com/agent-logic/agent-design-language/blob/bf159eb416950dfa3399933829726a7b7e71f897/csdlc-v3/src/commands/local/mod.rs), lines 1279–1312, and is treated as static evidence rather than an executed mutation test.

The prospective model should be a **marked semi-Markov process**: lifecycle state plus versioned relevant facts, with a distribution of time spent in each state and an explicit outcome for each attempted action. A phase-only memoryless Markov chain would hide stale evidence, pending external effects, and dependency identity—the very facts that govern admissibility.

For every attempt, record an immutable event containing:

```text
repository + issue; event_id; intent_id; attempt_id; parent_attempt_id
command/schema version; installed binary provenance; actor/session
state/version before; requested action; actual state/version after
outcome class; machine reason code; expected vs unexpected denial
dependency/remote-operation IDs; relevant evidence identities
wall timestamp; monotonic duration; clock/source identity
wait class; responsible owner; next admissible action
effects attempted/performed/reconciled; idempotency identity
```

Inject clock observations and keep telemetry separate from semantic authorization decisions so timing does not introduce nondeterminism into the pure kernel. Failed attempts and expected waits need events too. The command envelope and committed transition event must agree about effects. Correlation IDs should link a remote intent, remote observation, local commit, and later recovery.

Once this exists, estimate transition frequencies conditional on relevant state, dwell-time distributions, first-pass yield, expected visits to repair states, and transition-specific unexpected-failure rates. Those parameters are **not fitted by pretending the current untimed audits contain the missing data**.

## Reproduction

The supplied result files are the frozen analysis surface. Recomputing models from them needs NumPy; plotting additionally needs Matplotlib. The study used the bundled Python runtime, with Matplotlib installed only into this study’s ignored `.libraries/` directory. No repository runtime dependency was changed.

From the study directory:

```sh
python3 scripts/migration_routes.py
python3 scripts/analyze.py
python3 scripts/extended.py
python3 scripts/uncertainty.py
python3 scripts/catalogue.py
python3 scripts/figures.py
python3 scripts/validate_study.py
```

Use a Python environment with NumPy for the analysis steps. `figures.py` discovers the study-local plotting libraries. Collection is separate: `collect_github.py` and `collect_detail.py` perform read-only authenticated `gh api` calls and resume cached pages. `scan_sessions.py` reads local retained rollouts; `refine_sessions.py` re-evaluates extracted offsets with the current parser. Recollection is a new observational snapshot, not an exact reproduction of mutable GitHub or growing logs.

`SOURCE_MANIFEST.json` records source and script hashes, population checks, and current-source observations. `results/validation.json` records the checks actually performed. Superseded development scans are retained under explicit names and are not used by `analyze.py`.

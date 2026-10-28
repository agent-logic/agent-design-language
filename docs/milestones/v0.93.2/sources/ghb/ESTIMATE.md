# GHB time and token estimate

Date: 2026-10-10. Estimate class: initial engineering estimate with substantial integration/research uncertainty. Values are planning assumptions, not measured historical throughput, statistical P50/P90 forecasts, price quotes, or spending authorization.

## Units and assumptions

- One engineering day = eight focused hours of implementation, validation, review response and documentation effort. Agent automation may compress elapsed time but does not remove dependency, review, experiment or observation time.
- Base staffing = three implementation lanes plus shared independent review and an available program owner. Review effort is included in the work-package numbers; reviewer availability is a schedule assumption.
- Token numbers below mean aggregate model input plus output tokens, including repeated context and review invocations. They are not output tokens, unique source tokens, account credits, or a guarantee of billable cost. Record cached input, uncached input, output and provider-reported reasoning separately when available; do not double-count reasoning already included in output.
- Development tokens cover research follow-up, design, coding, tests, debugging, review and docs. Runtime experiment tokens are additional and computed separately.
- Existing identity, continuity, installed contracts and provider execution are reused. No foundation-model training, new formal theorem prover, new cloud fleet, arbitrary-domain benchmark creation or public paper submission is included.
- The full fleet is not inventoried yet. WP11 includes conformance tooling and one initial rollout plan, not unlimited per-resident remediation.

## Runtime scope included in the estimate

The delivery target is the installed Agent Logic Runtime. WP02 includes installed contract consumption; WP05 includes Runtime-controlled experiments; WP07 covers authentic resident authorities; WP08 covers successor loading and recovery; WP09 includes release packaging, configuration, state migration, scheduler integration and clean-install acceptance; WP11 includes installed-service restart and rollback qualification. These are mandatory parts of the existing estimate, not optional follow-up work.

The numerical estimate remains provisional and unchanged because live Runtime delivery was already included in those packages. WP00 must re-estimate if the actual supported deployment lacks the assumed provider, sandbox, persistence or body-loading mechanisms. A replacement runtime, new fleet infrastructure, or remediation of additional deployment platforms is not hidden inside this allowance. Bring the installed loading/invocation smoke slice forward to weeks 2–3 to expose this risk before extensive algorithm work.

## Bottom-up development estimate

Ranges include normal local iteration and bounded review fixes. The base column is a working allocation within each range, not a derived probability quantile. M = one million tokens.

| WP | Work | Engineering days low/base/high | Development tokens M low/base/high |
| --- | --- | --- | --- |
| 00 | Audit, owners and baseline | 2 / 3 / 4 | 0.15 / 0.25 / 0.40 |
| 01 | Normative spec and invariants | 2 / 3 / 4 | 0.15 / 0.25 / 0.40 |
| 02 | Schemas and installed parity | 3 / 4 / 6 | 0.30 / 0.45 / 0.70 |
| 03 | Bayesian engine and experiment choice | 5 / 7 / 10 | 0.50 / 0.80 / 1.20 |
| 04 | Proposers and archive | 4 / 6 / 8 | 0.40 / 0.65 / 1.00 |
| 05 | Isolated builder/executor | 5 / 7 / 10 | 0.50 / 0.80 / 1.20 |
| 06 | Evaluator and promotion | 5 / 8 / 12 | 0.50 / 0.90 / 1.40 |
| 07 | Resident birthday integration | 3 / 5 / 8 | 0.30 / 0.55 / 0.90 |
| 08 | Atomic activation and rollback | 5 / 7 / 10 | 0.50 / 0.80 / 1.20 |
| 09 | Live loop and Gate A | 4 / 6 / 9 | 0.40 / 0.65 / 1.00 |
| 10 | Recursive improvement | 5 / 8 / 12 | 0.50 / 0.90 / 1.50 |
| 11 | Qualification and fleet plan | 6 / 10 / 15 | 0.60 / 1.10 / 1.80 |
| 12 | Research packet and runbooks | 2 / 3 / 5 | 0.20 / 0.35 / 0.60 |
| Total | Before integration contingency | 51 / 77 / 113 | 5.00 / 8.45 / 13.30 |

Add a 25% program contingency to the base for cross-owner integration, schema changes and unexpected failure recovery: **approximately 96 engineering days and 10.6M development tokens**. Do not add this contingency a second time to risk-case numbers.

Gate A alone covers WP00–WP09: **38–81 engineering days**, base 56; **3.7–9.4M development tokens**, base 6.1M, before contingency. With 25% base contingency: **70 engineering days and 7.6M development tokens**. WP12 starts early but its final evidence work is reserved for Gate B.

## Elapsed schedule

| Delivery | Three lanes plus review | One implementation lane |
| --- | --- | --- |
| Gate A | 5–8 calendar weeks | Roughly 12–16 working weeks at base effort plus waits |
| Full Gate B | Base 10–12 calendar weeks; favorable 8; risk 16–20 | Roughly 20–26 working weeks at base effort plus waits |

The arithmetic lower bound for the base contingency allocation is 96 / 3 / 5 = 6.4 working weeks at perfect utilization. The actual 10–12 week plan allows for dependencies, shared-owner serialization, review/CI waits, live evaluation and at least a two-week longitudinal qualification window. More agents cannot proportionally shorten those constraints.

These are staffed implementation forecasts. They are not a promise that an autonomous model run will consume exactly that many wall-clock hours. Calendar dates should be set after WP00 identifies owners and availability.

## Runtime experiment token model

Use this explicit model rather than hiding experimental inference inside coding cost:

`experiment_tokens = variants × tasks_per_variant × trials_per_task × mean_tokens_per_trial + proposal_and_judge_tokens`

Variants include matched baselines. Mean tokens include the full multi-step trial, repeated context, tools summarized into model input and outputs. Deterministic tests do not consume model tokens unless an agent is actively supervising them. Reused cached baseline measurements must be compatible with the frozen environment and protocol.

| Campaign | Variants including baseline | Tasks | Trials | Mean tokens/trial | Trial tokens | Proposals/judges | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Gate A pilot | 5 | 20 | 2 | 8,000 | 1.6M | 0.4M | 2.0M |
| Main candidate selection | 13 | 40 | 3 | 10,000 | 15.6M | 2.4M | 18.0M |
| Final held-out/transfer | 3 | 60 | 3 | 10,000 | 5.4M | 0.6M | 6.0M |
| Bounded recursive-cycle evaluation | 5 | 40 | 3 | 10,000 | 6.0M | 1.0M | 7.0M |
| Total illustrative base campaign | | | | | 28.6M | 4.4M | **33.0M** |

This is an initial campaign design, not a statistical power calculation. Twenty or sixty tasks may be insufficient for the desired effect size or rare failure rate. WP00/WP06 must determine the minimum meaningful improvement, variance, acceptable error rate and resulting sample size. The final holdout is used once for release assessment; do not repeatedly tune against it.

For sequential execution, the base table represents 2,900 trial runs. At an assumed mean five minutes each, that is approximately 242 worker-hours; ten independent workers give a theoretical 24.2-hour lower bound before rate limits, shared builders, long-tail tasks, retries, and gated campaign sequencing. Actual latency must be measured in the pilot. Longitudinal qualification cannot be accelerated by increasing trial workers.

## Combined planning envelopes

| Scope/scenario | Development | Runtime experiments | Combined tokens | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Gate A base with contingency | 7.6M | 2.0M | **9.6M** | One useful persistent improvement demonstration, not full qualification |
| Full favorable case | 5.0M | 12.0M | **17.0M** | Strong reuse, cheap tasks, few revisions; lower confidence/coverage if trials shrink |
| Full base plan | 10.6M | 33.0M | **43.6M** | Recommended initial planning envelope; staged releases of budget |
| Full risk case | 16.6M | 80.0M | **96.6M** | High development range plus 25%; longer tasks and additional campaigns |

The favorable case cannot reduce mandatory coverage or statistical criteria just to stay cheap. If the criteria need more samples, the estimate must increase. Large software tasks averaging 50,000 tokens per trial multiply the trial portion by five. A million-token agent context is not assumed here.

No dollar estimate is given because provider/model mix, input/output ratio, cached-input accounting, infrastructure prices and account limits are not selected. Calculate cost from measured categories and current provider prices at scheduling time. Token counts across providers are approximate workload units and may not be directly comparable.

## Budget controls and recalibration

1. Release a first discovery allocation of **0.5M development tokens and five engineering days** for WP00 and initial WP01. Stop to reforecast if owner duplication, missing production dependencies or a larger domain emerges.
2. After the first two implementation issues, measure tokens per reviewed change, input/output/cache ratio, build/CI waits, review fixes and actual engineering effort. Update remaining work-package ranges.
3. Run a capped pilot before the main campaign. Measure tokens/trial, latency, variance, failure classes and evaluator leakage. Update the power/sample-size calculation and budget.
4. Enforce per-candidate and campaign token/cost ceilings. An incomplete trial produces an incomplete result; it does not silently retry without accounting.
5. Report weekly planned/actual/remaining tokens, completed acceptance gates, blocked dependencies and schedule forecast. Keep developer-model usage separate from runtime-model usage.
6. Re-estimate immediately if any package exceeds its base by 50%, trial usage exceeds its mean assumption by 2×, or the required task sample doubles.

## Main estimate risks

| Risk | Consequence | Response |
| --- | --- | --- |
| Existing owners have deeper production gaps | Integration and schedule grow | WP00 caller audit; reuse current issue owners |
| No candidate improves held-out performance | More experiments or narrower domain | Preserve negative result; revise hypothesis, not evaluator threshold |
| Overfitting or metric exploitation | Prior selection evidence becomes invalid | Independent metrics and fresh held-out release campaign |
| Schema/body migration conflicts | Activation and rollback need more work | Compatibility fixture before broad implementation |
| Runtime #112 remains unresolved | Gate A cannot claim authentic resident continuity | Continue isolated algorithm work; keep final gate explicit |
| Provider/rate-limit/account changes | Latency, cost and reproducibility change | Pin configuration and re-baseline changed providers |
| Fleet heterogeneity | Per-agent adoption exceeds initial allowance | Inventory at WP00, qualify profiles, estimate remediation separately |

The budget buys implementation and a bounded evidence campaign. It cannot guarantee discovery of a statistically significant improvement or indefinitely sustained recursive gains.

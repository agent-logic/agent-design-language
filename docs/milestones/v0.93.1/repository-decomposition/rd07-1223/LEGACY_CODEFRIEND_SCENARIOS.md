# Legacy CodeFriend scenario correspondence

Snapshots: ADL `9cfd035aac2a44637b3c8026bd40b669579a8c7f`; CodeFriend `273089b0ecbe20e1979654e0faeaca0604398de1`.

Read-only inventory: **37 files,338 named Rust tests,0 exact test-name matches** across139 tracked successor Rust files. This is a naming/correspondence result, not338 failed or missing behaviors. No builds were run. Generated macro tests, Python scenarios and external installed artifacts are outside this name denominator.

Every legacy file now has current product feature-source candidates with concrete symbol lines in the JSON. The feature modules still exist; no explicit retirement of these scenarios was found in the bounded README/docs search. That does not prove no retirement decision exists in issue history.

## Concrete acceptance surfaces to compare

- Fitness/structure: `docs/codefriend/INSTALLED_CONSUMERS_SOURCE.json:5-40` maps two old Python consumers and records original cases preserved. `crates/codefriend/tests/installed_consumers.rs:9-17` invokes the installed runner and requires fitness=3,structure=9,partial=3. This is useful case-level evidence for `codefriend_language_fitness*` and `codefriend_structure_v2`; it does not establish all legacy Rust negative assertions.
- Review resource bundle: `crates/codefriend/tests/review_regression_bundle.rs:8-16` checks7resources/4skill contracts/2fixture pairs/6manifest entries. It must not be presented as replacement coverage for the16 legacy review and20 assessment tests without assertion correspondence.
- Agent/journey: standalone `crates/codefriend/tests/support/codefriend_owned_journey.rs` and sibling support files are plausible changed scenario owners. The JSON identifies retained agent/publication/journey implementation symbols; installed receipt/cancellation/replay and publication negatives need comparison against those owned cases.
- Source dependency boundary: `crates/codefriend/Cargo.toml:10-11` uses authenticated `agent-product-support` with providers/memory features instead of monolithic ADL. Legacy tests importing ADL provider/memory types cannot simply be copied into the product without translating this boundary.

## Per-file inventory

| Legacy file | Named tests | Exact name matches | Feature-owner candidates |
|---|---:|---:|---|
| codefriend_agent_publication.rs | 8 | 0 | `agent/publication.rs` |
| codefriend_agent_receipt.rs | 9 | 0 | `agent.rs`, `agent/journey/verification.rs` |
| codefriend_baseline_repeat.rs | 1 | 0 | `memory/baseline.rs` |
| codefriend_cf_cog.rs | 18 | 0 | `architecture/artifact.rs` |
| codefriend_cf_cog_drift.rs | 10 | 0 | `architecture/drift.rs` |
| codefriend_cf_cog_impact.rs | 10 | 0 | `architecture/impact.rs` |
| codefriend_cf_cog_rationale.rs | 10 | 0 | `architecture/rationale.rs` |
| codefriend_cf_gov.rs | 11 | 0 | `governance/local.rs` |
| codefriend_cf_gov_ci.rs | 7 | 0 | `governance/ci.rs` |
| codefriend_cf_memory.rs | 12 | 0 | `memory/mod.rs` |
| codefriend_ci_ingestion.rs | 1 | 0 | `ingestion/ci.rs` |
| codefriend_drift_v2.rs | 6 | 0 | `architecture/drift_v2.rs` |
| codefriend_evidence.rs | 11 | 0 | `evidence/mod.rs`, `evidence/store.rs` |
| codefriend_four_plus_one.rs | 16 | 0 | `architecture/four_plus_one.rs` |
| codefriend_github_ingestion.rs | 8 | 0 | `ingestion/github.rs` |
| codefriend_impact_v2.rs | 4 | 0 | `architecture/impact_v2.rs` |
| codefriend_ingestion.rs | 11 | 0 | `ingestion/local.rs` |
| codefriend_integration.rs | 7 | 0 | `integration.rs` |
| codefriend_journey.rs | 21 | 0 | `integration/journey.rs` |
| codefriend_journey_v2.rs | 4 | 0 | `agent/journey.rs` |
| codefriend_language.rs | 7 | 0 | `language/mod.rs` |
| codefriend_language_fitness.rs | 11 | 0 | `governance/language.rs` |
| codefriend_language_fitness_ci.rs | 3 | 0 | `governance/ci_v2.rs` |
| codefriend_language_parser.rs | 11 | 0 | `language/parser.rs` |
| codefriend_plat_memory.rs | 10 | 0 | `memory/palace.rs` |
| codefriend_rationale_v2.rs | 6 | 0 | `architecture/rationale_v2.rs` |
| codefriend_remediate.rs | 9 | 0 | `actions/remediation.rs` |
| codefriend_render_html.rs | 6 | 0 | `publication/html.rs` |
| codefriend_render_md.rs | 5 | 0 | `publication/markdown.rs` |
| codefriend_render_pdf.rs | 4 | 0 | `publication/pdf.rs` |
| codefriend_review.rs | 16 | 0 | `review/runner.rs` |
| codefriend_review_assessments.rs | 20 | 0 | `evidence/assessments.rs` |
| codefriend_structure_v2.rs | 8 | 0 | `architecture/structure_v2.rs` |
| codefriend_synthesis.rs | 6 | 0 | `review/synthesis.rs` |
| codefriend_testplan.rs | 12 | 0 | `actions/test_plan.rs` |
| codefriend_update_cycle.rs | 12 | 0 | `cycle_bridge.rs` |
| codefriend_ux.rs | 7 | 0 | `operator/mod.rs` |

Next: compare scenario assertions by feature group; mark matching current proof with exact artifact identity, adapt only required uncovered tests, or cite an explicit behavior-retirement decision. No file is approved for deletion by this inventory.

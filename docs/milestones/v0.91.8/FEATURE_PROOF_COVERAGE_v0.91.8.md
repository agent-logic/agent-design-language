# v0.91.8 Feature And Proof Coverage

## Status

Planned mapping; evidence columns remain empty until execution.

| Feature | Primary doc | WPs | Required proof |
|---|---|---|---|
| Baseline and architecture | `features/BASELINE_AND_ARCHITECTURE_v0.91.8.md` | WP-02 | hashed denominator, owner closure, review |
| Characterization and parity | `features/CHARACTERIZATION_AND_PARITY_v0.91.8.md` | WP-03, WP-11 | corpus, normalizer, mismatch dispositions |
| Language and compiler | `features/LANGUAGE_AND_COMPILER_v0.91.8.md` | WP-04, WP-05 | schema, fixtures, deterministic plans |
| Portable engine and contracts | `features/PORTABLE_ENGINE_AND_CONTRACTS_v0.91.8.md` | WP-06, WP-07 | execution, failure, resume, trace and trust tests |
| CLI and adapters | `features/CLI_AND_ADAPTERS_v0.91.8.md` | WP-08 to WP-10 | integration matrix, dependency and binary budgets |
| Cutover and deletion | `features/SHADOW_PARITY_AND_CUTOVER_v0.91.8.md` | WP-11 to WP-13 | soak, rollback, selector, deletion manifest |
| Platform acceptance and deployment | `features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md` | WP-14 | stable installs, deployment, operations, recovery, lifecycle, Unity and Adaptive Learning dispositions |

## Coverage Rule

Every first-class claim needs a primary feature doc, WP owner, executable proof,
review disposition, and release-gate mapping. Missing coverage blocks WP-16.

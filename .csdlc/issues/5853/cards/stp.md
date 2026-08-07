# Structured Task Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Measure the already-approved 16-core GitHub-hosted runner with one cold baseline, three warm baselines, and three test-only canaries; adopt it for adl-rust-tests only when the frozen performance, reliability, cost, security, and exact-head production-canary gates pass.

## Deliverables

- Migration, budget, selected-repository access, bounded maximum concurrency ten, security, and rollback eligibility receipts
- One cold, three warm, and three test-only 16-core measurement receipts with queue, cache, timing, reliability, and cost accounting
- Recomputed performance and cost decision bound to frozen adoption thresholds
- Production routing of adl-rust-tests and heavy Rust validation producers to adl-ubuntu-24.04-16core with unchanged check identities and validation semantics
- Removal of the temporary experiment-dispatch harness
- Green exact-head production canary plus focused validator, workflow-contract, diff-hygiene, and independent review evidence

## Acceptance

1. WP-02, WP-02A, organization plan, owner budget and alerts, selected-repository access, bounded maximum concurrency ten, security, and rollback entry gates are verified
2. Exactly one cold, three warm, and three test-only 16-core measurements are retained with finite nonnegative timings, queue and cache evidence, and warm cache-hit proof
3. The issue-local validator recomputes statistics, reliability, and cost from retained per-run accounting and rejects denominator, toolchain, runner, report, threshold, or production-canary drift
4. The test-only p95 is at most 120 seconds and its median duration improves by at least 35 percent over the warm baseline
5. adl-rust-tests and the heavy Rust validation producers run on adl-ubuntu-24.04-16core before merge with required-check identities, commands, validation breadth, and proof semantics unchanged; main pushes do not automatically repeat paid validation, while scheduled and manual full validation remain available
6. The temporary workflow-dispatch experiment harness is absent from the final implementation
7. The implementation PR passes an exact-head direct adl-rust-tests production canary on the selected 16-core runner without substituting a skipped test shell
8. The focused evidence validator, workflow-routing contract, diff hygiene, and independent exact-head review pass

## Dependencies

- WP-02
- WP-02A
- Agent Logic organization-owner approval for budget and selected-repository runner access

## Inputs

- .csdlc/prepared/issues/5853/design.md
- .csdlc/prepared/issues/5853/validate-experiment.rb
- .github/workflows/ci.yaml
- docs/tooling/BUILD_PLATFORM_BENCHMARKS.md
- docs/tooling/VALIDATION_PLATFORM_ROUTING.md
- docs/tooling/DEVELOPER_THROUGHPUT_FAST_LANE.md
- docs/tooling/HARDLINKED_RUST_DEPENDENCY_CACHE.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh

## Non Goals

- AWS or a self-hosted runner platform
- Organization-wide larger-runner defaults
- Changes to validation breadth, test semantics, required-check names, or branch protection
- 32-core, custom-image, ARM64, or self-hosted experiments
- Treating runner provisioning, cache existence, or planning prose as acceleration proof

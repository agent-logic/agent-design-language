## Metadata

- Skill: `repo-review-tests`
- Reviewer identity: Codex test/PVF/CI specialist (`/root/review_313_tests`)
- Target: repository-wide review of exact revision `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Date: 2026-08-25 UTC
- Artifact: `docs/reviews/v0.92/internal-review-5846/specialists/tests.md`
- Finding count: 2 (`P1`: 1, `P2`: 1)

## Findings

- P1: Independent Rust packages can change without their own tests or compilation running in CI
  File: `.github/workflows/ci.yaml:386`
  Role: tests
  Scenario: A pull request changes `adl-characterization/**`, `adl-resilience/**`, or `tools/remote_validation/**` without also changing a path that selects a separately owned Rust lane.
  Impact: The required `adl-ci` aggregate can be green while the changed package is not compiled and its package-local tests are not run. This leaves executable characterization, shared resilience, and portable remote-validation contract behavior without change-triggered CI proof.
  Evidence: The ordinary Rust jobs set `working-directory: adl` and run that manifest's fmt, Clippy, nextest, and doc-test surfaces (`.github/workflows/ci.yaml:386-468`). The focused runtime job covers only `adl-runtime-kernel` (`.github/workflows/ci.yaml:351-384`). The path policy explicitly selects standalone packages only for `csdlc-v2/*` and `adl-v2/*` (`adl/tools/ci_path_policy.sh:1669-1677`), its fallback Rust routing recognizes only `adl/src/*`, `adl/tests/*`, and the `adl` manifest (`adl/tools/ci_path_policy.sh:1737-1760`), and its normalized classifier labels all `tools/*` as workflow tooling while leaving `adl-characterization/*` and `adl-resilience/*` unknown (`adl/tools/ci_path_policy.sh:1925-1943`). No selector entry names any of these three package roots. Yet the omitted packages have real proof: the exact-target-compatible local runs executed 35 tests in `adl-characterization`, 4 in `adl-resilience`, and 12 in `tools/remote_validation`, all passing. Add explicit path ownership and package-local test/format/Clippy lanes, plus routing contract cases proving each package change selects its lane.

- P2: The generated specialist assignment gives the test reviewer an empty denominator
  File: `docs/reviews/v0.92/internal-review-5846/specialist_assignments.json:177`
  Role: tests
  Scenario: The packet is handed to the tests specialist using the generated assignment map.
  Impact: The reviewer receives no assigned files even though the inventory contains test paths, so a mechanical or bounded review can truthfully inspect nothing and still produce an artifact. This makes test-review coverage non-reproducible and biases the review away from the repository's actual test and CI surfaces.
  Evidence: `specialist_assignments.json` records `"tests": []`, while `repo_inventory.json:653-693` records a 40-entry test sample. The sample is also only the first bounded slice, ending in `adl-runtime-kernel/tests/guardian_soak.rs`, and therefore is not an exhaustive repository test denominator. Generate a non-empty test assignment from all tracked test/fixture/runner/CI/PVF surfaces, record total and sampled counts separately, and fail packet validation when the tests lane is required but empty.

## Reviewed Surfaces

- Review packet metadata, scope, inventory, evidence index, and specialist assignments.
- All 17 workflow paths listed by the packet, with detailed inspection of `.github/workflows/ci.yaml` routing and aggregation.
- `adl/tools/ci_path_policy.sh` path selection and normalized change classification.
- `adl/config/validation_lane_selector.v0.91.6.json` lane ownership and path selectors.
- Cargo manifests and test trees for `adl`, `adl-characterization`, `adl-resilience`, `adl-runtime`, `adl-runtime-kernel`, `adl-v2`, `csdlc-v2`, `tools/aws_remote_validation`, and `tools/remote_validation`.
- Packet denominator: 23,622 tracked files, 341 manifest candidates, 17 CI files, and the packet's 40 sampled test paths. Detailed behavioral inspection was targeted to CI/PVF routing and the three uncovered independent packages; it was not a line-by-line review of every test in the repository.

## Missing Proof Map

| Behavior / surface | Existing proof found | Missing proof |
|---|---|---|
| Changes under `adl-characterization/**` | 35 local tests pass | No path-triggered CI/PVF owner lane; no routing regression case |
| Changes under `adl-resilience/**` | 4 local tests pass; package is consumed by runtime and C-SDLC crates | No direct package lane ensuring its own test/format/Clippy surface runs for a package-only change |
| Changes under `tools/remote_validation/**` | 12 local contract tests pass | No path-triggered Rust lane; `tools/*` is classified as workflow tooling |
| Required repository test specialist lane | Inventory samples 40 test paths | Assignment is empty; no exhaustive or risk-selected denominator and no fail-closed packet check |

## Validation Performed

- Verified the repository contains target commit `c6792e54df1db5969fa28c59b6dfe4c714ed5559` and used `git show`, `git grep`, and `git ls-tree` against that exact revision for source, test, manifest, selector, and workflow inspection.
- Confirmed `git diff --name-only c6792e54df1db5969fa28c59b6dfe4c714ed5559..HEAD` is empty for the three tested package roots and the reviewed CI/PVF routing files; package test execution therefore used source identical to the exact target.
- `cargo test --locked --manifest-path tools/remote_validation/Cargo.toml`: passed, 12 integration tests plus zero unit/doc tests.
- `cargo test --locked --manifest-path adl-characterization/Cargo.toml`: passed, 35 tests plus zero doc tests.
- `cargo test --locked --manifest-path adl-resilience/Cargo.toml`: passed, 4 tests plus zero doc tests.
- Static selector check: no occurrences of `adl-characterization`, `adl-resilience`, or `tools/remote_validation` in `.github/workflows/ci.yaml`, `adl/tools/ci_path_policy.sh`, or `adl/config/validation_lane_selector.v0.91.6.json` at the exact target.

## Residual Risk

- Broad workspace suites, provider-bound tests, AWS workflows, slow proofs, soak tests, and full coverage were not executed in this specialist lane.
- The three focused package suites prove the current source passes locally; they do not mitigate the missing change-triggered CI ownership.
- The review packet samples only 40 test paths and assigns none to the test lane, so repository-wide absence-of-gap claims are not supportable until the packet denominator is repaired.
- Implementation correctness, security exploitability, documentation truth, dependency posture, and architecture are owned by their respective specialist lanes.

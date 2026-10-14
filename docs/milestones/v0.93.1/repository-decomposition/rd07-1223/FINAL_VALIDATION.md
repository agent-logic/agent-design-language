# Final local qualification for #1223

The frozen production source is `ad8204be76528a2c13ffabf5ee1e2672df475c8b`. [The validation record](FINAL_VALIDATION.json) pins the actual results and earlier adverse evidence.

- Retirement: 3,407 actions (3,405 baseline and two transient removals), with the original 4,134 and captured additional 22 identities preserved. No unresolved rows or holds remain. The prior ad8204 guard at 3,406 actions passed both Rust wrapper tests, including all 57 Python regression cases. The proposed 3,407-action guard still requires its focused/full verification.
- Linux: a fresh exact-source clone in an isolated container passed all 15 package test/fmt/clippy commands and the retained public CLI runner. Network was disabled and no host directories were mounted. The initial pre-build failure remains recorded.
- macOS installed artifacts: five public artifacts passed seven consumer cases, 21 UTS fixtures and five negative cases under enforced sandbox denial. The earlier sandbox setup failure ran no consumer and remains recorded.
- Coverage: the reviewed prior six-package measurement is reused. All 42 final-source file hashes match the correspondence review; 39 match the measured source and three have reviewed formatting, help-text or equivalent-predicate changes. No final-source coverage percentage is claimed.

The five-repository build matrix is complete at its recorded revisions and bounded scopes. It does not assert that every private repository was rebuilt at its latest HEAD. Private-owner CLI evidence includes actual installed Runtime and CodeFriend cases; those earlier source-bound results remain distinct from final synthetic routing tests. A fresh final CLI invocation against authentic owner installations is not claimed: its explicit reuse disposition or bounded proof remains to be resolved.

Independent final review, required remote checks and native publication, reconciliation, finish and cleanup remain separate delivery steps. This update does not edit native lifecycle cards or claim issue completion. Historical Unity retirement follows the user's decision to discontinue it; failed editor attempts remain preserved and are not recast as acceptance.

The subsequent source commit `2095d3707487cfd8e5a9239691cea96f6f4a31a3` removes only the unused private `adl/src/cli/run.rs` and adds its receipt. [Exact source correspondence](FINAL_SOURCE_CORRESPONDENCE.json) verifies unchanged public package, compiled CLI and packaging inputs. Qualification remains attributed to ad8204; the new guard/accounting delta requires focused validation rather than a claimed heavy rerun.

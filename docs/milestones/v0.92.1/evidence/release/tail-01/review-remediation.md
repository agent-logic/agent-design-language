# PR #748 review remediation

The three findings from review of `899b60aab7b374a68284828d28a1308e0a1c9f9c` are addressed in the candidate implementation. Final exact-head review and publication receipts are separate evidence; this document does not claim merge or release acceptance.

- Branch reconciliation: one shared check validates Git branch syntax and the existing typed-argument safety contract before durable intent creation. The complete owner/head query value is percent-encoded, preserving plus, ampersand, equals, Unicode, hash, and percent characters. The supported contract deliberately excludes Git names rejected by the existing typed-argv secret/shell-string guards; these now fail before creation rather than during recovery. This addresses the behavior tracked in #746 without independently closing that issue.
- Review path evidence: a missing or malformed path list is rejected. Declared paths must equal the actual reviewed-to-implementation Git diff. Rename detection is disabled so both the removed source and added destination are checked. Only a complete metadata-only diff preserves review truth.
- Lifecycle records: all six legacy renderings under the native state index are regenerated through native edit and the active 1.0.4 registry. SRP and SOR record remediation, validation, review boundaries and the blocked release outcome. Original cards remain in Git history.

## Validation and boundaries

Local native library tests: 45 passed. Remote-publication integration: 12 passed. Operational CLI integration: 10 passed. Formatting and Clippy with warnings denied pass. Native six-card validation passes using the retained installed owner binary with worktree-routing and schema support from the preserved #749 extraction; this is operational tooling provenance, not an extra implementation change in #748.

Quality-gate validation passes for 393 inventoried rows and 366 required lanes. Eleven negative cases pass, including missing and incomplete review-path lists. New native branch tests are deterministic local CPU/Git contracts with fake transport; no live GitHub mutations or paid resources are used as test proof. The restart regression reuses the durable create intent for `codex/fix+retry` without repeating creation.

The release decision remains **BLOCKED**: 121 passing rows, 245 non-proving rows, zero absent rows, and five owned unresolved exceptions. A passing validator establishes correct gate computation, not satisfaction of the release acceptance criteria. Hosted checks and final exact-head review must pass before this candidate is ready for normal PR integration.

Template version 1.0.4 replaces locked pre-execution SRP literals and SPP pending statuses with typed value fields; version1.0.3 remains unchanged. This versioning is necessary for truthful post-review cards. The local-command active-registry fixtures follow the new version.

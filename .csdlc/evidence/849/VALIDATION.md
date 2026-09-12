# Issue #849 local validation and review

Implementation candidate: `0264f321ac0e19fb9ae957b4f05b6dfb2f7c63b2`.
Starting baseline: `57a82b08bf687afd0b1573fd8d78f4b2b2268dba`.

- `cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_cases`: 16 passed.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`: passed.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`: passed.
- `python3 docs/csdlc-v3/man/render.py --check`: 30 pages verified.
- `git diff --check`: passed.
- Full `cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1`: **failed**, release-preflight fixture `exact_candidate_preflight_and_negative_matrix` reports `release_inventory_omits_manifest: adl-uts/Cargo.toml`.
- Diagnostic remaining-suite run with explicit `--skip exact_candidate_preflight_and_negative_matrix`: 253 passed, 1 filtered. This is partial coverage, **not a green full suite**.

The failure's production guard, test fixture, release inventory and omitted
manifest have identical Git blobs at baseline and candidate; see
`release-preflight-baseline-defect.json`. It is routed separately. No release
validator bypass, inventory repair or failure waiver is included in #849.
Local full logs are retained under `.adl/runs/849/` until terminal archive.

Independent reviewer `sprint8_909` accepted this exact implementation candidate
with no actionable findings. Reviewer reran all 16 merge tests and the one
qualified numeric adapter test; inspected mode/digest/qualified-target binding,
initial/fresh/postmerge observations, complete-page refusal, no-second-PUT
replay, compatibility and explicit remote race boundaries. This review does
not claim the full suite or CI passed. Final card/integration revisions require
review renewal before publication.

A read-only GitHub GraphQL schema smoke on already-merged PR #939/issue #720
returned no GraphQL errors, merged=true, closing issue720 in the relation set,
and qualified issue state CLOSED. No live merge was attempted. All merge
behavior proof uses deterministic fake authenticated transport and local Git.
The PVF required owner contract is small CPU/filesystem with no paid resources.
Machine-readable payloads remain stdout; diagnostic failures remain stderr.
No token, credentials, raw PR bodies or environment dumps are retained here.

#948 remains a separately owned unmerged change. Its remote pending-receipt
assertions and latest adapter curl-config guard must be preserved on integration;
this candidate does not claim them as delivered. Shared binaries were not installed.

# Native retirement and integrity guards (#1350)

The eight assigned Python files are replaced by the standalone Rust package at
`tools/source_retirement/validator`. The CLI, native Cargo adapter, portable CI
successor gate, cleanup aggregate and podcast test caller use no Python or Ruby.
The old uncalled 70-removal helper remains available in Git history; current
admission uses the unchanged 3,407-action execution-overlay contract.

## Preserved evidence and behavior

All 29 literal constants from `check.py` at
`d064c65cb1b40994c74c3ddc9d2dc0157a727c73` match `reviewed-identities.json`
exactly (sets serialized in sorted order). The literal report fields are also
preserved. JSON contains identities, hashes, custody locations and counts, not
interpreter source. Native source pins the complete JSON bytes.

Original manifests, receipts, inventory denominators, held-group accounting,
protected paths, transient add/remove chains, 3,407 executed actions and rollback
requirements retain their original meaning. The guard reads committed Git
objects; dirty files cannot substitute approval inputs. Archive verification
checks pinned bytes, regular members, modes and content hashes. Exact supplemental
member sets remain exact; unrelated deletions and retained-file drift fail closed.

The native implementation additionally rejects duplicate archive members and
custody references to undeclared archives. These fail-closed corrections do not
admit new evidence or change valid checked-in custody. All existing custody
archives pass the native contract tests. Explicit after-identities still allow
only the original regular/executable/symlink Git modes.

## Exact successor reconciliation

`guards-transition.json` covers this issue's eight removals and six existing
caller/build/documentation changes, with exact before/after identities and
retrievable Git preimage hashes. New Rust/data files are independently reviewed;
none is an archived or renamed interpreter program.

The pre-existing #1356 failure first surfaced at `demos/README.md`. The separate
`podcast-successor-transition.json` pins the visitor-journey and offline-example
successors, preserving the original podcast manifest unchanged.

Further retained drift after the original #1223 integration is recorded separately
in `retained-successor-transition.json`: **53 modifications, zero removals**, from
14 commits already ancestors of the accepted main baseline above. The exact old
identities all match `c0a9983bac18f9bab3a03134b8fe58c8083b95ae`. Each row checks its
old blob hash, new blob hash, exact successor commit identity and ancestry. This
includes later public package license/YAML changes, session and observability
fixes, demo safety, README/public guidance and v0.93.2 planning updates. It is a
bounded historical reconciliation, not a general exemption for these files.
No historical evidence file is rewritten by this issue.

## Validation and review

The original 57 Python regressions passed before removal. Native `contracts`
contains ten grouped tests covering inventory/held groups; execution identity,
mode, custody and deletion refusals; both transient chains; separate baseline
paths; committed-input isolation; references; all original family scopes; all
119 recent family applications; and actual pinned custody plus adversarial archive
fixtures. This groups equivalent scenarios without treating test count as proof.

The six portable `successors` tests cover original cleanup/readiness/podcast
transitions, exact language overlays, current HEAD admission, and retained
successor provenance. The podcast's five JavaScript tests invoke the native
negative test and retain destination/feed/historical-evidence checks.

Independent review identified a missing actual-HEAD CI check; it was restored.
The CI successor gate uses the real committed tree as well as reconstructed
positive and tampering fixtures. Full historical custody remains a provisioned
local gate; CI does not pretend to possess those external archives.

Validation commands:

- `cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test contracts`
- `cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test successors --test retirement_contract`
- `cargo clippy --locked --manifest-path tools/source_retirement/validator/Cargo.toml --all-targets -- -D warnings`
- `cargo run --locked --manifest-path tools/source_retirement/validator/Cargo.toml -- --cleanup-contracts`
- `node --test tools/test_podcast_migration.mjs`

Final committed audit, interpreter-trap proof and CI outcomes are recorded in the
PR. A missing or corrupt original archive still blocks full audit admission.
Git/gh delivery uses the operator-authorized fallback for the already recorded
C-SDLC #330/#331 regressions; this does not claim native terminal reconciliation.

# Issue 861 operator manual proof

Outcome: 30 installable manual pages cover 26 root-help commands, the supported
rollback alias, both simple issue forms, the lifecycle workflow and 207 request
fields. The owner installation flow refreshes pages independently of binary
provenance. No lifecycle production semantics or active owner binary changed.

Source baseline: `2c09eca135808fb6385a6613f7317088b01ffeab` plus the issue's
reviewed change. `source-manifest.json` binds tested source bytes. Validation
logs normalize only the absolute issue-checkout prefix for publication.

## Local validation

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test operator_man_pages --example operator_manual`:
  six integration checks and two example checks pass. They cover command and
  parser-option parity, missing-command/new-option rejection, request-field
  parity, generated-page parity, native schema/guard examples, primary/linked
  cleanup receipt-location compatibility, digest helpers and real registration
  observation.
- Focused Clippy with warnings denied and crate formatting check pass.
- `render.py --check` and all 30 `mandoc -Tlint -Wwarning` checks pass.
- `git diff --check` passes.

## Installation proof

`test_install.sh` uses a clean isolated repository and a clearly identified
fixture executable through `install_owner_binaries.sh --bin csdlc --no-build`.
It checks all 30 pages with the real host `man` lookup and formatter, a prefix
containing spaces, documentation refresh while the owner's inode is unchanged,
and denial when the overview source is missing.

The same script passes on macOS and in an isolated Amazon Linux 2023 container.
Separate platform logs and metadata are in `validation/`. The existing local
Colima instance was started for this test without changing its configuration.
This proves manual installation/discovery on both platforms, not a Linux Rust
binary build, live owner activation or live GitHub workflow execution.

## Review and truth boundaries

Preliminary independent review found two P2 defects: overstating the native
review classifier and omitting primary reconciliation between linked finish and
primary cleanup. Both were corrected. A dedicated negative/positive Git fixture
now demonstrates the cleanup receipt-location boundary. Exact-head review is
recorded separately by the native publication process.

Schema examples use synthetic identities/digests and do not impersonate
authenticated reviews or remote readbacks. Native publication stays blocked
without actual receipt evidence; historical shadow/soak examples remain blocked.
The manual documents current diagnostic side effects, explicit credential-file
requirements and the post-creation publication-readback requirement. Unmerged
issue 867 behavior is not claimed as installed. No paid services, real GitHub
validation mutation, merge, terminal issue closure or live binary replacement
is part of this proof.

PVF: required tooling lane, deterministic source/fixture checks, small local
CPU/Git/filesystem and an isolated local Linux test environment. CI state and
independent exact-head review are separate from these local checks.

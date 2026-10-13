# Native operator manual

`manual.json` is the reviewed source for the installable roff suite in `man1/`.
Read `man csdlc`, `man csdlc-workflow`, `man csdlc-requests`, or a command page
such as `man csdlc-finish`. The suite contains 34 pages covering all
30 root-help commands, the supported `rollback` alias and both simple issue
forms. This extends the frozen 27 predecessor dispositions with `status`,
`prepare`, `recover` and `rebuild`; it does not change that historical denominator. `inventory.json` binds options
and request fields to their source contracts. Source/manual coverage is
separate from installed-binary provenance, exact-head review and activation.
See [issue intent commands](../INTENT_COMMANDS.md) for the ordinary interface and
shared advanced request.

## Installed owner manuals

Operational lookup uses the manuals delivered with the exact admitted C-SDLC
package, not this ADL source tree. Native `package-install` already includes the
roff pages under `installed/resources/docs/csdlc-v3/man/man1`; it does not create
`share/man`. No additional manual installation or copy is needed.

After a reviewed native installation succeeds, use its actual `executable` output:

```sh
# Set this to the exact executable returned by the successful native installation.
CSDLC_EXECUTABLE=/absolute/package/installed/bin/csdlc
bash adl/tools/install_csdlc_man_pages.sh --stable-bin-dir "$(dirname "$CSDLC_EXECUTABLE")"
# Use the exact MANPATH export printed above, then:
man -w csdlc-workflow
man csdlc-workflow
```

The retained `install_owner_binaries.sh --bin csdlc` compatibility path performs
only this lookup, including on a no-op request. It never refreshes manuals from
ADL source, copies a supplied source executable, or replaces the selected owner.
Public `adl` and `adl-process` installation remains separate. The explicitly
restricted `csdlc-transition` source route is unchanged by this manual cutover.

For first installation, an already authorized native owner accepts
`csdlc package-install --request REVIEWED_REQUEST.json`. The strict request has
`schema: csdlc.v3.portable_package_install.v1`, `repository_root`, `artifact_path`,
and `observation`. Observation requires `pull_request`, `producer_issue`,
`workflow_run`, `artifact`, `candidate_revision`, `merge_revision`,
`default_branch`, `workflow_attempt`, and `review_attestation_sha256`.
Use the original authenticated artifact ZIP and real independent-review
attestation; do not invent these values. This is a repository-bound first-import
route: it requires the actual primary consumer checkout, preserves existing
activation receipts and shared installs, authenticates current remote provenance,
and requires successful installed-child admission before reporting verification.
It is not a generic overwrite or shared-selector replacement command.

Missing/stale admission remains a native recovery/install boundary. A manifest,
manual directory, successful lookup, or local source build alone does not grant
operational authority. New package revisions deliver new paired manuals atomically;
do not edit an installed package to refresh documentation.

## Retained source authoring reference

Edit the JSON source, then run `python3 docs/csdlc-v3/man/render.py`. Do not edit
generated roff directly. The renderer uses only Python's standard library;
installation needs Bash and ordinary POSIX file utilities. Lookup needs the
host's `man` implementation. macOS ships `man` and `mandoc`; Linux distributions
may require their `man-db`/`man` and roff formatter packages.

The optional `csdlc-v3/examples/operator_manual.rs` helper reads native digests
and registrations using public APIs. It requires the repository Rust toolchain
and Cargo dependencies, is not installed by the owner installer, and grants no
lifecycle or authentication authority:

```sh
cargo run --quiet --manifest-path csdlc-v3/Cargo.toml --example operator_manual -- selector-digest .
cargo run --quiet --manifest-path csdlc-v3/Cargo.toml --example operator_manual -- review-digest docs/csdlc-v3/man/examples/typed-review.json
```

Examples are fixtures, not approved operational targets. Tests deserialize every
request family and exercise actual native plan/guard contracts. They do not
claim a live GitHub end-to-end delivery. Current diagnostic side effects,
credential-file requirements and the post-creation publication-readback gate
are described explicitly. Direct issue-writer and administrative-writer
`--request` forms are retained only as retired discovery surfaces; current
execution uses semantic issue intents or exact generated `--intent-request`
snapshots.

Validation:

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --test operator_man_pages
cargo test --manifest-path csdlc-v3/Cargo.toml --example operator_manual
bash docs/csdlc-v3/man/test_install.sh
TMPDIR="$PWD/target" bash adl/tools/test_owner_binary_install.sh
git diff --check
```

PVF classification is in `pvf.json`. Platform results and exact command outcomes
belong in the issue evidence packet. A test run on macOS is not Linux proof.

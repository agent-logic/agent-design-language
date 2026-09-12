# Native operator manual

`manual.json` is the reviewed source for the installable roff suite in `man1/`.
Read `man csdlc`, `man csdlc-workflow`, `man csdlc-requests`, or a command page
such as `man csdlc-finish`. All 26 root-help commands, the supported `rollback`
alias, and both simple issue forms are covered. `inventory.json` binds options
and request fields to their source contracts.

```sh
python3 docs/csdlc-v3/man/render.py --check
bash adl/tools/install_owner_binaries.sh --bin csdlc
export PATH="$PWD/.adl/bin/native-v3:$PATH"
export MANPATH="$PWD/.adl/bin/native-v3/share/man:${MANPATH:-}"
man csdlc
man csdlc-finish
```

The owner installer refreshes the manual even if binary provenance is unchanged.
It does not modify shell startup files, install system-wide pages, or treat
documentation changes as a reason to rebuild the binary. To install only pages
into an isolated prefix:

```sh
bash adl/tools/install_csdlc_man_pages.sh --stable-bin-dir ./target/man-install
MANPATH="$PWD/target/man-install/share/man:${MANPATH:-}" man csdlc
```

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
are described explicitly; unmerged simplification work is not documented as
already installed.

Validation:

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --test operator_man_pages
cargo test --manifest-path csdlc-v3/Cargo.toml --example operator_manual
bash docs/csdlc-v3/man/test_install.sh
git diff --check
```

PVF classification is in `pvf.json`. Platform results and exact command outcomes
belong in the issue evidence packet. A test run on macOS is not Linux proof.

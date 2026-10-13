# Public CLI Runtime selection

`adl <workflow.yaml> --run`, implicit execution for workflow versions other than
`0.1`, and `adl resume` execute through an explicitly selected Runtime installation.
First qualify that installation using Runtime's package installer. Then configure
both values for the ADL command:

```sh
export ADL_RUNTIME_INSTALL_PREFIX=/absolute/path/to/qualified/runtime
export ADL_RUNTIME_ARCHIVE_SHA256=<qualified-archive-sha256>
adl workflow.yaml --run
```

The prefix must be absolute and unaliased. Runtime's managed `current` symlink must
select `versions/<qualified-archive-sha256>`. ADL checks that selection, retained
archive identity, host manifest and executable metadata. Runtime owns full package
verification at installation; ADL does not repeat payload hashing on each command.
The qualified installation directory must remain trusted after installation.
Missing configuration or a mismatched selection fails before owner execution.

Ordinary `--print-plan` and `--print-prompts` presentation remains local when no
execution flag is supplied. Provider-profile expansion, overlays and trace output
belong to Runtime and therefore require the selected installation. ADL retains its resume argument/help diagnostics.
The public dispatcher delegates the other Runtime command families to that same
selected owner. Process status and repository session coordination remain local.

Delegation preserves argument boundaries, current directory, environment and
standard streams. Unix process replacement preserves owner exit status and signals.
ADL emits its existing dispatch event; Runtime emits an additional event identifying
its own dispatch. This is an intentional ownership distinction. Existing
`ADL_OBSERVABILITY=0`, `ADL_OBSERVABILITY_STDERR=0` and
`ADL_OBSERVABILITY_LOG` settings are inherited. Machine output stays on stdout and
ordinary observability stays on stderr unless the configured compatibility logging
controls select otherwise.

The #1223 evidence packet records actual Runtime package installation, installed
transport and pause/resume checks, legacy behavior comparison, and focused public
CLI dispatch tests. Those are distinct from the remaining manifest/source removal
and final public package acceptance.

## CodeFriend commands

`adl codefriend ...` consumes a CodeFriend installation qualified by the product's
`tools/product_distribution.py` installer. Set `ADL_CODEFRIEND_INSTALL_PREFIX`,
`ADL_CODEFRIEND_ARCHIVE_SHA256` and `ADL_CODEFRIEND_CANDIDATE` from the independently
authenticated owner package. ADL checks the existing `INSTALLATION.json` and
`manifest.json` identities and executable metadata in that trusted directory.
The installation must contain a binary compatible with the executing host.

CodeFriend receives the original argument boundaries and inherited streams and
returns its own exit status. Its human errors omit the old ADL `Error: ` prefix;
error identifiers and channels are preserved. JSON objects may use a different
key order. The older x86 distribution used to check the receipt-selection contract
is distinct from the newer ARM64 binaries used for current installed execution
proof; neither evidence record is relabeled as the other.

## Public build and installation

The ADL crate builds `adl` and `adl-process`. Its library reexports portable
contracts and retains local process/session support. The public installer defaults
to these two binaries; Runtime and CodeFriend binaries are installed from their
owner packages. `--bin csdlc` still uses the separate native owner route while its
external package cutover is being completed.

`bash tools/public_adl/run_cli_ci.sh` checks the public dependency closure and runs
the retained public library, CLI, document, resolver, schema and process assertions,
plus installed-owner selection and installer contracts. It does not substitute for
successor product integration tests or the five public package acceptance lane.

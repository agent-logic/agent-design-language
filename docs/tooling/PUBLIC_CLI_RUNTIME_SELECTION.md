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

Explicit `--print-plan` and `--print-prompts` presentation remains local when no
execution flag is supplied. ADL retains its resume argument/help diagnostics.
Other command families retain their existing implementation during the remaining
repository partition.

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

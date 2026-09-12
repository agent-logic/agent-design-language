# CI repository acquisition

`adl codefriend ingest ci` acquires the same immutable scoped Git packet as
`ingest local`. It requires explicit checkout, canonical HTTPS repository,
full commit revision, and scope. CI environment variables never select source
revision or widen scope. Repository instructions remain inert source text.

```sh
.adl/bin/adl codefriend ingest ci \
  --checkout /path/to/fixture-checkout \
  --repository https://example.com/team/repository \
  --revision FULL_SOURCE_COMMIT \
  --scope scope.json \
  --candidate-revision FULL_INSTALLED_CANDIDATE_COMMIT \
  --metadata ci-metadata.json \
  --out artifacts/packet.json \
  --receipt artifacts/receipt.json
.adl/bin/adl codefriend packet read --input artifacts/packet.json
```

Output parents must already exist outside the source checkout. Outputs are
create-only. Scope uses the local adapter's analysis/context path lists and
file/byte bounds. The metadata file is a bounded JSON object (4096 bytes) with
only optional `run_id`, `run_attempt` (decimal strings), and `job` (ASCII letters,
digits, hyphen or underscore; 100 characters maximum). `{}` is valid. Supply
only those deliberately selected values; do not dump the environment.

The separately bound receipt names packet identity, source revision, candidate
revision and metadata. Candidate revision is declared provenance, not binary
attestation; the smoke also hashes the actual installed executable. Run metadata
does not participate in the common packet identity. Moving a checkout or changing
run metadata leaves the packet unchanged. Neither packet nor receipt retains
runner checkout paths. Acquisition success does not imply review success,
fitness approval or artifact delivery.

Missing, malformed, unresolvable or non-commit revisions, mismatched repository
origin, path escapes, symlinks, limits and credential-shaped metadata fail with
category-only diagnostics. Available commits in shallow checkouts work; missing
commits or unavailable Git objects fail rather than triggering network fetch.
Missing scoped paths and deliberately omitted unsafe/binary content produce an
explicit partial packet. An error is never a complete packet claim.

The command returns JSON on stdout and failure diagnostics on stderr. It does
not change compatibility logging or add telemetry. A receipt write failure may
leave a valid packet already published, but the command fails and never reports
successful delivery. Use new output filenames when retrying after inspecting
such partial output; existing artifacts are not overwritten.

The required `codefriend-ci-acquisition` job builds and installs the exact PR
candidate, runs the production route and reader on a bounded deterministic Git
fixture, compares local/CI packets, and uploads packets, receipts and proof.
`proof.json` binds source revision, candidate revision, actual binary SHA256,
per-artifact hashes and executed case count. Upload must succeed with nonempty
artifact ID/digest; missing artifacts or failed upload fail the required job.
The aggregate requires successful selected execution and skipped unselected
execution. Hosted CI success is separate from local smoke success. Artifacts
expire after seven days; a past upload receipt is not a claim of perpetual
retrievability. No provider credentials or inference calls are needed.

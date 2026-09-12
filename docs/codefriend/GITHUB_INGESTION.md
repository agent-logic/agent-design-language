# GitHub repository ingestion

`adl codefriend ingest github` performs read-only acquisition from GitHub's Git
data API and emits the same repository packet consumed by local ingestion. It
never clones/checks out a repository, runs repository code, invokes a model, or
changes an issue or PR. Source instructions remain inert text.

Install the candidate with `bash adl/tools/install_owner_binaries.sh --bin adl`.
Use the sorted explicit scope and bounds documented in [local ingestion](LOCAL_INGESTION.md):

```sh
.adl/bin/adl codefriend ingest github \
  --repository https://github.com/owner/project \
  --revision FULL_LOWERCASE_SHA1 --scope scope.json --out new-capture
.adl/bin/adl codefriend ingest github \
  --repository https://github.com/owner/project \
  --pr 123 --scope scope.json --out new-pr-capture
.adl/bin/adl codefriend packet read --input new-pr-capture/packet.json
```

Exactly one of `--revision` and `--pr` is required. Branch names, abbreviated IDs,
revision expressions and SHA-256 inputs are rejected by this GitHub adapter;
the shared packet/local adapter still support both Git object formats. Repository
identity must match GitHub's canonical HTTPS `html_url` and `full_name`, without
`.git`, credentials, aliases, extra path components or query strings. The product
currently targets github.com; other hosts are rejected rather than guessed.

A PR is resolved to its head repository and full commit before source reads. For a
fork PR, the packet uses that fork's repository identity, allowing exact local
parity. `provenance.json` separately records the originally requested repository,
`refs/pull/<number>/head`, resolved source/commit, packet ID, actual request count,
transport class and successful head recheck. Changed PR heads/repositories fail
before output. Commit/tree metadata is trusted to the authenticated GitHub HTTPS
API; each fetched blob is independently checked against its Git SHA-1 object ID.
This is not a signed commit-membership proof. Controlled HTTP evidence is identified
as such and is never labelled authenticated live GitHub evidence.

`new-capture` is a create-only directory containing `packet.json` and
`provenance.json`; existing directories/files are never replaced. Both are read
back before successful stdout. Acquisition failures create no output; write
failures return failure and attempt to remove their own incomplete files. The
production `github::Acquisition::read` validates the receipt's association with
the unchanged shared packet contract. Receipt validation is integrity checking,
not authorization or a cryptographic attestation. Packet identity excludes all
transport/provenance fields, credentials and local paths.

GitHub requests are GET-only, pinned to the fixed API host, with redirects and
pagination rejected. Nonrecursive tree traversal visits only directories needed
for the exact selected paths; trees are not recursively downloaded. Limits:
2 MiB per metadata response, 8 MiB cumulative response bytes, 4,096 requests,
120 seconds per capture, 20 seconds per request and five seconds to connect.
The existing maximum 1,000 files/1 MiB total source and declared per-file limits
apply before blob reads, including omitted content. Large metadata, truncated
responses or missing expected blob objects fail; a genuinely absent selected tree
entry is recorded as `missing` and the packet is explicitly partial. Symlinks,
including intermediate path components, fail. Binary, unsafe and unsupported
objects retain only the existing omission records. No whole-repository claim is
made. Follow the exact ten-file/600 KiB Vector selection when it is the selected
input; this command does not authorize widening that scope or running Vector.

Credentials use the existing shared resolver (environment, approved token file,
keychain/default approved file precedence). No new resolver or token CLI argument
exists. Tokens are sensitive headers only; HTTP response bodies and transport
errors are never copied into diagnostics. Authentication/forbidden/not-found,
rate-limit, redirect, pagination, malformed/truncated data and identity errors
return bounded static reasons. Proxies are disabled for this bounded transport.
No credentials, cache paths or hostile source content enter receipts or errors.
Source redaction and object validation remain the shared local contract.

For deterministic installed-command proof, `--fixture-api http://127.0.0.1:PORT`
(or a literal IPv6 loopback address) uses the same Git-data HTTP implementation.
This explicit mode never resolves or forwards credentials, even if real token
sources are configured. It cannot select arbitrary remote hosts, redirects,
userinfo, paths or query strings. Its provenance says
`controlled_loopback_git_data_http`; it does not prove live GitHub availability.

Required proof is the `codefriend_github_ingestion` Rust integration target:
actual loopback HTTP, real Git object fixture bytes, command execution/reader,
commit/PR and fork parity, partial/hostile inputs, pin/transport failures and
aggregate metadata bounds. PVF lane runtime; required Beta ingestion gate;
deterministic local CPU/disk/loopback; no provider calls. Set
`ADL_CODEFRIEND_TEST_BINARY` to the installed candidate path to repeat CLI proof
against the stable owner binary. Default test runs use Cargo's candidate binary.
Live external corroboration, if separately run, must be recorded independently.

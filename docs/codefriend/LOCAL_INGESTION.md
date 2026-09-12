# Local repository ingestion

`adl codefriend ingest local` captures an explicit set of committed Git blobs into
one portable `codefriend.repository_packet.v1` artifact. It does not run repository
code, build dependencies, invoke a model, review the project, publish source, or
admit evidence to a durable store. GitHub/CI transports belong to #879/#880;
retention/deletion and governed evidence storage belong to #881.

Install the candidate `adl` with `bash adl/tools/install_owner_binaries.sh --bin adl`.
The installed command is `.adl/bin/adl`. Source checkout and output are explicit:

```sh
.adl/bin/adl codefriend ingest local \
  --checkout /path/to/checkout \
  --repository https://github.com/owner/project \
  --revision FULL_LOWERCASE_COMMIT_OBJECT_ID \
  --scope scope.json --out /path/outside/checkout/packet.json
.adl/bin/adl codefriend packet read --input /path/outside/checkout/packet.json
```

The repository identity must be a credential-free HTTPS URL without `.git`, query,
fragment, or userinfo. Its spelling must match `remote.origin.url` after removing
an optional `.git` suffix. Local-only repositories need this explicit provenance
configured by their operator; ingestion never adds or changes a remote. Full commit
object IDs (40 or 64 lowercase hex characters) are required, not tags, branches,
abbreviations or revision expressions. Acquisition never fetches missing objects.

A scope file declares every path and all limits. Both arrays are sorted and unique;
paths cannot appear in both. Paths are literal, relative, case-sensitive file names,
not globs or recursive directory expansions:

```json
{
  "analysis": ["src/lib.rs"],
  "context": ["Cargo.toml", "LICENSE"],
  "max_files": 10,
  "max_bytes": 614400,
  "max_file_bytes": 409600
}
```

The hard ceilings are 1,000 requested paths and 1 MiB of source bytes, with a
per-file limit no greater than the total. Bounds apply before blob reads and count
omitted unsafe/binary bytes as well. Exceeding a limit fails the entire acquisition
without creating output. Paths and scope metadata are bounded separately. The
packet reader caps serialized input at 8 MiB before parsing. A missing selected file
is an explicit `missing` object; a tree/submodule is `omitted_unsupported_object`.
Neither is silently expanded. Binary/non-UTF-8 inputs are `omitted_binary`.

## Snapshot and portability

Working-tree modifications, staged modifications and untracked files are excluded;
the packet records that policy. They are never reset, staged, copied or altered.
Git blobs come from the exact commit. Replacements, pathspec interpretation, lazy
fetch and repository code execution are disabled. HEAD and origin are checked
before and after capture; a change fails as `source_changed_during_capture`.
Changes to dirty/untracked bytes cannot alter the immutable selected snapshot.
A historical commit can be captured without checking it out.

Packet identity covers repository, full revision, canonical sorted scope and limits,
declared Git object format, object dispositions, admitted content and digests. It excludes timestamps and local
checkout/output paths. Identical committed inputs and scope produce identical bytes
after relocating the checkout. Every object carries its repo-relative identity and
source Git object ID, source size and (if included) BLAKE3 content digest. The packet
also carries its BLAKE3 scope digest and packet digest. The source repository's
storage object format (`sha1` or `sha256`) is explicit and participates in packet
identity. The reader independently hashes each included object's exact bytes with
Git's `blob <byte-length>\0` header using that algorithm and compares `source_object`.
Unknown formats, mixed object-ID lengths, revision/format contradictions and stale
Git blob IDs fail closed. Omitted objects cannot be rehashed without retaining the
omitted bytes; their Git IDs remain unverified source claims, not verified content.
These checks do not prove that a malicious producer's blob belongs to its claimed
commit: authenticated commit/tree provenance remains a separate trust boundary.
Packets from the pre-merge prototype without a declared object format are rejected;
preserve them as historical proof and reacquire rather than silently upgrading them.
These are integrity and
provenance claims, not a signature or proof that an untrusted producer is authorized.

All paths outside the declared scope are excluded by policy; no unbounded recursive
inventory is implied. Rust `.rs` inputs are identified as not yet analyzed. Context
and other languages explicitly have unsupported analysis coverage. Even
`complete_scoped_acquisition` always has `review_state: not_reviewed`.

## Unsafe inputs and output

Traversal, absolute/host paths, Git metadata paths and symlink objects are rejected.
Symlinks are never followed into the filesystem. Known credential filenames, private
key/token markers, credential assignments (including YAML), URL userinfo and machine-local path markers cause
whole-file omission before output. No raw unsafe content or raw-content digest is
retained. Known unsafe markers in scope paths are rejected, since paths themselves
are retained. Omission changes completeness to `partial`; consumers cannot relabel
it a complete acquisition or review. Unsupported/missing objects have the same
partial boundary. Repository instructions remain inert text.

This conservative marker policy is not a universal secret detector: unknown token
formats and arbitrary sensitive prose require operator scope review. The complete
marker list is the production `unsafe_content` function, shared by acquisition and
the production reader. JSON is scanned as inert data during deserialization: every decoded key, nested
object and array member is checked before duplicate members can overwrite earlier
ones. Escaped keys under duplicate parents therefore remain visible. JSON input
that fails parsing (including the bounded parser's depth limit) is omitted; it never
falls back to a raw-text scan that cannot decode its keys. `.json` files and text
beginning with a JSON object/array value are treated as JSON for this fail-closed policy. TOML section headers are not classified as JSON arrays. Other text is scanned linearly at
every `=` or `:` delimiter, not just the first assignment on a line. Unsupported or
unrecognized secret formats still require operator scope review. The policy
favors omission over trying to rewrite
source and then misrepresent its content digest. Adding a detector changes admitted
content and therefore packet identity. Preserve license files in the declared scope;
partial omission must not be interpreted as removal of a license obligation.

Output must have an existing parent outside the source checkout (resolved through
symlinks). It is validated, staged with owner-only permissions and atomically linked
to a new output name. Existing paths, including symlinks, are never overwritten.
An interruption can leave an owner-only redacted `.codefriend-*.pending` file; no
partial final artifact is published. This is artifact creation, not immutable
filesystem access control or a durable evidence store.

The command emits one JSON result on stdout and redacted diagnostics on stderr.
It retains the CLI's `ADL_OBSERVABILITY_STDERR=0` and `ADL_OBSERVABILITY_LOG` behavior;
logging records the fixed `codefriend` route, never source content or argument paths.
Errors are bounded codes, including `exact_revision_required`,
`repository_origin_mismatch`, `invalid_scope_path`, `byte_limit_exceeded`,
`symlink_input_rejected`, `source_changed_during_capture`, and
`packet_publish_failed_or_exists`. A failed command does not authorize retry with
wider scope or replacing an existing output.

## Production consumer boundary and qualification

`adl::codefriend::ingestion::AdmissionInput::read` is the shared production read
boundary for later evidence admission. It validates schema, closed fields, bounds,
path/scope membership, dispositions, content/digests and truthful completeness before
exposing `packet()`. The CLI reads its newly emitted artifact through this same
boundary; `packet read` exposes a usable installed inspection route. This does not
claim #881's durable admission/retention implementation.

`adl/tests/codefriend_ingestion.rs` contains executed local CLI conformance vectors:
relocation, dirty exclusion, missing/binary/unsafe inputs, byte/file limits,
traversal/symlink escape, revision/origin mismatch, output collision, tampered reader
input and private diagnostics. The production acquisition unit test injects origin
drift at the actual post-capture boundary. All are runtime PVF, deterministic local
Git/CPU/disk, no network/model, required #878 proof. GitHub/CI parity remains unproved
until those transports execute these obligations. OS support requires an actual run;
a local macOS pass alone is not Linux qualification.

The retained [installed proof](LOCAL_INGESTION_PROOF.json) records the local binary
and source digests, exact revisions, per-object digests and readback results for ADL
and the existing Vector checkout. The [Vector scope](fixtures/vector-scope.json)
is exactly the selected ten files, preserving crate MIT and root MPL-2.0 notices;
520,777 source bytes were captured without building or executing Vector. Raw external
source packets remain local artifacts, not additions to this repository.

Post-publication repairs for the multi-key credential and stale Git blob findings
are proved by `subsequent_nested_array_and_escaped_credentials_never_enter_packets`
and `git_sha1_and_sha256_blob_identity_is_verified_by_production_reader`. Both use
actual acquisition artifacts and the production library/CLI reader. The prior
installed proof remains unchanged; renewed proof is recorded separately in
`LOCAL_INGESTION_REPAIR_PROOF.json`.

The subsequent duplicate-parent review correction is proved by the same production
credential matrix, including overwritten escaped keys, excessive JSON nesting and
malformed JSON. Streaming scanning retains no JSON tree and keeps parser depth
limits. `LOCAL_INGESTION_STREAMING_PROOF.json` records the renewed installed proof;
the original and first-repair proofs remain unchanged.

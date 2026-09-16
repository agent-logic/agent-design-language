# CodeFriend production Memory Palace consumer

Issue #889 stores only admitted shared `BaselineRef` digest references through
`RuntimeMemoryPalaceService::commit`. Retrieval uses the shared ADL context
projection and a strict Runtime latest load. Missing or corrupt latest state is
rejected without journal recovery. Every retrieved reference is admitted again
through the live CodeFriend store; deletion cannot be undone by a saved Memory
Palace packet. The bounded reference/time trace is persisted under the Runtime
observability namespace inside the service root and its SHA-256 is checked again
on retrieval; it contains no source or finding prose. Comparisons preserve the #885 compatibility boundary.

The `baselines/<record_digest>.json` citation is a logical shared-record reference.
It is resolved through `BaselineRef` and `AdmittedBaselines::load`, which checks
the retained run identity and live admission; it is not a claim that the baseline
store has a file with that digest as its filename. Its citation SHA-256 binds the
compact serialized `BaselineRef` (field order `run_id`, `packet_id`,
`record_digest`); the shared CodeFriend IDs inside retain their original BLAKE3
semantics. Runtime trace and policy SHA-256 fields use SHA-256 as declared. The Runtime trace citation,
in contrast, names persisted bytes whose digest is checked on retrieval.

## Operator authority

`palace-index` requires two independently selected files: `--trust` is operator
configuration and `--authority` is untrusted signed evidence. Never accept the
trust file or public-key pins from a repository under review. The consumer does
not infer trust from an evidence digest, a status flag, or a matching key supplied
inside evidence.

The `codefriend.palace.trust.v1` trust object contains identity/private/continuity
public keys and key IDs, three expected generations, and an explicit relative
`runtime_state_dir`. The latter is resolved against the trust-file parent. The
production assembly initializes local adapter state there but is never started:
no adapters, listeners, providers, or time samples execute. The pinned bootstrap
uses the existing Runtime sanctuary/projection policy.

The `codefriend.palace.authority-evidence.v1` evidence contains the identity
record, signed identity binding and checkpoint, signed private-state record,
available projection, continuity record, and signed checkpoint manifests. Actual
`LiveAssembly::provision_memory_palace_authority` verifies them before returning
an opaque authority token. Runtime also exposes preparation through the same
pinned assembly: verify signed identity evidence to derive candidate references,
then build identity/continuity records and re-admit them through the ordinary
provisioner. This is needed to author durable records without exposing private
policy constructors or inventing verified tokens. The input caps are 16 KiB trust and 2 MiB evidence,
64 continuity manifests, and 32 projection fields. Errors omit input data and
paths. Symlink and parent-traversal inputs are rejected.

The Runtime preparation API preserves the caller's accepted private-state lineage.
It verifies on a cloned lineage and advances the caller only after all identity
and continuity checks succeed, including valid successor records.

Private-state lineage checking in the file-based CLI adapter is invocation-local. This route does not claim a
new durable private-state lineage service. The existing Runtime Memory Palace
service separately enforces identity and continuity succession across commits.

## Commands

Retain review records through the existing `memory retain` command first.
An index request has schema `codefriend.palace.v1`, `references`, explicit
`observed_epoch_ms`, positive `stale_after_ms`, and `max_working_set_items` in
1–64. It contains at most 64 distinct run references.

```sh
adl codefriend memory palace-index --store STORE --baselines BASELINES \
  --palace PALACE --trust TRUST.json --authority AUTHORITY.json --input INDEX.json
adl codefriend memory palace-compare --store STORE --baselines BASELINES \
  --palace PALACE --input RETRIEVE.json --out RESULT.json
```

The retrieve request supplies shared `baseline` and `current` references,
`expected_identity_root`, `expected_continuity_head`, the packet's explicit
`packet_observed_epoch_ms`, the reader's `observed_epoch_ms`, and the same
staleness and working-set bounds. Wrong identities, continuity, selected baseline,
staleness, citations, or unavailable admission fail closed. Incompatible or
partial admitted review records yield `not_comparable`, never a resolved claim.
Output is JSON on stdout; command events remain on stderr.

## Installed proof recipe

Create fresh fixture directories. The examples produce deterministic signed
fixtures with **public test signing seeds**, not operational credentials.
The fixture identity goes through the same public builders and production
provisioner as the consumer; no test-support authority constructor is used.

```sh
cargo run --offline --locked --manifest-path adl/Cargo.toml \
  --example codefriend_memory_fixture -- /absolute/fresh-memory-fixture
cargo run --offline --locked --manifest-path adl/Cargo.toml \
  --example codefriend_palace_fixture -- /absolute/fresh-authority-fixture
python3 adl/tools/codefriend_palace_installed_proof.py \
  --binary /absolute/installed/adl \
  --fixture-root /absolute/fresh-memory-fixture \
  --authority-root /absolute/fresh-authority-fixture
```

The binary must be installed outside Cargo target output with installer provenance.
The authority example emits `trust.json`, `authority-evidence.json`, and a public
`authority-summary.json`. The runner exercises separate CLI processes for first
commit, second-run retrieval, and second commit, plus denial and compatibility
cases. It writes `palace-installed-proof.json` only after its assertions pass.
The tracked PVF manifest classifies this as deterministic CPU/filesystem Runtime
acceptance. A recipe alone is not a passing execution receipt; current validation
results belong in the issue proof packet.

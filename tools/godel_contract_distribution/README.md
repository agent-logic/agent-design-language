# Frozen Gödel contracts 0.8.0

This ADL-owned data distribution supplies the four existing Gödel v0.8 JSON
schemas and three canonical examples needed by Runtime's installed demo proof.
It preserves exact source bytes from revision
`248f00e359e412f6bc0061ac88be9ff374e3a212`, including the original LICENSE.
It does not change schema semantics or example replay digests, export private
Runtime code, or reopen RD03/RD05 acceptance.

`artifact-lock.json` pins the archive and `source-manifest.json` digests. The
manifest records source repository, revision, Git blobs, byte counts and SHA256
for every data file. Producer acceptance is established by the reviewed merged
commit and its CI, separately from these self-descriptive source pins. Consumers
must pin that accepted commit and the lock/manifest digest through their own
trusted dependency decision; an arbitrary supplied manifest is not authority.

## Installed root contract

The archive extracts directly into a new destination, with no enclosing package
folder. `source-manifest.json`, `LICENSE`, and the seven original paths below
`adl-spec/schemas/v0.8/` and `adl-spec/examples/v0.8/` are relative to that root.
Runtime's consumer integration uses explicit `ADL_GODEL_CONTRACT_ROOT`; no
working-directory, sibling checkout, compile-time source directory or network
fallback is part of this interface. A consumer must verify the pinned manifest
and exact required file identities, refuse missing/tampered/aliased inputs, and
preserve existing schema validation and replay digest checks. This distribution
does not itself implement Runtime's lookup or claim runnable installed demos.

From the repository root:

```sh
python3 tools/godel_contract_distribution/package.py verify \
  --artifact tools/godel_contract_distribution/adl-godel-contracts-0.8.0.tar.gz \
  --extract "$NEW_INSTALL_ROOT"
python3 tools/godel_contract_distribution/package.py verify-root --root "$NEW_INSTALL_ROOT"
```

The extractor is create-only and checks the full archive before destination
writes. It does not activate anything. It assumes the caller owns the new
installation parent; concurrent hostile mutation of that parent is outside the
extractor contract. Do not use a shared writable extraction parent.

To reproduce from a Git repository containing the frozen source objects:

```sh
python3 tools/godel_contract_distribution/package.py export \
  --repository "$ADL_SOURCE_REPOSITORY" --output "$NEW_ARCHIVE_PATH"
```

This authenticates all source blob identities and produces byte-identical
archive output. The frozen objects need not be fetched by an installed consumer.

## Validation

```sh
python3 -B -m unittest discover -s tools/godel_contract_distribution -p 'test_*.py' -v
```

PVF: fast deterministic offline data-contract tests; small local temporary
storage; no services, credentials, downloads, Runtime execution or deployment.
The focused CI is a required proof of this distribution's bytes and refusal
behavior, not the separate RD08 installed Runtime acceptance.

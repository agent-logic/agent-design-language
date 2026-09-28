# ADL engine and records producer delivery

Status: local candidate only; accepted:false. This is separate from RD03's accepted five-package distribution and does not reopen issue1185. ADL remains implementation owner for both packages under RD02 D-GENERATION. Runtime owns only its adapter/consumer integration. No release or destination admission is performed by this proposal.

## Smallest producer change

Add tools/runtime_contracts/build_candidates.py. It builds exactly adl-engine0.92.1 and adl-records0.92.1 from a caller-supplied full immutable ADL revision, including license and per-file Git/SHA256 provenance. Normalize inherited package version/workspace and exact compiler/language sibling dependency versions. Retain every production source byte, including records signing, trust policy and replay verification. For the engine test package only, copy the exact19ADL-owned inert characterization fixtures and redirect its existing fixture-root path; preserve all assertions and fixture contents.

Invoke: python3 tools/runtime_contracts/build_candidates.py --repo <ADL-checkout> --revision <full-commit> --output <new-output-directory>. Existing output directories fail closed. No public registry publication, network download, installation or lifecycle action is performed by this builder.

## Consumer qualification and lock contract

Verify archive digest, exact inventory and provenance before extraction into a fresh source-isolated consumer root. Supply the unchanged accepted RD03 compiler/language artifacts at their exact versions/digests. Resolve consumer locks explicitly offline from frozen package locks, then execute cargo test --offline --locked --tests for both extracted package manifests. Record resolved lock hashes; source package locks alone are not proof of the normalized consumer resolution. The local v2 run passes28engine tests,15records tests and both fresh-process drivers. Zero-test library targets do not add to that count. OS-denied producer reads/network plus positive artifact reads were verified. Public dependency cache preparation was a separate non-qualification step.

PVF: deterministic local CPU/filesystem contract and offline installed-consumer integration; no paid provider/cloud; source/network denial required for isolation claim. Producer acceptance requires independent review and integration in the authoritative ADL issue/worktree; this local result is evidence for that work, not native lifecycle authority.

## Required producer acceptance

Bind exact source commit, immutable asset identity and SHA256, package versions, licenses, dependency/consumer locks, build provenance, supported Runtime adapter version and rollback to the prior accepted pins. Current local candidate IDs do not become accepted by renaming the manifest flag. Do not claim signatures: these archives are digest-pinned, unsigned candidates. Preserve ADL ownership and accepted RD03 bytes. Issue #1207 tracks producer integration under Sprint 1 #1180. The original thirteen RD acceptance items remain unchanged. Integration, artifact qualification, producer acceptance and Runtime adoption are separate evidence claims.

After producer acceptance, Runtime verifies the admitted IDs and updates its consumer lockset under its own lifecycle authority; rebuild/review the affected candidate if bytes change. Do not use this proposal to broaden the separate external-admission sixteen-scenario contract or start RD04 VM/native work.


## Rebuild at an immutable source revision

Run from the bound #1207 checkout after committing the builder and contract.
Python 3.11 or later is required for `tomllib`. Choose two new output paths;
the builder refuses either path if it already exists.

```sh
producer_revision=$(git rev-parse HEAD)
python3 tools/runtime_contracts/build_candidates.py --repo . --revision "$producer_revision" --output .csdlc/evidence/1207/build-a
python3 tools/runtime_contracts/build_candidates.py --repo . --revision "$producer_revision" --output .csdlc/evidence/1207/build-b
cmp .csdlc/evidence/1207/build-a/manifest.json .csdlc/evidence/1207/build-b/manifest.json
cmp .csdlc/evidence/1207/build-a/adl-engine-0.92.1-candidate.tar .csdlc/evidence/1207/build-b/adl-engine-0.92.1-candidate.tar
cmp .csdlc/evidence/1207/build-a/adl-records-0.92.1-candidate.tar .csdlc/evidence/1207/build-b/adl-records-0.92.1-candidate.tar
```

These archive names apply to the current 0.92.1 source workspace version;
use the explicit manifest names if that version changes. A nonzero command
exit blocks qualification. Equality proves reproducibility at this revision,
not acceptance. Retain both manifests and exact archive hashes.

## Consumer inputs and evidence

The qualification operator supplies a fresh consumer directory, the two
verified candidate archives, the unchanged accepted RD03 package manifest and
assets, a prepared public Cargo dependency cache, and an OS sandbox policy.
Extract only after checking exact members, safe regular-file paths and hashes.
Lay out the engine, records, compiler and language package directories as
siblings under `vendor`; no sibling producer checkout is an input.

For each of `vendor/adl-engine/Cargo.toml` and
`vendor/adl-records/Cargo.toml`, execute the following through the OS sandbox:

```sh
cargo metadata --offline --manifest-path vendor/adl-engine/Cargo.toml --format-version 1
cargo test --offline --locked --manifest-path vendor/adl-engine/Cargo.toml --tests
cargo metadata --offline --manifest-path vendor/adl-records/Cargo.toml --format-version 1
cargo test --offline --locked --manifest-path vendor/adl-records/Cargo.toml --tests
```

These Cargo commands alone do not establish source or network isolation.
Retain the actual sandbox policy, effective nonsensitive environment, argv,
exit codes, stdout/stderr, successful artifact-read probe, denied producer-read
probe and denied network probe. Prepare public dependency caches separately,
then prohibit network for resolution and tests. Record the resolved lock files
and hashes before claiming locked consumer proof. Confirm metadata lists only
admitted vendor directories and public registry dependencies.

The current source baseline is 28 engine assertions, 15 records assertions,
and two executed fresh-process drivers. Identify actual test target names and
counts from logs; do not count zero-test library targets. New source revisions
require fresh evidence and an explicit explanation of any denominator change.

## Acceptance and rollback boundary

The issue-local acceptance record must name the committed producer revision,
archive manifest and digests, unchanged RD03 asset pins, resolved consumer
locks, exact Runtime adapter source revision, source-preservation comparison,
isolation results, tests and independent review. `accepted:false` in builder
output remains truthful: the builder cannot grant lifecycle acceptance.

Before any later Runtime adoption, retain the complete previous Runtime
candidate manifest and its exact artifact pins. If qualification fails, keep
those pins; no Runtime switch occurs in #1207. Consumer rollout and rollback
are governed by Runtime's own admission and state-compatibility evidence.

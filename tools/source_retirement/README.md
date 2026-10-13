# Issue 1223 source retirement guard

The immutable original register preserves 4,134 ordered identities. The current
combined ledger classifies those same rows and 22 additional paths separately.
Conditional proposals never authorize deletion. The executed boundary is 34
original removals, 36 approved component removals, and one public-cutover removal:
`RD07-2113 adl/tools/test_select_validation_lanes.sh` (71 total).

`check.py` reads committed Git objects, including approval packets. It verifies
original identities, combined counts, disjoint held groups, exact executed deletion
scope, protected/historical evidence, rollback custody and direct references.
Dirty files cannot override committed approval inputs.

## Reviewed execution reconciliation

`EXECUTION_OVERLAY.json` records 77 existing retained-path changes through 112
immutable Git transitions, independently reviewed against baseline and final
identities, 25 packet references and rollback custody. Its 38 original public
cutover entries preserve actual applied hashes, separately from prepared hashes.
The completed manifest is pinned by `EXECUTION_APPROVAL_SHA256` in the guard.

The overlay admits only these existing reviewed changes. Conditional proposals
remain unexecuted and confer no new deletion authority. Later changes require
new exact source, review and custody evidence before updating the immutable pin.
Original approvals remain unchanged.

The exact-identity validator rejects unapproved deletion, retained source/mode
drift, duplicate paths, stale before-identities and absent provenance/custody.
The original baseline-specific validator remains available as historical logic;
the current run uses the combined ledger and execution overlay instead.

## Focused validation

Run `python3 -B tools/source_retirement/test_check.py` for deterministic Python
contract/negative tests (small CPU/disk, no build/network). Run
`python3 -B tools/source_retirement/check.py` only against the committed candidate;
an incomplete or changed execution manifest fails closed.
Tests do not establish installed product qualification or deletion authorization.

Provision the historical rollback archive in the resolved Git common directory
at `csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz`, SHA256
`12205043566913435c25ea414b1fd9148b46727821f7bfefd5d452cfe47a1c17`.
Missing or corrupt custody fails closed. Digest checks are separate from restore
rehearsal. The extra cutover deletion also needs its exact pre-cutover Git blob
preserved; the old archive is not presumed to contain newer bytes.

Reference checks retain the three original component boundaries and exact
historical allowlists. Operational references cannot be hidden by a blanket docs
exception. Dynamic/indirect callers still require independent review. Further
historical reference additions must be reviewed individually before final proof.

## Proof storage

The installed proof honors `TMPDIR` for its fresh scratch and Cargo target.
Set it to an existing directory on the selected build volume. Evidence defaults
to this worktree's ignored `target/public-adl-proof/<issue>/<run>`;
`ADL_PUBLIC_ADL_PROOF_ROOT` can select a different evidence root. Neither path
is derived from the linked worktree's common Git directory.

The scratch exception in the macOS sandbox permits only the fresh proof
directory, excluding the synthetic credential sentinel, and literal ancestor
metadata needed by Cargo dep-info resolution. Sibling file contents, producer checkout and
network access remain denied. Run the seven small storage/sandbox regressions with
`python3 -B tools/public_adl/test_install_storage.py` under the same `TMPDIR`.
These are PVF `local_contract` path-isolation tests (small CPU/disk, no build or
external service); the five OS sandbox cases require macOS. They do not replace
the final five-package, seven-consumer, 21-UTS, five-negative proof.

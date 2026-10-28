# Issue 872 copied-record conversion rehearsal proof contract

This directory defines the stable, reviewable proof contract for issue #872.
It is not execution evidence and does not authorize live conversion, writer
activation, shared-binary replacement, or restore of operational records.

The rehearsal runs only through the issue-scoped
`csdlc-conversion-rehearsal --request <JSON>` executable. The request uses
schema `csdlc.v3.copied_record_conversion_rehearsal_request.v1` and names an
isolated Git primary checkout, a genuine registered linked worktree, copied
source snapshot, retained old and candidate executables, isolated output root,
the seven required role records, deterministic fake authenticated transport,
and optionally one `{point,boundary}` fault injection.

Success produces one JSON result on stdout. Human `adl_event` records use
stderr. All durable evidence stays under the caller's isolated output root;
the operational issue store and shared owner-binary directory are outside the
allowed write boundary.

The complete run must retain exactly seven distinct lifecycle roles, twelve
named scenarios, and both sides of all fifteen durability boundaries. A
machine-derived summary cannot replace the underlying scenario results,
invent a passing disposition, or omit a file from `manifest.json`.

The retained root `request.json` binds the isolated paths, exact ordered role
census, and old-writer command. Every scenario retains `commands.jsonl`,
before/crash/after inventory hashes, hash-bound stdout and stderr JSONL,
human-readable stderr, and its result. Every fault boundary additionally
retains its journal and source/effect state; remote boundaries retain their
exact fake-transport ledger. `snapshots/source/<issue>/` retains the source
bytes behind the census, and binary provenance includes a non-empty identity
for both executables.

Validate retained evidence with:

```bash
python3 adl/tools/validate_issue872_conversion_rehearsal.py \
  --evidence-root .csdlc/evidence/872/conversion-rehearsal
```

The validator independently checks:

- complete classified source-file union and per-role semantic/evidence identity;
- old-binary pre-fence control plus byte-preserving rejection during conversion and after activation;
- one-operation fake remote reconciliation with no second dispatch;
- installed status/validate parity from the isolated primary and linked worktree;
- exact pre-effect source/executable restore;
- refusal after a new-format local or fake remote effect while preserving evidence;
- 30 restart cases with no lost artifacts or duplicate effects; and
- a SHA-256 manifest covering every retained file other than the root manifest
  itself, including any nested file named `manifest.json`.

The PVF lane is deterministic local CPU integration. It uses bounded temporary
disk, local Git, copied records, retained executables, controlled faults, and a
fake transport. It uses no provider, cloud, paid, or live repository mutation.
It is a mandatory SIM-07 input and grants no release or activation authority.

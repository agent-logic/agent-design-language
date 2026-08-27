# DEC-01 Runtime v2/v3 Authority Separation Design

Issue #513 accepts one bounded authority topology for Runtime v2 and Runtime v3. The deliverable is an executable, repository-local contract that inventories the current source and reverse-reference denominator, assigns every observed reference to an owner/disposition, proves supported compatibility behavior, and supplies executable migration and rollback checks. Runtime v4 remains out of scope.

## Authority Topology

- Runtime v2 owns `adl/src/runtime_v2/**` and compatibility-facing documentation for legacy Runtime v2 behavior.
- Runtime v3 owns `adl-runtime/**` and `adl-runtime-kernel/**`.
- Shared documentation under `docs/runtime/**` records the topology and transition contract but does not move runtime authority by itself.
- Evidence under `docs/milestones/v0.92.1/evidence/runtime-decoupling/**` is DEC-01 proof material and does not create Runtime v4 scope.

## Implementation Shape

The issue will add:

- `docs/runtime/runtime-v2-v3-authority-topology.md` as the human-readable authority topology and migration/rollback contract.
- `docs/milestones/v0.92.1/evidence/runtime-decoupling/runtime-authority-topology.json` as the machine-readable ownership/disposition manifest.
- `docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh` as the executable proof for source denominator, reverse-reference census, compatibility, rollback, migration, and Runtime v4 exclusion.

The validator intentionally checks repository state instead of embedding a one-time transcript. It compares the declared manifest against the live tree, checks all declared source roots exist, verifies reverse-reference coverage for the current runtime v2/v3 references, and proves the migration/rollback commands remain executable dry-run contracts.

## Non-Goals

- No Runtime v4 implementation or planning expansion.
- No Runtime v2 deletion.
- No Runtime v3 default cutover.
- No behavioral rewrite of runtime code beyond what is needed to prove the topology.

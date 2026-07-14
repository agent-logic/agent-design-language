# v0.91.8 Demo Matrix

## Status

Planned. Commands and evidence paths are contracts for later WPs, not current proof.

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers
- Related issues / work packages: WP-03, WP-10 through WP-20

## Purpose

Define the reviewer-visible demonstrations for behavioral parity, product
independence, rollback, and source deletion.

## How To Use

Replace command stubs with stable owner-binary commands during implementation.
No row becomes READY until retained evidence exists.

## Scope

In scope:
- deterministic parse/compile comparison;
- bounded mock execution and Runtime v3 integration;
- selector rollback and deletion proof.

Out of scope:
- live production providers;
- v0.92 birthday behavior.

## Runtime Preconditions

Working directory:

```bash
cd /path/to/agent-design-language
```

Deterministic runtime/provider assumptions:

```bash
export ADL_PROVIDER=mock
```

Additional requirements: exact revisions and fixture digests must be recorded.

## Related Docs

- [Design](DESIGN_v0.91.8.md)
- [WBS](WBS_v0.91.8.md)
- [Sprint](SPRINT_v0.91.8.md)
- [Quality gate](QUALITY_GATE_v0.91.8.md)

## Demo Coverage Summary

| Demo ID | Demo title | Claim / WP | Command entry point | Proof surface | Success signal | Determinism note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Document and plan parity | WP-03/WP-11 | `adl-v2 parity plan adl-v2/fixtures/corpus.json` | parity packet | no unclassified mismatch | repeated canonical equality | PLANNED |
| D2 | Bounded mock execution | WP-06/WP-11 | `adl-v2 run --provider mock adl-v2/fixtures/fork-join.yaml` | event/result bundle | stable order and result | captured inputs only | PLANNED |
| D3 | Runtime v3 consumption | WP-08/WP-14 | `adl-runtime-kernel ...` | integration packet | plan admitted and deployed outcome retained | nondeterminism captured as events | PLANNED |
| D4 | Selector rollback | WP-12 | `adl-v2 select --prove-rollback` | selector transaction | v2 default and v1 restore both pass | exact revisions | PLANNED |
| D5 | Deletion gate | WP-13 | `adl-v2 proof deletion` | deletion manifest | deletion >=80% | pinned denominator | PLANNED |
| D6 | Three-product deployed lifecycle | WP-14/WP-15 | owner-binary acceptance and integrated demo commands | deployment and demo packets | ADL v2, Runtime v3, and C-SDLC v2 complete the accepted lifecycle | exact installed revisions | PLANNED |

## Coverage Rules

- Every command emits machine-readable output and retained evidence.
- Live credentials are unnecessary for the required demo set.
- Formatting differences do not pass as behavioral mismatches without classification.

## Demo Details

### D1 - Document and plan parity

Runs positive and negative corpus cases through both generations and compares
normalized documents, plans, errors, and stable node IDs.

### D2 - Bounded mock execution

Runs sequential, fork/join, retry, failure, and resume fixtures with captured
mock outcomes and compares canonical event streams.

### D3 - Runtime v3 consumption

Proves the runtime adapter can admit and supervise a plan without moving
runtime lifecycle authority into ADL core.

### D4 - Selector rollback

Proves install, explicit generation selection, v2 default, and v1 restoration.

### D5 - Deletion gate

Recomputes the pinned denominator and reports deleted, retained, and newly
added code separately.

## Cross-Demo Validation

D1 and D2 gate D3. D3 and full parity gate D4. D4 gates D5.

## Determinism Evidence

Each deterministic demo runs at least twice and records canonical digest
equality. External values are captured before influencing engine state.

## Reviewer Sign-Off Surface

WP-18 and WP-19 record exact revisions, commands, findings, dispositions,
non-claims, and residual risks for every demo; WP-20 closes accepted findings.

## Exit Criteria

- Every row is READY or explicitly blocked with approved evidence.
- D4 through D6 are required for release.

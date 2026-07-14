# v0.91.8 Quality Gate

## Status

Planned; no gate result exists.

## Required Gates

| Gate | Required result | Evidence owner |
|---|---|---|
| Baseline | exact revision, hashed lists, implementation/test split | WP-02 |
| Characterization | positive/negative corpus and repeatability | WP-03 |
| Core validation | fmt, check, clippy, tests, schema fixtures | WP-04 to WP-07 |
| Dependency budget | default core excludes runtime/control-plane/cloud graphs | WP-10 |
| Size budget | implementation <=30k and tests <=15k LoC | WP-10/WP-16 |
| Validation latency | warm <=2m; deterministic full <=10m or approved evidence-backed revision | WP-16 |
| Shadow parity | no unclassified mismatch | WP-11 |
| Soak and rollback | representative scenarios pass and v1 restoration works | WP-12 |
| Default switch | exact reviewed selector transaction | WP-12 |
| Deletion | >=80% of pinned denominator | WP-13 |
| ADL deployment | stable installed default, operations, rollback and consumer proof | WP-14 |
| Runtime deployment | approved topology, readiness, operations, recovery and consumer proof | WP-14 |
| C-SDLC deployment | stable binaries/skills and full init-to-closeout proof | WP-14 |
| Unity/Adaptive learning | moved issues closed or evidence-backed approved blockers | WP-14 |
| Review | no unresolved critical/high actionable finding | WP-18 through WP-20 |
| Handoff | v0.92 accepts exact contracts and risks after next-milestone review | WP-21 through WP-22 |

## Deletion Accounting

- Pin legacy files and line counts at WP-02 revision.
- Report implementation, tests, scripts, generated code, and new v2 code separately.
- New or moved code cannot improve the legacy deletion percentage.
- Retained code requires path, owner, capability, justification, and disposition.
- Below 80% is failure; 80-89% requires an explicit retained-surface review;
  90% or above satisfies the target.

## Proof Boundaries

Local focused proof, CI integration proof, provider/live proof, and release proof
must be labeled separately. Planning or mock-provider proof cannot establish a
live-provider claim.

## Exit Criteria

All required rows pass at the release revision and are referenced by SOR and
release evidence.

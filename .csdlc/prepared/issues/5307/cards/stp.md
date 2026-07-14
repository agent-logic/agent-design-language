# Structured Task Prompt

Template: 1.0.0

Issue: 5307

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Remove only the exact importer surface after time, approval, migration, contract, and health gates pass.

## Deliverables

- Machine-readable importer sunset decision
- No-active-contract proof
- Durable migration evidence
- Post-removal v2 proof

## Acceptance

1. Trusted time is at or after the not-before timestamp
2. Explicit sunset approval is current
3. Migration evidence is complete and durable
4. No active contract requires the importer
5. Exact-revision review and required checks pass

## Dependencies

- #5305 merged
- Not-before date
- Explicit sunset approval
- Completed migration and compatibility evidence

## Inputs

- docs/architecture/csdlc-v2/gate8/GATE8_IMPORT_SHADOW_DESIGN.md
- docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json

## Non Goals

- Unrelated v1 deletion
- ADL or Runtime cleanup

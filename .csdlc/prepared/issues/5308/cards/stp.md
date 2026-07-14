# Structured Task Prompt

Template: 1.0.0

Issue: 5308

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Remove only the exact reviewed rollback inventory after all typed gates pass.

## Deliverables

- Machine-readable sunset decision
- Exact rollback removal manifest
- Post-removal v2 and importer-presence proof

## Acceptance

1. Trusted time is at or after the not-before timestamp
2. Explicit sunset approval is current
3. Current v2 proof is green
4. Importer remains untouched
5. Exact-revision review and required checks pass

## Dependencies

- #5305 merged
- Any approved #5306 topology
- Not-before date
- Explicit sunset approval

## Inputs

- docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json
- docs/architecture/csdlc-v2/gate10d1/ELIGIBILITY_EVIDENCE.json

## Non Goals

- Importer removal
- Unrelated deletion or cleanup

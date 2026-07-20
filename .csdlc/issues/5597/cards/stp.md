# Structured Task Prompt

Template: 1.0.0

Issue: 5597

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the grouped C-SDLC v2 authority repair required by issue #5597.

## Deliverables

- generation-aware registry contract
- lossless typed input and migration
- preparation-safe SIP, STP, and SRP operations
- review-assignment synchronization
- focused regression proof

## Acceptance

1. AC-1: Registry distinguishes legacy/import 1.0.3 from native compact v2 1.0.0 without false provenance
2. AC-2: Initialization validates registry and compact shape before writing and fails closed on incompatible truth
3. AC-3: Existing native 1.0.0 records remain readable and legacy 1.0.3 import remains separate
4. AC-4: Bootstrap and migration preserve explicit SIP operator constraints and SRP review scope including explicit none
5. AC-5: Typed SIP operator-constraint replacement is preparation-safe in initialized, ready, and bound phases
6. AC-6: Typed STP acceptance replacement atomically revalidates SPP step and VPP proof-role coverage
7. AC-7: Bound SRP replanning is preparation-safe and review assignment synchronizes authoritative scope immediately
8. AC-8: Focused Gate 2, identity, review, migration, and Gate 9 parity tests plus strict lint and owner validation pass
9. AC-9: Exact-revision review has no unresolved actionable finding and blocked consumer packets can use typed repair

## Dependencies

- sprint umbrella #5595
- blocked consumers #5336, #5337, #5358, and #5361

## Inputs

- docs/templates/prompts/current.json
- csdlc-v2/src
- csdlc-v2/tests
- issue #5597

## Non Goals

- product or runtime implementation
- bulk historical migration
- template-family relabeling
- AWS
- raw gh
- new issues

# Structured Task Prompt

Template: 1.0.0

Issue: 217

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Restore and validate only #209's retained native proof contract and typed proof truth; do not change runtime production behavior or broaden Sprint 4 scope.

## Deliverables

- Exact restored ten-file historical packet beneath .csdlc/evidence/209
- .csdlc/prepared/issues/217/historical-c640-denominator.json
- .csdlc/prepared/issues/217/protected-source-denominator.json
- .csdlc/prepared/issues/217/proof-contract-paths.json
- .csdlc/prepared/issues/217/h2-retention-allowlist.json
- Issue-owned detached-c640 historical packet verification wrapper
- Issue-owned fresh native receipt producer
- Issue-owned merge-safe retained native proof and retention-chain validator
- Narrow issue-owned GitHub Actions workflow for fresh current-head native proof
- Fresh exact ten-file native packet plus machine-consumed denominator beneath .csdlc/evidence/217
- .csdlc/evidence/217/h2-retained-surface-manifest.json with exact retained path digests and no self-binding H2 commit field
- .csdlc/evidence/217/h2-retention-review-receipt.json binding H, H2, the retained-surface manifest digest, exact diff, and independent review identity
- .csdlc/issues/217 complete typed lifecycle and review evidence

## Acceptance

1. AC-1: The exact historical ten-path set from #209 run 31453636709 is restored unchanged and matches the machine-consumed historical denominator byte-for-byte; detached c640 validation passes with the original GitHub environment and remains provenance-only.
2. AC-2: A fresh workflow run at reviewed producer head H produces a separate exact ten-file #217 Linux/macOS packet; absence of fresh proof fails closed unless a later reviewed design records explicit operator rebaseline approval.
3. AC-3: H2 differs from H only by the current evidence denominator, its exact ten evidence paths, the H2 retained-surface manifest, and explicitly named #217 lifecycle paths; no other source, proof tool, workflow, design, or repository path changes.
4. AC-4: The producer, historical wrapper, retained validator, native workflow, exact seventeen-path source denominator, exact eight-path proof-contract denominator, and exact fourteen-path H2 lifecycle allowlist are byte-identical from H through reviewed H2.
5. AC-5: The H2 manifest records the exact retained path set and digests for the current denominator, its ten evidence paths, and all eight proof-contract paths; it excludes itself to avoid self-digest recursion, while its own digest is bound by the later independent-review receipt.
6. AC-6: Independent review of exact H2 produces a receipt introduced at H3; the validator finds the unique first commit on current-head ancestry that adds the receipt path, retains that H3 or squash/integration anchor object, reads its exact receipt blob, and requires current receipt bytes/blob identity to match before trusting its H, H2, diff, manifest, denominator, proof, reviewer, scope, result, and no-drift bindings.
7. AC-7: Only H2 commit/tree objects may be unavailable. H2-to-H3 changes are limited to the anchored review receipt and named #217 lifecycle paths; later heads may evolve unrelated paths but must retain the ancestral receipt anchor and cannot change the receipt, retained surface, proof paths, evidence, or protected seventeen-path source set. Coherent receipt-plus-manifest rewrite and missing, ambiguous, or non-ancestral anchor cases fail closed.
8. AC-8: Typed #217 VPP/SOR execute the historical ten-file, protected seventeen-path, proof eight-path, and lifecycle fourteen-path preparation contracts and name the fresh retained/receipt validator as terminal proof; fresh exact-head implementation, H2, and receipt reviews have no unresolved findings, no implementation/publication occurs before the second full-package review passes, and the visible green PR links #209/PR #215 and #142 while remaining unmerged pending operator review.

## Dependencies

- Closed issue #209 and merged PR #215
- Source revision c640066f284a915b638add377cc4b0a2e221e6f9 and successful run 31453636709
- Merge revision a77519c3fca9f64752af41c9a2ebd396468891f7
- Issue #142 remains blocked by this follow-on

## Inputs

- .csdlc/prepared/issues/209/produce-native-receipt.rb
- .csdlc/prepared/issues/209/validate-native-receipts.rb
- .csdlc/evidence/209 from commit b27b61597b7e6bc6563d6a7fef6f13ec9c6d3e98
- .github/workflows/wp14-production-acip-repair.yml
- .csdlc/issues/209/cards/vpp.values.json
- .csdlc/issues/209/cards/sor.values.json

## Non Goals

- Production Rust, ACIP, replay, API, Guardian, kernel, or runtime behavior changes
- Implicit or unreviewed rebaseline of legitimate post-c640 protected-source drift
- AWS, cloud provisioning, or broader distributed-runtime execution
- Modification of terminal #209 cards, derived terminal state, or historical artifact bytes
- Implementation or publication before the second independent full-package review passes
- Merging before operator review or closing #142

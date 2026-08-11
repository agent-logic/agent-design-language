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
- Reviewed H2 retention receipt binding producer head H, evidence-only H2, exact diffs, denominators, and review identity
- .csdlc/issues/217 complete typed lifecycle and review evidence

## Acceptance

1. AC-1: The exact historical ten-path set from #209 run 31453636709 is restored unchanged and matches the machine-consumed historical denominator byte-for-byte; detached c640 validation passes with the original GitHub environment and remains provenance-only.
2. AC-2: A fresh workflow run at reviewed producer head H produces a separate exact ten-file #217 Linux/macOS packet; absence of fresh proof fails closed unless a later reviewed design records explicit operator rebaseline approval.
3. AC-3: Evidence-only H2 differs from H only by the current evidence denominator, its exact ten evidence paths, and explicitly named #217 lifecycle paths; no other source, proof tool, workflow, design, or repository path changes.
4. AC-4: The producer, historical wrapper, retained validator, native workflow, exact seventeen-path source denominator, proof-contract path denominator, and H2 allowlist are byte-identical from H through reviewed H2.
5. AC-5: The retained validator verifies exact evidence/source/proof-contract denominators, all internal digests and provenance, complete semantics, and H-to-H2 diff status; missing, extra, duplicate, tampered, unprotected-source, proof-tool, workflow, protected-source, semantic, provenance, and unrelated drift fail closed.
6. AC-6: Independent review of exact H2 produces a later retained receipt binding H, H2, both tree identities, exact changed-path/status digest, denominators, proof-contract digests, reviewer identity/result/scope, and no-drift result; later heads anchor that reviewed-H2 receipt through ancestry or complete retained-tree equivalence.
7. AC-7: Typed #217 VPP/SOR execute both preparation denominators, name the fresh retained/receipt validator as terminal proof, distinguish c640 as provenance-only, and no implementation/publication occurs before the second independent full-package review passes.
8. AC-8: Fresh exact-head implementation, H2, and retained-receipt reviews have no unresolved actionable findings; the visible qualified PR links #209/PR #215 and #142, stays unmerged pending operator review, and is green before terminal delivery.

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

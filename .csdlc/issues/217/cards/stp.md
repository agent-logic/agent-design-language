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
- .csdlc/prepared/issues/217/verify-historical-c640-packet.rb
- .csdlc/prepared/issues/217/produce-native-receipt.rb
- .csdlc/prepared/issues/217/validate-retained-native-proof.rb
- Narrow issue-owned GitHub Actions workflow for fresh current-head native proof
- Fresh exact ten-file native packet plus machine-consumed denominator beneath .csdlc/evidence/217
- .csdlc/issues/217 complete typed lifecycle and review evidence

## Acceptance

1. AC-1: The exact eight platform files plus native-validation-manifest.json and native-receipts-validation.log from #209 run 31453636709 are retained with their original verified digests.
2. AC-2: The existing source-run validator still authenticates the source packet at c640066f284a915b638add377cc4b0a2e221e6f9.
3. AC-3: The retained validator verifies every packet digest, runner/job provenance field, source manifest, semantic projection, assertion inventory, and path-hygiene rule.
4. AC-4: A later final head passes only through proved source ancestry or protected-tree digest equivalence, and both modes reject any current protected-source drift.
5. AC-5: Focused regressions reject missing files, digest tampering, runner/source mismatch, semantic mismatch, protected-source drift, and unrelated non-equivalent source history while accepting squash-equivalent history.
6. AC-6: VPP and SOR name the retained-proof validator as the final-head command and that command passes on the reviewed PR head.
7. AC-7: Fresh independent design and exact-head implementation reviews have no unresolved actionable findings.
8. AC-8: A visible qualified PR links #209/PR #215 and blocker #142, remains unmerged until operator review, and is green before terminal delivery.

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

- Production Rust, ACIP, replay, API, Guardian, kernel, or runtime changes
- A new native run unless the retained source evidence cannot be authenticated
- AWS, cloud provisioning, or broader distributed-runtime execution
- Rewriting #209 history or silently treating its missing retained proof as passed
- Merging before operator review or closing #142

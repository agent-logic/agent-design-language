# Issue #849 merge-linkage correction mapping

This is the bounded v0.92.2 correction mapping for MERGE-LINKAGE-001 observed
at `25498d7709cb5fe13242aac81988ecdb344db968` in #844/PR #847. It does not
rewrite #835/#844 output or replace historical failed #522/#833 release evidence.
Review acceptance and exact local/CI outcomes belong to #849 SRP/SOR and its PR.

| Existing criterion | Corrected candidate evidence | Boundary |
| --- | --- | --- |
| V3-A:retained-161-ac-8 | `merge_linkage_review_digest_prevents_missing_or_changed_review_linkage`; qualified `PublicationLinkage` in typed review digest and durable intent | Native exact-review admission; no new signature authority |
| V3-E:retained-175-ac-10 | `merge_linkage_same_head_drift_before_dispatch_and_after_uncertain_result`; `merge_linkage_negative_matrix_rejects_before_intent_and_dispatch` | No PUT for rejected preflight; head CAS does not freeze body/base/policy |
| V3-E:retained-175-ac-11 | `merge_linkage_positive_modes_and_qualified_split_repository_replay`; existing uncertain transport/target guard cases | Authenticated linkage/state reconciliation; no automatic retry |
| V3-E:retained-177-ac-3 | Positive Closing CLOSED and PartOf OPEN observations plus existing finish observation test | Merge receipt is not native terminal authority; finish still independently observes |
| V3-E:V3-E-ac-2 | Same bounded native merge/linkage coverage above | Qualified extension only; no blanket release acceptance |

Tests live in `csdlc-v3/src/commands/remote/tests/merge_cases.rs`; the constrained
GraphQL transport regression is in `csdlc-v3/src/adapters/mod.rs`. PVF: required
native owner contract, deterministic fake authenticated transport plus local Git,
small CPU/filesystem, no external service or paid resource, no live merge.
The complete native owner all-target suite, clippy and formatting are required
for this candidate. Logs and actual results are recorded by #849. Current source
adds behavior and tests; it makes no Rust size-reduction claim.

The operator contract is [PULL_REQUEST_MERGE.md](../../../csdlc-v3/PULL_REQUEST_MERGE.md).
GitHub's [GraphQL PR schema](https://docs.github.com/en/graphql/reference/pulls)
provides body and closing references; the separately qualified issue observation
provides its current state. All pages must be complete. A later remote race or
lagging/changed issue state blocks reconciliation rather than manufacturing success.

# v0.92.1 documentation review and external-review handoff — #518

This packet delivers the documentation review of the merged TAIL-01 candidate.
**Release acceptance remains BLOCKED.** Documentation-review completion is not
release approval and supplies no missing runtime, cloud, or review evidence.

## Candidate and review identity

Predecessor #517 / PR #748 merged as
`e734438b5fdba099ed607e0604254430ddd7d091` after independent review at
`09e120c0496441215b33a6ccd51c07f2beb98dc1`. This branch includes that merge and the subsequent terminal reconciliation
PR #750 (`f61deb36d5eccb4a3e391510bbea4bb6f2d51216`). The latter adds terminal
receipts for 11 issues without changing the recorded quality evaluation.
The operator explicitly authorized #518 to proceed with the blocked quality
decision recorded as a finding. No dependency on a passing *release* decision
is imposed on completing this documentation handoff.

[handoff-content.json](handoff-content.json) binds the reviewed documentation
paths to SHA-256 hashes and identifies the merged source baseline. The final
independent review and exact reviewed commit are recorded in the typed SRP at
`.csdlc/issues/518/cards/srp.md`. Verify both content hashes and typed review
before accepting the publication revision; a content inventory alone is not
review approval.

## Start here

1. Read the [canonical inventory](../../../CANONICAL_DOC_INVENTORY_v0.92.1.md)
   and [quality decision](../../../QUALITY_GATE_v0.92.1.md).
2. Inspect [dependency-observation.json](dependency-observation.json) for the
   merged predecessor identity and all five unresolved quality exceptions.
3. Inspect [finding-dispositions.json](finding-dispositions.json) for the 15
   documentation findings, corrections and explicitly retained limitations.
4. Use [creation-map.json](creation-map.json) for the exact 45 child issues from
   WP-01/#480. Creation and closure do not prove feature acceptance.
5. Read [the complete Cargo audit](CARGO_MANIFEST_REVIEW.md) for all 24 tracked
   manifests and 20 local dependency references. No package version was changed.

## Evidence denominators and limits

The initial [737-document inventory](document-inventory.json) remains an immutable
source-time audit at `bf617859982f8b9613737344400626eb90768930`. All selected files
were read and statically screened; this did not verify every historical sentence
or runtime behavior. The current content inventory includes the original set,
new README/AGENTS files, and new milestone documentation introduced by the merge.
Historical records preserve their source-time meaning.

[source-proof-snapshot.json](source-proof-snapshot.json) preserves the integration
diagnostic's 35 execution rows, source/merge revisions, review status and evidence
paths at candidate `bf159eb416950dfa3399933829726a7b7e71f897`. It is not current
product acceptance. Its zero-backlog count does not override the explicit #84
Unity and #251 TLS deferrals. #122 public exposure retains separate ownership.

The merged quality evaluation inventories 393 rows and evaluates 366 required
lanes: 121 passing, 245 non-proving, zero absent and zero demonstrated product
failures. Its five exceptions remain visible: exact-head review gaps; live/spec
synchronization; current criterion proof gaps; retained predecessor proof
crosswalks; and shared-path owner sign-off. Affected row counts overlap and are
not a separate issue denominator. These are release findings, not facts repaired
by this documentation task.

## Reviewer responsibilities and downstream ownership

Check that claimed features have criterion-level evidence, that current docs agree
with explicit deferrals, and that historical diagnostics are not promoted to
release approval. Use the recorded exact source references rather than issue
closure as acceptance proof. Coverage, Rust-module and gap/risk tracker freshness,
end-of-milestone reporting and later review dispositions remain explicit release
closeout obligations; inventorying them does not mark them complete.

#519 owns publication finalization and release-version decisions. #523's merged
successor planning owns the v0.92.2 SIM program; this packet claims no SIM
implementation. Product and evidence owners retain the five quality exceptions.

## Reproduction

- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --all`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --final`
- `python3 .csdlc/prepared/issues/518/audit-cargo-manifests.py --check`

These local checks verify inventories/hashes, extracted relative links, snapshot
parity and merged predecessor ancestry. They do not prove every bare reference,
HTTP reachability, compilation, vulnerability posture, full lockfile
reproducibility or release readiness. Typed independent review remains separate.

Earlier local-validation and independent-review records are historical snapshots
bound to the commits identified by their addenda; they do not claim current-file
hashes after this refresh.

The operator authorized the retained v2 route for #518 after the native binding
regression. That bounded exception does not change the repository's default v3
authority or bypass typed review, publication and finish guards.

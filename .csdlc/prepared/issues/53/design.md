# Issue 53 Design: Non-self-referential execution-proof receipts

## Problem

The WP-04 proof contract currently requires `source_revision` to equal repository `HEAD`. A tracked receipt written for substantive commit A can only be committed in later evidence commit B, so exact equality makes the receipt recursively stale.

## Design

Execution proof schema `adl.wp04.execution_proof.v3` separates two identities without storing the evidence commit inside the commit that creates it:

- `source_revision`: the exact substantive commit whose code and proving commands produced the receipt.
- derived evidence revision: the unique Git commit that first introduces the receipt path after `source_revision`.

The receipt declares `evidence_revision_strategy: derive_from_receipt_introduction`. The validator derives the evidence revision from Git history and reports it; a stored `evidence_revision` field is rejected because embedding B in content committed by B would recreate the same self-reference.

The validator resolves both commits and fails closed unless:

1. `source_revision` resolves exactly to a full Git commit ID.
2. The receipt path has exactly one introduction commit after `source_revision`; that commit is the evidence revision.
3. `source_revision` is an ancestor of the derived evidence revision.
4. The derived evidence revision is an ancestor of current `HEAD`, allowing later metadata-only lifecycle commits without rewriting the receipt.
5. Every changed path between source and evidence revisions is confined to the issue's declared `.csdlc/evidence/<issue>/` surface.
6. No file in that evidence surface changes after the derived evidence revision.
7. Existing source-at-revision, command, log, negative-case, native-receipt, and artifact digests still validate exactly.

The contract keeps v2 receipts fail closed with their original exact-HEAD rule. Only explicit v3 receipts use the two-revision model; there is no silent reinterpretation of retained evidence.

## Failure semantics

- Unknown schema: reject.
- Missing, malformed, or unresolved source revision: reject.
- Missing or ambiguous receipt-introduction commit: reject.
- Stored self-referential `evidence_revision`: reject.
- Reversed or unrelated ancestry: reject.
- Product/source change in the evidence commit range: reject.
- Evidence revision not contained by current HEAD: reject.
- Any existing digest or runner-provenance mismatch: reject.

## Validation

A focused temporary Git-repository regression creates substantive commit A, evidence-only commit B, and later metadata commit C. It proves A/B/C acceptance and retained-v2 exact-HEAD behavior, and rejects product drift, unrelated ancestry, receipt tampering, source-digest tampering, log tampering, and malformed revisions.

## Rollback

Revert the v3 schema branch and regression while leaving retained v2 evidence unchanged. No receipt migration is required.

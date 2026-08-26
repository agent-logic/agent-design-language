# Issue #353 design: publication review anchor consistency

## Defect

After supported `recover_review`, a fresh exact review, and typed republish, publication evidence records the reviewed pre-publication commit. Finish treats that commit as the publication metadata anchor and requires its historical issue index to contain the current review. That commit necessarily predates `record_review`, so its review is null and terminal delivery becomes impossible even though the republished PR head contains matching review truth.

## Bounded correction

Publication must retain two identities without conflation:

- reviewed revision: immutable substantive/recovery commit authorized by fresh review;
- publication metadata commit: exact pushed PR head containing the canonical completed review and publication projections.

Use finish's `expected_head_sha`, which is resolved from the live PR by the typed finish owner and checked against the exact remote head, as the historical metadata anchor during finish lineage validation. This is non-recursive: it is read-only live input to finish, not another field persisted before or after publication. Continue using `review.reviewed_revision` and `publication.revision` as the reviewed substantive authority for scoped equivalence and ancestry. Before accepting forward metadata drift, finish must read `expected_head_sha:.csdlc/issues/<issue>/index.json` and require its review object to equal canonical review; null/unequal/missing indexes fail closed. No model/schema widening is required.

## Regression

Add focused unit coverage around the lineage helper plus the existing complete finish test target, reproducing:

1. implemented candidate reviewed at commit A;
2. review metadata committed at B;
3. publication metadata pushed/observed at C with canonical review retained;
4. expected PR head C differs from reviewed A only through governed metadata;
5. finish accepts equal review in C and verifies A is ancestral/scoped-equivalent;
6. a review-null or unequal historical anchor fails closed;
7. substantive or non-governed drift still fails.

## Ownership

- `csdlc-v2/src/finish.rs`
- `csdlc-v2/src/publication.rs` and model/schema only if required to retain the exact metadata anchor
- focused `csdlc-v2/tests/` finish/publication regression
- issue-local #353 lifecycle/evidence

#349 implementation and PR #352 are read-only consumers. #342 is forbidden and untouched.

## Validation

Run the focused regression, existing finish/publication tests, formatting, strict C-SDLC v2 Clippy, and exact review. Preserve all linkage, CI, review, ancestry, and governed-metadata gates.

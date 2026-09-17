# PR #1053 repair

The merge with origin/main preserves #1051's caller-supplied semantic version through publication amendment admission and the locked reservation compare-and-swap. Saved A cannot overwrite newer B, including stale no-op content or a missing version. Pending publication rejects both changed and unchanged metadata before reservation. Generic projection recovery no longer reports a publication amendment. The merged documentation now has one publication-edit contract.

Independent reviewer `/root/review_1048/review_1053_cas` reviewed source/docs merge `2000ef872df02b3ef9642db0fa7bb92d06d750b2` against parents `2f0ccd456131f91bb1cb325eed10bce9ace42365` and `e76dd7e785d778916b524864118ef079e0a1836f`: no remaining actionable findings. The generic projection result and pending-noop findings are fixed.

Fresh validation: 11 installed publication tests passed (300.29 seconds); six operator-manual tests passed. A focused pending-publication regression also passed after adding the unchanged-metadata guard; this overlaps the publication suite and is not an additional unique test count. The standalone prepared/bound stale-request test passed and is included in the 11-test suite. Deterministic synthetic transport proof only; no live provider execution claim. Historical 149-test evidence predates this merge and is not fresh repair proof.

The isolated owner was rebuilt at `.csdlc/owners/1048/csdlc`; the shared owner was not replaced. PR #1053 already exists and remains unmerged. Exact final-head native proof and independent review must succeed before updating that existing publication. Original completed authenticated-no-effect recovery coverage remains static-only as documented in REVIEW.md.

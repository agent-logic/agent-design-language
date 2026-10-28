# Cross-repository CodeFriend integration review

**INTEGRATION-001 P1:** The frozen native and website source pair is incompatible for current assessment cycles. Native emits lane contract v4, but website admission requires v2 in both the run version map and each lane. This blocks current review-bearing cycle result admission. The141 passing website tests use historical hand-authored v2 fixtures and do not prove current native interoperability.

The actual website validator accepted its legacy fixture and rejected a resealed v4-version variant. This is a version-gate reproduction, not a native-emitted complete record or deployed failure. Native lanes.rs3 and runner539–547 establish producer version independently. Runs.recordResult→validateCycleResult→validateReviewResult→validateAssessmentReview establishes the consumer path. Preserve historical identities while adding v4 prompt/gap compatibility and real cross-repository fixtures.

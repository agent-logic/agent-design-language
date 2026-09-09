# Issue 761 design

Classify every path, select lane samples after classification, record auditable denominator manifests, validate integrity, prove exact fixture, obtain independent review and publish.

AC-1: Classify complete inventory before sampling.
AC-2: Record lane denominator, selected paths, exclusions, selection rule and digest.
AC-3: Reject empty or category-invalid materially present mandatory lanes.
AC-4: Distinguish complete deterministic routing scans from sampled manual review.
AC-5: Exact 5481-path #520 fixture populates all six mandatory lanes.
AC-6: Empty, invalid-category and digest-mismatch negatives fail deterministically.

Use a complete path inventory and per-lane eligible sets, then deterministic samples. Preserve exclusions and SHA-256 hashes. Independent packet validation recomputes assignments against inventory. Exercise #520 immutable changed paths and malformed manifests.

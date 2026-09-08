# TAIL-01 Quality Gate Design

Issue #517 produces one fail-closed quality-gate decision for the exact v0.92.1 candidate admitted by #516. It inventories every required proving lane, rejects absent, skipped, zero-test, stale, or non-proving evidence, and records whether any exception lacks an owner. It does not repair documents, implement product changes, publish, merge, or perform release ceremony.

Execution starts only after #516 has a reviewed merge. The output is limited to `docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md` and issue-owned evidence under `docs/milestones/v0.92.1/evidence/release/tail-01/`.

# Multi-Perspective Review

Status: planned. Owners: CF-REVIEW, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN.

The review engine runs correctness, security, adversarial, and constitutional perspectives as attributed lanes before synthesis. CF-SYNTHESIS deduplicates without erasing provenance and explains severity. CF-REMEDIATE and CF-TESTPLAN separately consume those findings to produce usable remediation and test plans.

Acceptance requires evidence-linked findings, perspective attribution, severity calibration, duplicate control, and proof that action plans do not mutate source. Security tournaments and autonomous fixing are deferred.

Each perspective receives the same scoped evidence packet plus its own instructions. It commits its result before seeing peer findings; only synthesis consumes all lane results. A no-cross-lane-input fixture proves isolation, and synthesis retains disagreement. This requires independent inputs, not four different vendors. Partial lanes remain explicitly incomplete.

The listed owners deliver separate working tasks, with complete consumer and failure evidence defined in the [atomic task contracts](../ATOMIC_TASK_CONTRACTS_v0.92.2.md). A feature packet or schema alone cannot close an implementation row.

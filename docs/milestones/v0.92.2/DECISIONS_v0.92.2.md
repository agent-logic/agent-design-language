# v0.92.2 Planning Decisions

Status: proposed decisions for milestone execution.

| ID | Decision | Consequence |
|---|---|---|
| CF-D01 | v0.92.2 is the complete CodeFriend Beta 1 milestone. | Beta 1 is not spread across the older v0.93-v0.95 alpha schedule. |
| CF-D02 | All tools consume shared provider contracts. | CodeFriend-specific provider forks are rejected. |
| CF-D03 | Evidence identity, provenance, redaction, and retention precede report generation. | Unsupported or unsafe evidence fails closed. |
| CF-D04 | Four review perspectives remain distinct before synthesis. | Synthesis cannot erase perspective ownership or severity rationale. |
| CF-D05 | Publication and repository mutation remain human-controlled. | Beta 1 proposes actions and artifacts but does not autonomously edit source. |
| CF-D06 | Longitudinal comparison is part of Beta 1. | Stable identity and schema compatibility are release requirements. |
| CF-D07 | ADL self-review plus one bounded external open-source review are acceptance proofs. | Synthetic-only demos are insufficient. |
| CF-D08 | The canonical ten-step release tail is preserved. | Closeout bookkeeping remains asynchronous and does not gate unrelated implementation. |
| CF-D09 | No calendar deadline is encoded. | Readiness is evidence-based, not date-based. |
| CF-D10 | ATE and Runtime v4 remain separately planned. | Their absence does not block Beta 1 unless an admitted dependency proves otherwise. |
| CF-D11 | The operator explicitly admitted one bounded MLX/Apple Metal provider adapter to v0.92.2. | PLAT-MLX is separate from provider-definition work, follows PLAT-PROVIDER, and makes no general local-model claim. |
| CF-D12 | Completed issue #484 is the baseline AWS ownership inventory, not work to repeat. | OPS-AWS produces a current delta, stale-item disposition, and maintenance runbook without recreating #484. |

None of these decisions claims implementation, review approval, or release readiness.

## #523 planning corrections

- CF-D13: CF-EVIDENCE owns the shared finding/run contract and conformance fixtures before parallel consumers.
- CF-D14: Pre-synthesis lane outputs are isolated; synthesis preserves disagreement.
- CF-D15: Candidate-changing remediation refreshes affected artifacts, proof and internal/external review before release.
- CF-D16: Existing #720 is reused. Operator direction on 2026-09-09 promoted #717/#718 into v0.92.1; v0.92.2 consumes their merged results as predecessors and does not recreate them. Other backlog is excluded.
- CF-D17: Local CLI with artifact browsing in this repository is the bounded product design candidate; shared ADL contracts are consumed, not forked. WP-01 accepts or explicitly revises it before new-wave execution.
- CF-D18: The declared release-gating support set converges at the milestone quality gate; only actual product prerequisites block integration. Independently deliverable required issues such as OBS-S3 and ARCH-ADR retain their own acceptance outside the closeout tail.
- CF-D19: The operator admitted the bounded NVIDIA PAIR experiment as PLAT-PAIR after PLAT-PROVIDER; it is an experiment with a retained decision, not a production-provider claim.
- CF-D20: The operator admitted the company GCP move-in reconciliation as OPS-GCP after WP-01; it is read-only planning and inventory work that consumes merged v0.92.1 foundations and does not repeat six-resident qualification or authorize cloud mutation.
- CF-D21: The operator admitted OBS-S3 as a non-product-gating deployment sidecar. It consumes the completed #679/merged PR #685 Terraform design, waits for OBS-LIVE, uses the Agent Logic business AWS account, and does not authorize customer-scale hosting or an apply during #525.
- CF-D22: The operator admitted ARCH-ADR as a distinct milestone work package. WP-01 creates its issue; it generates and reconciles the ADR set needed by v0.92.2 without implementing decisions or treating candidate prose as accepted authority.

These corrections do not accept ADRs, create issues, apply cloud infrastructure, approve Beta implementation, or complete #524/#525.

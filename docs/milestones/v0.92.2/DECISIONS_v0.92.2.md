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

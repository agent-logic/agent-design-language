# v0.92.2 ADR Plan

Status: planned candidates owned by the `ARCH-ADR` work package; this file does not accept ADRs. WP-01 creates the work-package issue and assigns its canonical identity.

| Candidate | Decision surface | Owner | Trigger |
|---|---|---|---|
| ADR-CF-01 | Portable Adapter v2 packet boundary | CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI | Before incompatible adapter implementations |
| ADR-CF-02 | Stable evidence identity and provenance model | CF-EVIDENCE | Before persistent artifacts are emitted |
| ADR-CF-03 | Redaction and retention authority | CF-EVIDENCE | Before repository proof packets are retained |
| ADR-CF-04 | Architecture cognition model and explanation contract | CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE | Before scoring or finding schemas stabilize |
| ADR-CF-05 | Fitness-function boundary between machine proof and human judgment | CF-GOV, CF-GOV-CI | Before CI enforcement |
| ADR-CF-06 | Four-perspective review and synthesis authority | CF-REVIEW, CF-SYNTHESIS | Before review-engine integration |
| ADR-CF-07 | Longitudinal compatibility and delta semantics | CF-MEMORY | Before second-run storage stabilizes |
| ADR-CF-08 | Human publication boundary and renderer parity | CF-UX, CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF | Before publish controls stabilize |

Each candidate must cite alternatives, consequences, reversibility, and evidence. Implementation does not silently accept the decision.

`ARCH-ADR` must begin by reconciling this seed list against the final WP-01 issue wave. It may add, combine, retire, or defer candidates only with source-grounded rationale, explicit ownership, and status truth. Its primary result is one milestone ADR set and supersession map—not implementation of the decisions and not automatic acceptance of candidate text.

The owner sets above are participating implementation tasks, not bundled implementation issues. ARCH-ADR remains one required planning deliverable: the completed source-grounded decision set and supersession map. Reconcile provider configuration versus lifecycle, action-planner consumers, Runtime evidence ownership and local/remote command decomposition against the [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md) when selecting necessary ADRs. A candidate record cannot close an implementation task.

ARCH-ADR completion and its decision-set acceptance explicitly gate TAIL-10 milestone closure. It remains outside CF-INTEGRATE and the early TAIL-01 quality gate; the ten-step release order is preserved.

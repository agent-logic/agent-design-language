# [v0.92.2][TAIL-01] Quality gate

## One completed quality decision

Execute the milestone quality gate against the current installed candidate and independent CF-PROOF evidence. Own `docs/milestones/v0.92.2/QUALITY_GATE_v0.92.2.md` result references, feature/proof coverage reconciliation and an issue-local machine-readable quality decision. Consume actual producer artifacts rather than recreating their tests under this gate.

Account for all 69 task identities, preserving seven planning results as distinct from product behavior. Only the declared dependencies require accepted merged completion here; later tail stages are not prerequisites. OBS-S3 and ARCH-ADR remain explicitly outside this early gate and must complete for TAIL-10.

Evaluate all twelve required quality gates: planning parity; three input routes; identity/provenance/redaction/retention; explainable architecture; deterministic fitness; four attributed lanes/synthesis; bounded plans; second-run comparison; exact approved three-format parity; privacy/legal/manifests; ADL/external executed proof; integrated success/failure with no unresolved P1. Record pass/fail/not-proven, candidate/manifest digests, evidence pointers, skipped/missing denominators and findings. Missing or non-covering green CI cannot become pass. A passing decision with current independent review is required for downstream finalization; a failed report is retained but does not satisfy the gate.

PVF: deterministic local evidence/identity/denominator validation and independent human quality audit; local CPU/Git/files plus authenticated read-only observations, required release gate. Negative checks reject missing feature proof, stale candidate, counterfeit producer success, missing prerequisite and unresolved P1; do not run paid workloads by implication.

## Startup, dependencies and scope authority

The operator's global startup gate applies: **all 69 issue identities must be created and all batch reviews must pass before any implementation starts**. Creation/review batching adds no execution dependency edges. After that gate, each task still requires its own exact prerequisites, native readiness, bound FastWork ownership and issue-bound goal; root main remains inspection-only. Issue creation does not authorize paid/provider/cloud operations, external messages, public publication, writer activation or a successor milestone. Use explicit existing scope authority for those effects.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `RELEASE_PLAN_v0.92.2.md`, `QUALITY_GATE_v0.92.2.md`, `MILESTONE_CHECKLIST_v0.92.2.md` and `NEXT_MILESTONE_HANDOFF_v0.92.2.md`. Re-resolve exact candidate and existing ownership before acting. Preserve canonical TAIL-01 through TAIL-10 order and all seven planning tasks. Each issue delivers its one complete named result; supporting code/docs/proof are part of that result, not unrelated work.

New tests declare lane, role, determinism, resources and gate in the coupled proof manifest. Preserve redacted stderr diagnostics and machine-readable stdout, immutable evidence and private-source boundaries. Independent exact-head review and required checks precede publication; distinguish local validation, CI, actual execution, human approval and terminal reconciliation. Missing proof/authority, stale identity or conflicting ownership stops dependent action with durable evidence; do not retry around guards.

## Canonical obligation ledger

Dependencies: CF-INTEGRATE, CF-PROOF, PLAT-MLX, PLAT-PAIR, PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, OBS-LIVE, ARCH-SPLIT, CSDLC-MERGE, QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, QUAL-EVIDENCE, RT-COST, CSDLC-MAN, CSDLC-DECOMPOSE, CSDLC-REMOTE, SIM-UMBRELLA. Existing canonical mappings include WP-01/#864, RT-PROVIDER/#855 and SIM-UMBRELLA/#866; resolve remaining logical IDs from the verified creation map, never infer numbers.

acceptance: `quality_gate_evaluated`, `no_unresolved_p1`.

pvf: `release_evidence_audit`.

Wave proof: `denominator_reconciliation`, `quality_gate_validation`.

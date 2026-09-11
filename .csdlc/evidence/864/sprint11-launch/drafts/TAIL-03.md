# [v0.92.2][TAIL-03] Publication finalization

## One finalized publication packet

Produce the complete versioned release artifact set and manifest ready for explicit human publication decisions, without publishing it. Own final draft `RELEASE_NOTES_v0.92.2.md`, issue-local finalization manifest and copies/references of approved product outputs. Do not implement renderers here.

Bind every capability claim to actual merged implementation and current proof. Reconcile Markdown/HTML/PDF outputs, claims/nonclaims, provenance, privacy/legal checks, artifact versions/digests and destinations. Open actual outputs and verify format parity and completeness; reject missing/stale/tampered artifacts, redaction failure or mismatched candidate. Record approval as pending or explicit with identity and exact scope; never infer approval from completion of this issue. Independent review leaves a usable publication packet for TAIL-04, not an outline. External publication awaits separate authority and the remaining tail.

PVF: local deterministic manifest/digest/parity validation plus human content/privacy review; bounded CPU/files/render checks; required tail finalization. No provider/cloud use or public upload is implied.

## Startup, dependencies and scope authority

The operator's global startup gate applies: **all 69 issue identities must be created and all batch reviews must pass before any implementation starts**. Creation/review batching adds no execution dependency edges. After that gate, each task still requires its own exact prerequisites, native readiness, bound FastWork ownership and issue-bound goal; root main remains inspection-only. Issue creation does not authorize paid/provider/cloud operations, external messages, public publication, writer activation or a successor milestone. Use explicit existing scope authority for those effects.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `RELEASE_PLAN_v0.92.2.md`, `QUALITY_GATE_v0.92.2.md`, `MILESTONE_CHECKLIST_v0.92.2.md` and `NEXT_MILESTONE_HANDOFF_v0.92.2.md`. Re-resolve exact candidate and existing ownership before acting. Preserve canonical TAIL-01 through TAIL-10 order and all seven planning tasks. Each issue delivers its one complete named result; supporting code/docs/proof are part of that result, not unrelated work.

New tests declare lane, role, determinism, resources and gate in the coupled proof manifest. Preserve redacted stderr diagnostics and machine-readable stdout, immutable evidence and private-source boundaries. Independent exact-head review and required checks precede publication; distinguish local validation, CI, actual execution, human approval and terminal reconciliation. Missing proof/authority, stale identity or conflicting ownership stops dependent action with durable evidence; do not retry around guards.

## Canonical obligation ledger

Dependencies: TAIL-02. Existing canonical mappings include WP-01/#864, RT-PROVIDER/#855 and SIM-UMBRELLA/#866; resolve remaining logical IDs from the verified creation map, never infer numbers.

acceptance: `artifacts_versioned`, `manifests_complete`, `approval_pending_or_explicit`.

pvf: `artifact_integrity`.

Wave proof: `artifact_manifest_validation`, `candidate_binding`.

# [v0.92.2][TAIL-05] External or third-party review

## One independent external review

Obtain and retain a context-free external or third-party review against the exact candidate/manifest following TAIL-04. Own issue-local external handoff, reviewer provenance and findings/disposition references. The reviewer must be independent of implementation and internal review authorship; a local rerun by the same author cannot be relabeled external.

Provide the validated TAIL-02/03 handoff and internal findings with explicit disclosure/scope. Sending private evidence or contacting an outside reviewer requires explicit authorization; issue creation alone does not authorize those communications. If no authorized independent reviewer is available, retain the blocker rather than inventing a review.

Retain exact reviewed identities, methods, accessible source/evidence, findings, severity, limits and pass/fail/not-proven assessment. Check that observations actually address the selected candidate; reject stale or unauthenticated review, omitted required scope and inaccessible evidence claims. Route every accepted finding to TAIL-06 without editing the original review. The task delivers the completed independent assessment, not a request waiting for feedback; adverse findings do not disappear because the review occurred.

PVF: independent human/external review plus deterministic identity/reference checks, bounded read-only evidence access; required external review gate. No claimed Runtime test is added by merely validating the handoff.

## Startup, dependencies and scope authority

The operator's global startup gate applies: **all 69 issue identities must be created and all batch reviews must pass before any implementation starts**. Creation/review batching adds no execution dependency edges. After that gate, each task still requires its own exact prerequisites, native readiness, bound FastWork ownership and issue-bound goal; root main remains inspection-only. Issue creation does not authorize paid/provider/cloud operations, external messages, public publication, writer activation or a successor milestone. Use explicit existing scope authority for those effects.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `RELEASE_PLAN_v0.92.2.md`, `QUALITY_GATE_v0.92.2.md`, `MILESTONE_CHECKLIST_v0.92.2.md` and `NEXT_MILESTONE_HANDOFF_v0.92.2.md`. Re-resolve exact candidate and existing ownership before acting. Preserve canonical TAIL-01 through TAIL-10 order and all seven planning tasks. Each issue delivers its one complete named result; supporting code/docs/proof are part of that result, not unrelated work.

New tests declare lane, role, determinism, resources and gate in the coupled proof manifest. Preserve redacted stderr diagnostics and machine-readable stdout, immutable evidence and private-source boundaries. Independent exact-head review and required checks precede publication; distinguish local validation, CI, actual execution, human approval and terminal reconciliation. Missing proof/authority, stale identity or conflicting ownership stops dependent action with durable evidence; do not retry around guards.

## Canonical obligation ledger

Dependencies: TAIL-04. Existing canonical mappings include WP-01/#864, RT-PROVIDER/#855 and SIM-UMBRELLA/#866; resolve remaining logical IDs from the verified creation map, never infer numbers.

acceptance: `external_handoff_complete`, `external_findings_retained`.

pvf: `external_review`.

Wave proof: `context_free_external_review`, `exact_candidate_binding`.

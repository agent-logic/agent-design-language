# Corporate and Runtime evidence notes from the accounting audit

These are bounded source-inspection suggestions for 42 corporate/Runtime rows. They are not an implementation plan for #517. The operator clarified docs-only accounting and identified completed successor closeouts in PR #750; use the authoritative successor and terminal joins before deciding whether any suggested further work is necessary. It does not create 42 new issues, claim that 42 implementations are broken, authorize cloud launches, or change historical acceptance criteria. Existing scope amendments remain in force. Complete the local evidence joins first; only the uncovered production or private-attestation obligations require further action.

## Work that can proceed locally now

| Work package | Retained criteria | Concrete work and acceptance evidence |
| --- | --- | --- |
| Corporate inventory and safeguards | 153-ac-2/3; 157-ac-1/2/3; 158-ac-4; 159-ac-1/4; 160-ac-1 | Join the critical-asset schedule to the custody register by asset ID. Show current/target control, transfer dependency, recovery authority and rollback for every required row, or name the exact missing fields. Recover the approved seven-repository migration inventory and before/after visibility/history receipts. Inventory temporary versus deliberately persistent bootstrap resources; map state recovery and runbook proof. Add focused negative fixtures for missing required recovery/exclusion fields. Do not fill factual custody fields with guesses. |
| ACIP production proof | 181-ac-4; 182-ac-1/2/3/4 | Separate qualification-contract examples from production signed-message admission. Enumerate the supported message families and required identity/authority/permit/causation/correlation/sequence/term/polis mutations. Join existing real codec/admission/replay tests and their exact executed receipts to that denominator. Add only missing production-boundary checks, including repaired-positive controls. Retained input/output/commit digests must support replay; fixture self-equality is insufficient. |
| Distributed execution evidence consumer | 183-ac-1..4; 184-ac-1..4; 185-ac-1..4; 186-ac-1..4; 187-ac-1..4 | Build one evidence intake/checker that binds actual source revisions, producer commands, identities, quorum/lease decisions, clocks, feeds, failure phases, replay and cleanup. Replace the use of generated `deterministic_drt_c` metadata as execution evidence. Preserve it as contract-fixture proof. Consume existing producer artifacts first and report exactly which fields/scenarios remain missing. A negative case must reject copied fixture metadata as observed execution. |

Start from these existing sources:

- `docs/operations/corporate/asset-register/critical-asset-schedule.v1.json` and `docs/operations/corporate/account-custody/corporate-custody-register.v1.json`.
- `docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json`, `docs/milestones/v0.92.1/evidence/corporate/corp-c/live-control-plane-readonly-probe.v1.json`, and `docs/operations/cloud/aws/terraform-bootstrap/AWS_TERRAFORM_BOOTSTRAP_RUNBOOK.md`.
- `adl-runtime/tests/distributed_identity.rs`: real signing/enrollment and durable identity-store rejection boundaries, including wrong domain, replay, malformed signature, expiry and shared voter keys.
- `adl-runtime/tests/distributed_lease.rs`: signed certificate quorum and lease ledger behavior. `stable_and_joint_openraft_membership_enforce_exact_quorum` proves certificate verification, not independently started voter processes.
- `adl-runtime/tests/distributed_observatory_authority_projection.rs`: authentic verifier binding and redacted projection tests. These do not alone prove every displayed operation or an entire process partition/recovery journey.
- `adl-runtime-kernel/tests/guardian_soak.rs`: real-kernel process tests for lease loss, shutdown, pressure, signed HTTPS/WSS and forged shutdown denial.
- `adl-runtime/tests/runtime_v3_soak.rs`: bounded execution-evidence schema and negative checks for omitted faults, reversed clocks, cleanup and cancellation. These are useful checker implementation, not proof that a required soak actually ran.

The bounded source examination identified reusable production-boundary tests, but did not verify an executed receipt covering an entire additional retained criterion. No row was promoted merely because a promising test exists. Join immutable execution/review provenance before promotion.

## Existing live and historical proof to reuse without another launch

`docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json` retains real #509 GCP run `adl-509-drt-d-20260902192222`, source `f61de6ac171253db3d0afb47ec3e4c1838b47c54`, six resident tool receipts, model artifacts, population restoration and run-specific cleanup. `.csdlc/issues/509/cards/srp.md` records review at `310ad9b65701bdc20e6aaa52056fb511dda2dfcc` and expressly says the reviewer did not rerun the paid proof. Its topology is two machines: one Runtime and one Ollama node. It therefore provides useful six-resident and GCP cleanup evidence, but cannot stand in for three voter processes, separate AWS AZs, Wuji isolation, quorum healing or both distributed soak windows.

Also examined:

- `docs/architecture/runtime_v3_guardian_soak_report.v1.json`: local 100-cycle Guardian proof from July, with source hashes and focused faults.
- `docs/architecture/runtime_v3_soak_rollback_5253.v1.json`: explicitly bounded production-like engineering soak; remote multi-day and GPU proof deferred.
- `docs/architecture/runtime_v3_continuity_replay_recovery_5280.v1.json`: direct kernel continuity checks, explicitly no distributed multi-node/Polis-resilience claim.
- `docs/architecture/runtime_v3_horust_qualification_evidence.v1.json`: focused native lifecycle and systemd containment; explicitly no complete soak or GPU qualification.

These boundaries prevent accidental reuse of a successful narrow run as proof of a broader requirement. This was a bounded search of declared successor evidence, the named older proof packets and selected production tests; it is not a claim that no other proof exists anywhere.

## Work requiring external facts or an approved execution environment

| Work package | Retained criteria | Required result |
| --- | --- | --- |
| Redacted corporate approval and use rights | 154-ac-1..4; 155-ac-1..3; 160-ac-2 | Locate existing counsel/corporate approval, required-party execution/disposition, private-custody and per-asset use-rights attestations. Retain only safe identifiers, exact binding digests, approved scope, custodian role and outcome. Existing public tuple hashes prove structural consistency, not underlying execution or counsel approval. Code cannot manufacture these facts. Private-vault operational work already routed to #704 remains non-gating within the explicit #483 amendment; do not broaden that amendment to unrelated legal approval obligations. |
| Local production Runtime journey | Uncovered parts of 183,185,186,187 | Run independently started voters and governed work through production entrypoints, with a non-voting Shepherd and leased Observatory; demonstrate quorum loss, stale-owner denial, restart/restore, replay, failure injection and cleanup. This can be local only where the original criterion permits local execution and the necessary existing runtime binaries/identity material are available. Use declared finite workloads and real clocks; record every actual command and source/model digest. |
| Hybrid AWS/Wuji journey | Uncovered parts of 184 and hybrid window187 | Requires the approved Agent Logic business AWS account, suitable nodes in separate AZs, private authenticated transport and an authorized bounded run. Before launching anything, recover existing paid authorization and existing execution receipts. If proof is still missing, make exact resources, cost ceiling, duration, fault phases, cleanup selectors and rollback reviewable through the owning lifecycle. Demonstrate isolation, AWS-only quorum, stale-node denial, loss-of-quorum stop and healing before traffic resumes. |

## Completion check

For each work package, update the criterion mapping only after the cited evidence proves the full original wording or an exact operator amendment disposes that obligation. Keep partial proof and scope amendments distinct from executed pass. Recompute grouped counts, run the focused gate validator, and obtain independent review of the final mapping and any changed evidence-consumer behavior. The release gate cannot truthfully pass solely because this plan or a checker exists.

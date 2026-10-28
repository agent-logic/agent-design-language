# Ceremony preflight gaps discovered during #526 preparation

Status: unresolved; source inspection only. No release mutation or retired lifecycle command was executed.

1. **Version mismatch:** `adl/Cargo.toml` declares `0.92.0`. `adl/tools/release_ceremony.sh` function `check_cargo_version` requires `0.92.1` for `--version v0.92.1` and otherwise rejects before printing a successful plan. Resolve version alignment in the authorized version/release tooling scope before final candidate selection. A version bump may affect lockfiles and other manifests; do not assume changing one string suffices.
2. **Retired authority fallback:** no `docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json` is present. The wrapper's `check_typed_closeout_gate` consequently falls back to `run_csdlc_doctor`, which invokes `.adl/bin/csdlc-v2/csdlc-doctor` or builds the v2 owner. Current repository authority requires native v3 and permits v2 only under an explicit exception. Supply a reviewed milestone-specific gate or repair the wrapper in a bounded tooling issue. Do not use `--skip-sor-gate` as remediation.

Safe reproduction: inspect these tracked functions, the manifest version, and absence of the milestone gate at the preparation SHA recorded in `ceremony-preparation.json`. The control flow is version check, typed closeout gate, then plan. No resource or credential access is needed to establish these gaps.

These are local durable findings, not newly created GitHub issues or completed fixes. They must receive an execution owner before the ceremony is considered ready. #526's requested preparation does not silently widen into implementation of shared tooling.

## Where the version obligation was lost

The existing review scope already includes this check:

- `docs/templates/planning/1.1.0/release_plan.md`, Release-Tail Convergence, requires release-truth alignment including `Cargo.toml`.
- `docs/milestones/v0.92.1/CANONICAL_DOC_INVENTORY_v0.92.1.md` explicitly includes all 24 tracked manifests.
- `docs/milestones/v0.92.1/evidence/release/tail-02/CARGO_MANIFEST_REVIEW.md`, Release disposition and limits, records effective version `0.92.0` and assigns identification of release artifacts and required coordinated version changes to #519.
- `docs/milestones/v0.92.1/evidence/release/tail-03/README.md` defines #519's delivered checks as linkage, exact-head integrity and redaction, and explicitly says it changes no package versions. The live #519 issue was CLOSED when checked on 2026-09-11; its acceptance criteria cover candidate/linkage and closing relationships, not version reconciliation.

This establishes a lost handoff obligation, rather than an absent manifest inventory. It does not establish what an external reviewer read or why that reviewer missed the consequence. Closing #519 supplies no evidence that this inherited obligation was resolved.

Required acceptance evidence before candidate approval: an explicit release-artifact inventory; coordinated manifest/lockfile versions for those artifacts; and a successful non-mutating ceremony preflight for `v0.92.1` at the exact proposed candidate using current v3 authority. Keep independently versioned helpers separate where justified. A recorded version, a manifest parse, or a closed issue is insufficient.

## Routing update — 2026-09-11

The original findings and discovery-time status above are preserved verbatim. Subsequent routing does not resolve either finding.

Repair owner: [#856](https://github.com/agent-logic/agent-design-language/issues/856), created and authenticated through native v3. It covers coordinated release versions and current v3 preflight, including focused positive and negative proof. Both findings remain OPEN until the repair is reviewed, integrated and verified. #833 and its review worktree are outside this repair scope.

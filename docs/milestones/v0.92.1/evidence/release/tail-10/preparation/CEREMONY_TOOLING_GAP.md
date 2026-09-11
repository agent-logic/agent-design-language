# Ceremony preflight gaps discovered during #526 preparation

Status: unresolved; source inspection only. No release mutation or retired lifecycle command was executed.

1. **Version mismatch:** `adl/Cargo.toml` declares `0.92.0`. `adl/tools/release_ceremony.sh` function `check_cargo_version` requires `0.92.1` for `--version v0.92.1` and otherwise rejects before printing a successful plan. Resolve version alignment in the authorized version/release tooling scope before final candidate selection. A version bump may affect lockfiles and other manifests; do not assume changing one string suffices.
2. **Retired authority fallback:** no `docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json` is present. The wrapper's `check_typed_closeout_gate` consequently falls back to `run_csdlc_doctor`, which invokes `.adl/bin/csdlc-v2/csdlc-doctor` or builds the v2 owner. Current repository authority requires native v3 and permits v2 only under an explicit exception. Supply a reviewed milestone-specific gate or repair the wrapper in a bounded tooling issue. Do not use `--skip-sor-gate` as remediation.

Safe reproduction: inspect these tracked functions, the manifest version, and absence of the milestone gate at the preparation SHA recorded in `ceremony-preparation.json`. The control flow is version check, typed closeout gate, then plan. No resource or credential access is needed to establish these gaps.

These are local durable findings, not newly created GitHub issues or completed fixes. They must receive an execution owner before the ceremony is considered ready. #526's requested preparation does not silently widen into implementation of shared tooling.

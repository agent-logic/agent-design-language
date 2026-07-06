# CSM Restore Fire-Drill Evidence (#4919)

Status: `passed_local_drill`

This packet retains the first CSM continuity fire-drill proof for WP-07. The
drill uses the runtime-owned `csm continuity drill` command against a
current-format continuity capsule derived from the retained #4910 runtime state.

## Command Surface

```sh
csm continuity drill \
  --bundle docs/milestones/v0.91.7/review/runtime/csm_restore_fire_drill_4919/current-capsule \
  --out docs/milestones/v0.91.7/review/runtime/csm_restore_fire_drill_4919/drill \
  --target-host local \
  --cadence pre-v0.92 \
  --json
```

The command stages and restores into the drill output directory only. It does
not mutate live runtime state or perform production traffic cutover.

## Cadence

- Daily: operator may run a non-mutating local drill against the newest retained capsule.
- Per release: release candidates must retain a drill report or an evidence-backed blocker.
- Pre-v0.92: runtime-coherence gate consumes the latest successful drill report.
- Manual: operator may run the drill whenever recovery evidence is stale or suspicious.

## Retained Evidence

- `current-capsule/continuity_capsule_manifest.json`
- `current-capsule/segments/continuity_checkpoint.snapshot.segment`
- `drill/fire_drill_report.json`
- `drill/stage/stage_report.json`
- `drill/restored-runtime/restore_report.json`

## Result

- Status: `passed`
- Target host: `local`
- Cadence: `pre-v0.92`
- Local RTO measurement: `8ms` for stage plus restore in this retained run.
- RPO scope: selected continuity capsule point-in-time.
- Negative cases: `missing_artifact` and `corrupted_manifest` failed as expected.
- Observability: `continuity_fire_drill`, `continuity_capsule_stage`, and `continuity_capsule_restore` are recorded as expected event stages.

## Validation

- `cargo check --manifest-path adl/Cargo.toml --locked --bin csm`
  - Result: passed.
  - Local wall time: `7s` after the initial warm compile.
- `cargo test --manifest-path adl/Cargo.toml --locked --test cli_smoke csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles -- --nocapture`
  - Result: passed.
  - Build wall time: `28s`; test execution time: `0.36s`.
- JSON parse checks passed for the current capsule manifest, fire-drill report, stage report, and restore report.
- Scoped retained-evidence hygiene scan found no host-private paths, secret-shaped strings, or prohibited recovery wording in this packet.

## Build Platform Timing

| Platform | Cache posture | Build | Test | Benchmark total | Wrapper wall | Status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `wuji` | `local_warm_target` | `93s` | `34s` | `127s` | `127s` | passed |
| `nessus` | pending remote run | pending | pending | pending | pending | pending |
| `codebuild` | pending remote run | pending | pending | pending | pending | pending |
| `aws_spot` | pending remote run | pending | pending | pending | pending | pending |

## Non-Claims

- This is not a production traffic failover proof.
- This is not a provider-secret restore proof.
- This is not a multi-region disaster recovery proof.

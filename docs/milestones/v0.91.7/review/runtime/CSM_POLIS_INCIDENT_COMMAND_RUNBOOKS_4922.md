# CSM Polis Incident-Command Runbooks (#4922)

Status: `operator_runbook_packet`

This packet gives operators a bounded incident-command playbook for the CSM
runtime and Polis survival surface. It is source-backed by the current WP-07
runtime evidence and intentionally separates proven commands from scheduled or
blocked capabilities.

## Command Policy

Use repo-native commands only:

- Runtime owner: `csm daemon --spec <agent-spec.yaml> --checkpoint-interval-secs 3 --json`
- Runtime API: `csm api serve --spec <agent-spec.yaml> --bind 127.0.0.1:0 --json`
- Agent status: `adl agent status --spec <agent-spec.yaml> --json`
- Agent stop: `adl agent stop --spec <agent-spec.yaml> --reason "<reason>" --json`
- Process liveness: `adl process status --pid-file <path> --json` or
  `adl process status --port <port> --json`
- Continuity: `csm continuity capture --spec <agent-spec.yaml> --out <dir>`
  then `csm continuity stage --bundle <bundle-dir> --out <stage-dir>` and
  `csm continuity restore --bundle <bundle-dir> --out <runtime-dir>` when the
  continuity surface is present in the active owner binary.

AWS work must use the Agent Logic business profile `agent-logic-admin`. Do not
print, copy, or commit provider credentials, AWS credentials, account
identifiers, tokens, or credential-file contents.

Unsafe commands are non-commands here: do not use broad `ps`, broad `pgrep`,
broad `lsof`, raw `gh`, credential dumps, or ad hoc cloud profile defaults for
incident handling.

## Source Ledger

| Surface | Current evidence | Runbook use |
| --- | --- | --- |
| CSM daemon supervision | `RUNTIME_DAEMON_SUPERVISION_4885.md` | Daemon lifecycle, checkpoint cadence, recoverable stop, restart budget events. |
| No-sparrow observability | `no_sparrow_4909/README.md` | Event-retention coverage and telemetry non-claims. |
| OTLP/local OTel | `csm_otlp_4904/README.md` and `CSM_RUNTIME_API_4929.md` | Local OTel/status/log evidence and API observability checks. |
| Continuity capsule | `csm_continuity_capsule_4910/README.md` | Capture, stage, restore, EC2 Spot fire-up proof, migration boundaries. |
| Runtime API | `CSM_RUNTIME_API_4929.md` | `/status`, `/health`, `/ready`, `/metrics`, `/events` diagnostics. |
| Resilience middleware | `RUNTIME_RESILIENCE_MIDDLEWARE_4783.md` | Scheduler watcher, AEE disposition, backpressure, timeout, cancellation signals. |
| Failure injection | `v0917_integrated_resilience_failure_injection_4784/README.md` | Negative cases, recovery classification, failure-injection reviewer path. |
| Soak 2 | `soak2_4682/README.md` | Integrated local runtime proof and remaining blocked-row boundaries. |

Scheduled but not claimed by this packet: hosted telemetry backend, production
multi-region disaster recovery, production-grade provider-secret export,
universal cloud control-plane automation, and final CAV red/blue production
coverage.

## Incident Taxonomy

| Incident | Primary signals | Initial classification | First stabilizing action |
| --- | --- | --- | --- |
| Daemon not reachable | `adl process status` fails for the pid file or port; `/health` unavailable. | `runtime_liveness_unknown` | Check exact pid/port, then inspect retained `daemon_status.json` and `operator_events.jsonl`. |
| Daemon restart budget exhausted | `daemon_status.json` or `/events` includes `restart_budget_exhausted`. | `runtime_degraded_terminal_for_current_budget` | Stop new work, preserve artifacts, capture continuity if available. |
| Checkpoint stale or missing | `/health` degraded or `/ready` not ready with checkpoint blockers. | `continuity_risk` | Trigger/capture continuity, verify checkpoint and replay manifest freshness. |
| OTel/exporter unavailable | `otel_status.json` reports failure or no retained OTel log when expected. | `observability_degraded` | Preserve local `observability.log`, `otel.jsonl`, and `/events`; do not claim hosted telemetry. |
| Storage unavailable | continuity capture/stage cannot write or read required artifacts. | `state_durability_risk` | Preserve the last readable state root, classify missing files, route to storage owner. |
| Snapshot/capsule corrupt | continuity negative case matches corrupted manifest or missing file. | `portable_state_invalid` | Do not restore; retain manifest, hash/check output, and route to continuity owner. |
| Cloud hook failure | AWS stage/fire-up path fails under `agent-logic-admin`. | `cloud_control_plane_degraded` | Preserve redacted AWS wrapper summary, keep local runtime path separate. |
| Suspected tamper | CAV/red-blue or custody packet indicates mismatch, replay anomaly, or unauthorized change. | `security_incident` | Freeze new writes where possible, preserve evidence, route to CAV/security issue owner. |
| Credential compromise | credential-like key in artifact, log, capsule, or API output. | `security_incident_credential` | Stop exposure, retain redacted evidence, rotate through approved break-glass policy owner. |
| Provider outage | provider step timeout/quota/auth/policy terminal event. | `provider_path_degraded` | Use AEE disposition and degraded fallback policy; do not retry uncontrolled. |
| Overload/backpressure | scheduler watcher queues/saturates, rate or bulkhead signal appears. | `capacity_degraded` | Stop admission of nonessential work, preserve queue/backpressure evidence. |

## Universal Incident Flow

1. Detect:
   - Check exact process target with `adl process status`.
   - Query `csm api serve` endpoints when the local API can be started:
     `/status`, `/health`, `/ready`, `/metrics`, `/events`.
   - Inspect retained state files only through repo-relative artifact refs in
     records; avoid publishing host-private absolute paths.
2. Classify:
   - Choose one taxonomy row.
   - Record whether the state is recoverable, degraded, terminal for current
     budget, security incident, or non-claim/scheduled capability.
3. Stabilize:
   - Prefer graceful `adl agent stop --spec ... --reason ... --json`.
   - Avoid uncontrolled restarts when checkpoint, custody, or credential
     evidence is suspect.
4. Preserve:
   - Retain `daemon_status.json`, `status.json`, `continuity_checkpoint.json`,
     `continuity_replay_manifest.json`, `cycle_ledger.jsonl`,
     `operator_events.jsonl`, OTel status/logs, runtime API response samples,
     and wrapper summaries.
5. Serialize:
   - Use continuity capture/stage when present. If unavailable, record
     `continuity_command_unavailable` and preserve the last readable state root.
6. Restore or degrade:
   - Restore only from a valid continuity capsule or known-good retained state.
   - For provider, cloud, storage, or telemetry failures, degrade the affected
     path without claiming full Polis readiness.
7. Verify:
   - Re-check `/health`, `/ready`, OTel status/log retention, checkpoint
     freshness, and any issue-specific proof command.
8. Communicate:
   - Publish only redacted, repo-relative evidence. Name non-claims plainly.
9. Close out:
   - Write the post-incident record template below and link retained artifacts.

## Runbooks

### Daemon Not Reachable

Detect:

```sh
adl process status --pid-file <runtime-root>/state/daemon.pid --json
adl process status --port <runtime-api-port> --json
```

Classify:

- If the pid/port is unreachable but `daemon_status.json` completed normally,
  classify as `runtime_stopped_recoverable`.
- If `daemon_status.json` includes restart-budget exhaustion, classify as
  `runtime_degraded_terminal_for_current_budget`.
- If no daemon artifacts exist, classify as `runtime_liveness_unknown`.

Stabilize and preserve:

```sh
adl agent status --spec <agent-spec.yaml> --json
adl agent stop --spec <agent-spec.yaml> --reason "incident stabilization" --json
```

Evidence checklist:

- `state/daemon_status.json`
- `state/status.json`
- `state/operator_events.jsonl`
- process-status JSON
- runtime API `/health` and `/ready` samples if available

Non-claim: this packet does not implement OS service-manager restart across
host reboot or forceful termination.

### Checkpoint Stale Or Missing

Detect:

```sh
csm api serve --spec <agent-spec.yaml> --bind 127.0.0.1:0 --json
```

Then query `/health` and `/ready` from the printed loopback address.

Classify:

- `/health` degraded with checkpoint blockers: `continuity_risk`.
- `/ready` not ready with `continuity_checkpoint_missing`:
  `continuity_not_ready`.

Stabilize:

- Stop admitting new runtime work.
- Preserve the last `continuity_checkpoint.json` and replay manifest.
- Capture a continuity capsule if the continuity command is available.

Verify:

- The next checkpoint is newer than the incident start time.
- `continuity_replay_manifest.json` references readable replay material.
- `/ready` clears checkpoint blockers before resuming normal work.

### OTel Or Observability Degraded

Detect:

- Missing or failed `otel_status.json`.
- Missing `otel.jsonl` when expected.
- Runtime API `/events` cannot return retained operator events.

Stabilize:

- Preserve local `observability.log`, `otel.jsonl`, `otel_status.json`, and
  `operator_events.jsonl`.
- Keep machine-readable command output on stdout and human `adl_event`
  observability on stderr or configured compatibility log.

Verify:

- Event classes are either retained locally or explicitly non-claimed.
- API responses redact host-private paths, secrets, authorization material,
  AWS ARNs, and cloud account identifiers.

Non-claim: hosted telemetry backend readiness is outside this packet unless a
later issue proves it.

### Continuity Capsule Invalid Or Restore Fails

Detect:

```sh
csm continuity stage --bundle <bundle-dir> --out <stage-dir>
csm continuity restore --bundle <bundle-dir> --out <runtime-dir>
```

Classify:

- Manifest version mismatch: `portable_state_schema_mismatch`.
- Missing file: `portable_state_incomplete`.
- Path or credential leakage: `portable_state_rejected_for_hygiene`.
- Unsupported target host: `portable_state_target_unsupported`.

Stabilize:

- Do not restore from a failed capsule.
- Preserve the capsule manifest, stage report, restore report, and negative
  result artifact.
- If local runtime remains recoverable, capture a fresh capsule from the last
  known-good state root.

Verify:

- `restore_report.json` exists for a successful restore.
- Restored `csm daemon` fire-up evidence exists before claiming migration
  success.
- EC2/Spot claims cite the redacted Agent Logic AWS proof packet, not local
  staging alone.

### Cloud Hook Failure

Detect:

- AWS wrapper summary reports failure for CodeBuild, Spot, SSM, SNS, storage,
  or CloudFront-adjacent hooks.
- Profile check fails for `agent-logic-admin`.

Stabilize:

- Preserve wrapper summaries and redacted log references.
- Keep local CSM runtime status separate from cloud-control-plane status.
- Do not fall back to a personal/default AWS profile.

Verify:

- The profile was checked before relying on AWS state.
- No credential or account identifier appears in committed evidence.
- Local runtime recoverability is documented independently from cloud hook
  success.

### Suspected Tamper Or Credential Compromise

Detect:

- CAV/red-blue evidence reports unsafe tactic, custody mismatch, unauthorized
  artifact mutation, credential-like key, or redaction failure.

Stabilize:

- Stop new writes where possible.
- Preserve the exact redacted evidence packet.
- Route to the active CAV/security and break-glass owners.
- Do not attempt capsule restore until custody and credential posture are
  classified.

Verify:

- Red/blue finding disposition exists.
- Credential rotation/break-glass policy owner has accepted the incident.
- Post-incident record contains no exposed secret, token, provider key, AWS
  credential, account id, or host-private path.

### Provider Outage, Overload, Or Backpressure

Detect:

- Runtime resilience trace records timeout, cancellation, auth/quota/policy
  terminal classification, rate/backpressure, bulkhead, degraded fallback, or
  scheduler queued-backpressure.

Stabilize:

- Stop nonessential provider work.
- Let AEE degraded fallback policy handle bounded continue-on-error paths.
- Do not convert terminal auth/quota/policy failures into unbounded retries.

Verify:

- Trace envelope, action log, normalized trace, and ObsMem index retain the
  resilience disposition.
- `/metrics` or retained runtime artifacts show the queue/backpressure state
  has cleared before normal admission resumes.

## Dry-Run Evidence Check

This issue dry-runs the `checkpoint stale or missing` and `continuity capsule`
incident paths against retained local evidence rather than inducing a new live
runtime incident:

```sh
test -f docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json
test -f docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/aws_remote_restore_fireup_summary.json
test -f docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/proof_summary.json
test -f docs/milestones/v0.91.7/review/runtime/soak2_4682/evidence_index.json
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json >/dev/null
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/proof_summary.json >/dev/null
```

Expected result: all files exist and JSON proof summaries parse. This validates
that the runbook references retained evidence for continuity and no-sparrow
observability. It does not prove a new live incident recovery.

## Build Platform Timing

Comparable benchmark command:

```sh
bash adl/tools/run_build_platform_benchmark.sh --platform wuji --cache-posture local_warm_target --out .adl/local-artifacts/build-platform/4922-wuji-summary.json --artifact-dir .adl/local-artifacts/build-platform/4922-wuji
```

Current rows:

| Platform | Cache posture | Build | Test | Benchmark total | Wrapper wall | Status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `wuji` | `local_warm_target` | `107s` | `107s` | `214s` | `214s` | passed |
| `nessus` | `persistent_remote_target_cache` | `79s` | `46s` | `125s` | `132s` | passed via direct Nessus runner |
| `codebuild` | `fixed_builder_image_stable_local_target_cache_s3_sccache` | `98s` | `76s` | `174s` | `201s` | passed |
| `aws_spot` | `fixed_builder_image_warm_ebs_cache` | not_run | not_run | not_run | not_run | failed before launch |

Observed speed note: the docs-only #4922 issue still paid a cold worktree
build/test-profile cost locally, so the benchmark is dominated by Rust profile
warmth rather than changed-file size. The #4929 local comparison row was
`93s` total; #4922's first local row is `214s`. Nessus was the fastest
completed row for this issue at `125s` benchmark total, while CodeBuild was
close at `174s` and reported a high sccache hit rate.

Remote-build problems observed:

- Validation-manager rejected the Nessus lane for this docs-only issue because
  the selected runtime class was `tiny` / `docs_diff_check_profile`; the
  explicit benchmark was run through the lower-level repo-native Nessus runner
  instead.
- AWS Spot account profile resolution passed, but the wrapper failed before
  launch because the required warmed
  `adl-aws-remote-validation` repo binary was not present in the issue
  worktree. This packet does not claim a Spot benchmark row for #4922.

## Post-Incident Record Template

```yaml
incident_id: "<date>-<short-id>"
opened_at: "<rfc3339>"
closed_at: "<rfc3339-or-open>"
commander: "<operator>"
classification: "<taxonomy-classification>"
recoverable_state: "<recoverable|degraded|terminal_for_current_budget|security_incident|non_claim>"
affected_surfaces:
  - "<csm-daemon|runtime-api|otel|continuity|cloud-hook|provider|security|storage>"
signals:
  process_status_ref: "<repo-relative-artifact-or-not_available>"
  runtime_api_status_ref: "<repo-relative-artifact-or-not_available>"
  runtime_api_health_ref: "<repo-relative-artifact-or-not_available>"
  runtime_api_ready_ref: "<repo-relative-artifact-or-not_available>"
  otel_status_ref: "<repo-relative-artifact-or-not_available>"
  operator_events_ref: "<repo-relative-artifact-or-not_available>"
state_artifacts:
  daemon_status_ref: "<repo-relative-artifact-or-not_available>"
  agent_status_ref: "<repo-relative-artifact-or-not_available>"
  checkpoint_ref: "<repo-relative-artifact-or-not_available>"
  replay_manifest_ref: "<repo-relative-artifact-or-not_available>"
  continuity_capsule_ref: "<repo-relative-artifact-or-not_available>"
actions:
  - command: "<repo-native-command-or-not_run>"
    purpose: "<what it checked or changed>"
    result: "<pass|fail|degraded|not_run>"
non_claims:
  - "<capability explicitly not proven by this incident>"
security_privacy:
  redaction_checked: "<true|false>"
  credential_exposure_detected: "<true|false>"
  cav_route_required: "<true|false>"
  cav_route_ref: "<repo-relative-artifact-or-not_applicable>"
cloud:
  aws_profile: "agent-logic-admin-or-not_applicable"
  cloud_artifact_ref: "<repo-relative-artifact-or-not_applicable>"
closeout:
  readiness_restored: "<true|false|partial>"
  follow_up_issue: "<issue-or-not_applicable>"
  evidence_index_ref: "<repo-relative-artifact>"
```

## Reviewer Notes

- This packet is a runbook and evidence-index artifact, not proof that every
  underlying survival feature is complete.
- Commands are intentionally limited to surfaced repo-native commands and
  current evidence packets.
- Where an issue is scheduled but no retained proof packet exists in this
  checkout, the runbook records a non-claim and routes the incident to that
  owner rather than inventing behavior.

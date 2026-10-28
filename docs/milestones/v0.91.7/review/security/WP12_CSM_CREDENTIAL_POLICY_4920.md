# WP-12 CSM Credential Rotation And Break-Glass Policy

Issue: #4920
Milestone: v0.91.7
Status: implemented proof surface

## Policy Summary

CSM credential handling is class-based. The v0.91.7 proof records credential classes, evidence requirements, rotation triggers, break-glass constraints, audit events, and failed-closed negative cases without retaining secret values, raw credential paths, raw AWS account ids, or raw operator identity.

Credential classes covered:

- `csm_aws_control_plane`
- `csm_polis_storage`
- `csm_observability_exporters`
- `csm_custody_signing_keys`

Default rotation cadence is 30 days. Emergency rotation deadline is 15 minutes after a rotation-triggering denial, revocation, or account/key mismatch. Runtime fallback is degraded operation with safe-fail artifacts until rebind evidence exists.

Break-glass requires explicit approval, bounded scope, audit start/denial/revocation events, and revocation before return to normal operation. Maximum break-glass duration is 30 minutes. Forbidden actions include printing secrets, copying credential files, committing secret material, bypassing custody validation, or unbounded cloud mutation.

## Implemented Checks

The standalone CSM runtime now exposes:

```text
csm credential-policy prove --out <proof-dir> [--run-id <id>] [--operator <identity>] [--requested-at <RFC3339>] [--json]
```

The command writes:

- `credential_policy_summary.json`
- `credential_lifecycle_events.jsonl`

The proof simulates and records these failed-closed negative cases:

- missing credential
- expired credential
- denied break-glass
- stale binding

## Retained Evidence

Retained local evidence for this issue:

- `docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_summary.json`
- `docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_lifecycle_events.jsonl`
- `docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_stdout.json`
- `docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_observability.log`

The retained proof was generated with:

```text
ADL_OBSERVABILITY_STDERR=0 ADL_OBSERVABILITY_LOG=docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_observability.log adl/target/debug/csm credential-policy prove --out docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920 --run-id wp12-4920-retained --operator local-operator --requested-at 2026-07-10T00:00:00Z --json
```

## Non-Claims

This issue does not rotate live provider, AWS, storage, OTel, or payment credentials. It does not grant broad cloud mutation authority. It does not implement x402 settlement or payment flows; x402 remains in later-milestone planning.

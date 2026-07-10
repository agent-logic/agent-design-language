# CSM Continuity Capsule Proof (#4910)

This packet retains a bounded WP-07 proof for `csm continuity capture` and
`csm continuity stage`.

Evidence:
- `capsule/continuity_capsule_manifest.json` is the portable capsule manifest.
- `capsule/custody_manifest.json` is the signed RustCrypto P-256/ECDSA custody
  manifest for retained capsule artifacts and binary segments.
- `capsule/state/` contains retained CSM runtime state: identity/spec, status,
  daemon status, continuity checkpoint, replay manifest, cycle ledger, memory
  index, provider binding history, operator events, and cycle artifacts.
- `ec2_staged/stage_report.json` proves local EC2-staging validation.
- `ec2_blocked/stage_report.json` records the live EC2 transfer boundary and
  required `agent-logic-admin` business AWS profile.
- `ec2_restored/restore_report.json` proves capsule restore into a runtime root,
  and `logs/restored_daemon_stdout.json` proves `csm daemon` fired from the
  restored spec/state.
- `aws_remote_restore_fireup_summary.json` records the redacted Agent Logic AWS
  proof that the same restore/fire-up lane passed on an EC2 Spot builder.
- `negative_results.json` records version mismatch, missing file, missing
  custody manifest, custody signature tampering, untrusted custody public key,
  path leakage, credential-like key, corrupted manifest, and unsupported
  target-host rejection.
- `logs/observability.log`, `logs/otel.jsonl`, and `logs/otel_status.json`
  retain runtime observability for daemon, capture, stage, and restore events.

Truth boundary: this proves portable capture, staging, restore, and restored
daemon fire-up of current CSM runtime state, including an EC2 restore/fire-up run in the Agent Logic business AWS account. It does not claim provider-secret
export or production multi-region disaster recovery.

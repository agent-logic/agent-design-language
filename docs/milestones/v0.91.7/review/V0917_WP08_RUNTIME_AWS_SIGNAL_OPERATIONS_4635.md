# WP-08 Runtime AWS And Signal Operations Packet for `#4635`

Status: `complete_child_issues_merged`
Issue: `#4635`
Date: 2026-07-07

## Scope

This packet records the WP-08 umbrella evidence for runtime AWS and signal
operations. It consumes child proof surfaces rather than re-running every child
proof from the umbrella issue.

The merged child issues establish heartbeat, ACIP/SNS, AWS signal integration,
local polis SSM, S3 archive policy, durable CSM/polis storage, and
CloudFront/control-plane hooks. A later WP-07/WP-08 bridge tail, `#4998`,
adds retained CSM governed shutdown/degradation notice proof through
CloudWatch, ACIP/SNS, and EventBridge/Lambda control-plane delivery. This
umbrella packet records the retained proof surfaces consumed by `#4635` plus
that adjacent bridge-tail proof; it does not rerun each live AWS proof.

## Child Issue State

| Issue | Scope | Current truth | Retained proof |
| --- | --- | --- | --- |
| `#4684` | Heartbeat publisher | merged with live CloudWatch proof | `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json` |
| `#4685` | ACIP to SNS | merged with live SNS publish proof | `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json` |
| `#4686` | AWS signal integration | merged with integrated heartbeat plus ACIP/SNS proof | `docs/milestones/v0.91.7/review/runtime/wp08_aws_signal_integration_4686/aws_signal_integration_summary.json` |
| `#4687` | Local polis SSM operations | merged with live SSM proof for wuji, nessus, and opticon | `docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json` |
| `#4688` | S3 ObsMem archive policy | merged with live S3 bucket policy proof | `docs/milestones/v0.91.7/review/runtime/wp08_s3_obsmem_archive_4688/archive_policy_summary.json` |
| `#4913` | Durable CSM/polis storage | merged with live S3 write/read/restore and negative-case proof | `docs/milestones/v0.91.7/review/runtime/csm_polis_storage_4913/polis_storage_proof_summary.json` |
| `#4915` | CloudFront/control-plane hooks | merged with live CloudFront status proof and green PR checks | `docs/milestones/v0.91.7/review/runtime/wp08_cloudfront_4915/cloudfront_status_summary.json` |

## Adjacent Bridge-Tail Proof

| Issue | Scope | Current truth | Retained proof |
| --- | --- | --- | --- |
| `#4998` | WP-07/WP-08 CSM governed shutdown/degradation notices | closed after PR `#5016` merged with `adl-ci` and `adl-coverage` green; repo-native closeout validated STP/SIP/SOR | `docs/milestones/v0.91.7/review/runtime/csm_governed_notice_4998/` |

## Merged Capability Evidence

The landed WP-08 child issues currently prove:

- Runtime heartbeat publication to CloudWatch Logs using the Agent Logic AWS
  profile, with redacted retained summary and no credential retention.
- ACIP projection to SNS with retained SNS message id, redacted topic/account
  references, and negative-case policy for missing profile, missing topic,
  denied projection, and publish failure.
- Integrated AWS signal proof that combines heartbeat and ACIP/SNS under the
  same Agent Logic account hash and records expected-account verification.
- Local polis SSM operations against wuji, nessus, and opticon through AWS SSM,
  with CloudWatch output observed and raw command/instance ids redacted.
- S3 ObsMem/community-memory archive policy with versioning, public access
  block, SSE-S3 encryption, governance object lock, and lifecycle transitions.
- Durable CSM/polis storage using that archive bucket, with live object
  write/read/restore, checksum validation, object-lock metadata, unsigned access
  denial, missing-object, and corrupted-restore negative cases.
- CloudFront/control-plane status through `csm cloud-control cloudfront-status`,
  with live Agent Logic AWS proof, retained distribution metadata, redacted
  account identity, and command evidence in the CSM runtime path.
- CSM governed shutdown/degradation notices through retained local notice
  records plus live CloudWatch Logs, ACIP/SNS, and EventBridge/Lambda
  delivery proof for the bounded `#4998` bridge-tail path.

## CloudFront Proof

Issue `#4915` / PR `#4994` adds the `csm cloud-control cloudfront-status`
runtime path and retained CloudFront proof. On 2026-07-07 the proof ran with
`AWS_PROFILE=agent-logic-admin` and the Agent Logic account SHA guard. The
retained account hash uses the same 16-character account-SHA prefix convention
as the other WP-08 AWS proofs.

Repo-native validation for PR `#4994` reported `pr_state: MERGED`,
`projection_status: merged`, `adl-ci: SUCCESS`, `adl-coverage: SUCCESS`, and
`adl-slow-proof: SKIPPED` for head
`9c12793459f84d6c22b3a97f5161f554cd3b8e78`.

## Validation Evidence

Merged child proof validators and issue-local checks are recorded in the child
issues and proof packets. Umbrella preparation checks run so far:

```text
adl tooling validate-structured-prompt --type spp --input .adl/v0.91.7/tasks/issue-4635__v0-91-7-wp-08-implement-runtime-aws-and-signal-operations-in-full/spp.md
adl tooling validate-structured-prompt --type vpp --input .adl/v0.91.7/tasks/issue-4635__v0-91-7-wp-08-implement-runtime-aws-and-signal-operations-in-full/vpp.md
adl pr doctor 4635 --slug v0-91-7-wp-08-implement-runtime-aws-and-signal-operations-in-full --version v0.91.7 --allow-open-pr-wave --json
```

The earlier doctor result was a pre-publication readiness check, not final
closeout proof. After the CloudFront PR merged, the umbrella SOR was refreshed
from scaffold truth to publication-ready truth for this packet.

Final umbrella refresh also consumed repo-native `adl pr validation 4994
--json`, which reported PR `#4994` merged and all required checks resolved. A
subagent pre-PR review found stale lifecycle wording and SOR integration wording
that could read as terminal main-repo truth; both were corrected before
publication.

After the original WP-08 umbrella closeout, bridge-tail issue `#4998` / PR
`#5016` merged with `adl-ci: SUCCESS`, `adl-coverage: SUCCESS`, and
`adl-slow-proof: SKIPPED` at head
`987297982ccdd0e24d6c730f317cb14c2e4b2ae1`. Repo-native `pr.sh closeout
4998 --version v0.91.7` then validated STP, SIP, and SOR and found the issue
worktree already absent, so no prune was needed.

## Non-Claims

- This packet does not claim paid AWS proof runs automatically in ordinary CI.
- This packet does not claim SSM owns polis state or governance authority.
- This packet does not claim mathematical 12-nines durability from the selected
  single-region S3 backend.
- This packet does not claim public/production ACIP signal routing beyond the
  bounded live SNS proof and redaction policy recorded by the child issue.
- This packet does not claim WP-07 release readiness from the adjacent `#4998`
  bridge-tail proof.

# WP-08 Findings Review

Status: review_findings_record
Issue: #5143
WP: WP-08
Umbrella: #4635
Date: 2026-07-10

## Scope

This document records WP-08 review findings from the runtime AWS/signal child
issues and the adjacent WP-07/WP-08 CSM governed-notice bridge tail. It indexes
retained findings and dispositions; it does not rerun paid AWS proof and does
not widen WP-08 into WP-07 release readiness.

Source note: several fine-grained review findings are recorded in operational
SRP/SOR cards under `.adl/` in the primary checkout rather than in this issue
worktree. Those findings are paired below with merged PR URLs or tracked proof
packets so this review document remains portable.

## Findings

| ID | Severity | Status | Finding | Evidence | Disposition |
| --- | --- | --- | --- | --- | --- |
| WP08-F-001 | P2 | fixed | The heartbeat redaction proof could falsely pass for the real AWS account id because the wrapper checked only the fixture account id in observability output. | Operational SRP/SOR cards for `#4684`; merged PR [#4966](https://github.com/danielbaustin/agent-design-language/pull/4966); `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json` | Fixed by passing the actual account id only to local summary generation, checking observability and fetched CloudWatch JSON for that value, and retaining only boolean redaction results plus the account hash. |
| WP08-F-002 | P3 | fixed | The heartbeat proof wrapper reused retained state/output, so repeated live proofs could produce mixed artifacts. | Operational SRP/SOR cards for `#4684`; merged PR [#4966](https://github.com/danielbaustin/agent-design-language/pull/4966); `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json` | Fixed by writing run-scoped state under `state/<run-id>/`, using run-scoped observability logs, clearing current summary/stdout/stderr/event files before each proof, and regenerating retained proof. |
| WP08-F-003 | P3 | fixed | The heartbeat `--cleanup` path deleted the shared issue log group instead of only the run-specific stream. | Operational SRP/SOR cards for `#4684`; merged PR [#4966](https://github.com/danielbaustin/agent-design-language/pull/4966) | Fixed so cleanup deletes only the run-specific CloudWatch log stream and preserves the bounded issue log group under the recorded retention policy. |
| WP08-F-004 | P2 | fixed | The ACIP/SNS live wrapper needed a stronger approved Agent Logic account-hash guard before SNS mutation, and cleanup needed an exit trap after SNS topic creation. | Operational SRP card for `#4685`; merged PR [#4975](https://github.com/danielbaustin/agent-design-language/pull/4975); `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json` | Fixed before PR publication: the live wrapper verifies the selected profile against an approved full Agent Logic account SHA-256 before mutation; cleanup registers an exit trap after topic creation; finish validation checks both retained summaries. |
| WP08-F-005 | P1 | fixed | The local polis SSM HOW-TO self-authorized the account hash by deriving it from the checked profile, weakening the intended operator-approved account guard. | Operational SOR card for `#4687`; merged PR [#4978](https://github.com/danielbaustin/agent-design-language/pull/4978); `docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json` | Fixed by requiring operator-approved `ADL_AWS_LOCAL_POLIS_SSM_ACCOUNT_SHA256`. |
| WP08-F-006 | P2 | fixed | The local polis SSM mismatch test did not prove that no SSM mutation occurred after account mismatch. | Operational SOR card for `#4687`; merged PR [#4978](https://github.com/danielbaustin/agent-design-language/pull/4978) | Fixed by clearing the fake AWS log before mismatch and asserting no SSM or CloudWatch discovery calls occur after mismatch. |
| WP08-F-007 | P2 | fixed | The S3 archive validation selector classified the archive setup lane as runtime-owned capability proof even though it is a provisioning/control-plane helper. | Operational SRP/SOR cards for `#4688`; merged PR [#4982](https://github.com/danielbaustin/agent-design-language/pull/4982); `docs/milestones/v0.91.7/review/runtime/wp08_s3_obsmem_archive_4688/archive_policy_summary.json` | Fixed by classifying the lane as tooling-owned AWS storage provisioning policy proof. |
| WP08-F-008 | P2 | fixed | The #4998 Lambda/EventBridge live control-plane transports parsed `ADL_AWS_PROFILE` / `AWS_PROFILE` but loaded AWS SDK config without binding the selected profile, so a run could pass profile/account guardrails while publishing with ambient default credentials. | Operational SRP/SOR cards for `#4998`; merged PR [#5016](https://github.com/danielbaustin/agent-design-language/pull/5016); `docs/milestones/v0.91.7/review/runtime/csm_governed_notice_4998/` | Fixed in rebased commit `491e0e2e3` by routing Lambda and EventBridge through a shared control-plane AWS config loader that applies region, timeout config, and `.profile_name(...)` when configured. Focused `csm_control_plane_notice` checks and status validation passed. |

## Retained Proof And Boundaries

The umbrella packet
`docs/milestones/v0.91.7/review/V0917_WP08_RUNTIME_AWS_SIGNAL_OPERATIONS_4635.md`
records the retained proof surface for heartbeat, ACIP/SNS, integrated AWS
signal, local polis SSM, S3 archive policy, durable CSM/polis storage,
CloudFront/control-plane status, and adjacent #4998 CSM governed notices.

These findings do not change the recorded WP-08 non-claims:

- paid AWS proof does not run automatically in ordinary CI;
- SSM does not own polis state or governance authority;
- the selected single-region S3 backend is not a mathematical 12-nines proof;
- bounded live SNS proof is not public/production ACIP routing;
- #4998 bridge-tail proof does not establish WP-07 release readiness.

## Review Conclusion

WP-08 has multiple fixed review findings, with the highest-severity fixed item
being the local polis SSM operator-account guard. The retained AWS proof surface
is useful, but release consumption must keep the live-AWS proof boundaries and
the WP-07 non-claim visible.

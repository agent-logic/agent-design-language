# WP-12 SSM Readiness Proof (#4657)

## Metadata

- Issue: `#4657`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: SSM operations readiness proven from retained live evidence
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json`
- Validator: `adl/tools/validate_wp12_ssm_readiness_4657.py`

## Purpose

Prove the WP-12 SSM readiness row for the pre-`v0.92` activation gate without
expanding into sibling protocol, CAV, custody, or access-rule work.

This packet consumes the retained live WP-08 local-polis SSM proof from `#4687`
and records the WP-12 disposition for `#4657`. The proof covers the intended
business AWS profile, account identity hash, managed-node identity, online SSM
status, successful command execution, CloudWatch observability, and redaction
boundaries.

## Evidence Consumed

- `docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json`
- `docs/adr/0035-local-polis-ssm-operations-boundary.md`
- `docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json`
- `docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json`

## Readiness Result

| Requirement | Result | Evidence |
| --- | --- | --- |
| AWS access profile | Passed | `agent-logic-admin` in `us-west-2`; account hash matches retained proof. |
| Managed-node identity | Passed | `wuji`, `nessus`, and `opticon` are retained as the required local-polis host set. |
| SSM online status | Passed | Each host has `ssm_ping_status: Online`. |
| Command execution | Passed | Each host has `command_status: Success` and status schema `adl.local_polis_status.v1`. |
| Observable status evidence | Passed | CloudWatch output was enabled and stream hashes were retained. |
| Redaction boundary | Passed | Raw account ids, instance ids, command ids, and AWS credentials are not retained. |

## Gate Disposition

The `ssm_and_local_polis_secret_readiness` row in
`docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json` now
records:

- owner issue: `#4657`
- state: `integrated_proven`
- v0.92 disposition: `supports_ssm_operations_claims`
- evidence: this packet, the machine-readable readiness summary, the #4687
  retained live proof, ADR 0035, and the validator.

## Failure Modes Preserved

- Account-hash mismatch fails before SSM mutation.
- Missing expected account hash fails before SSM mutation.
- Missing online managed node fails before `send-command`.
- Command timeout or failure records failed summary status and per-host terminal
  command status.

## Non-Claims

- SSM remains operations-plane only and does not own polis state or governance
  authority.
- This packet does not claim provider/model execution through SSM.
- This packet does not claim unattended runtime mutation authority.
- This packet does not retain or expose secret values, credentials, raw account
  ids, raw instance ids, or raw command ids.

## Validation

Focused local validation:

```sh
python3 adl/tools/validate_wp12_ssm_readiness_4657.py \
  --source-summary docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json \
  --readiness-summary docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json \
  --gate docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json

bash adl/tools/test_validate_wp12_ssm_readiness_4657.sh
git diff --check
```

These checks prove the retained live evidence is parseable, redacted,
business-account-bound, host-complete, observable through CloudWatch, and wired
into the WP-12 gate row.

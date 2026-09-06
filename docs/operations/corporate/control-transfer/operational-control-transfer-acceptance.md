# Corporate Operational-Control Transfer Acceptance

- Issue: #497
- Sprint umbrella: #532
- Machine-readable packet: `docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json`
- Evidence directory: `docs/milestones/v0.92.1/evidence/corporate/corp-c/`

## Decision

CORP-C is accepted for #497 corporate IP-transfer acceptance.

The #497 packet now separates corporate IP-transfer acceptance from broader
post-move-in operational control-plane hardening. The hardening rows discovered
during closeout remain important and truthful evidence, but they are sidecar
scope in #624 rather than blockers for #497.

This packet must not be read as claiming that GitHub/CI, DNS/certificate,
deployment rollback, AWS account guardrails, or private custody hardening is
complete. It records that those items no longer block #497 transfer acceptance.

## Prerequisite Gate

The prerequisite gate passes for #497:

| Lane | Issue | Closing PR | Merge commit | Ancestral to `origin/main` |
| --- | ---: | ---: | --- | --- |
| CORP-A critical-asset schedule | #482 | #545 | `e2c1d1649b0c930a5a1254575a07ef2a4496d48d` | yes |
| CORP-B corporate account custody register | #483 | #562 | `4a0b49c0071bacdaab19d6d9eb8c44380beb51be` | yes |
| GCP-D private platform foundation | #493 | #587 | `c0bf217934508d6dbc70d78633e6a95d5ddd9d06` | yes |
| AWS-G CloudFormation retirement decision | #496 | #599 | `83077ca029d52c9d613ed5a373da30f1dd42d9b3` | yes |

Evidence: `docs/milestones/v0.92.1/evidence/corporate/corp-c/prerequisite-ancestry.v1.json`.

## Current Evidence

- `agent-logic-admin` STS identity was read back without mutation and retained
  as a redacted hash-only receipt:
  `docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-identity-readback-redacted.v1.json`.
- That account hash matches retained Agent Logic AWS evidence from
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/`.
- Repository IaC source custody and some domain-registration custody are present
  through CORP-A/CORP-B evidence.
- AWS-G retains CloudFormation rollback/source-denominator evidence and does
  not authorize deletion or live-stack retirement.

## Sidecar Operational-Hardening Rows

These rows are tracked by sidecar issue #624 and do not block #497:

| Row | Blocking proof |
| --- | --- |
| Source control | GitHub organization owner roster, billing plan owner, MFA, repository recovery, and emergency access readbacks are missing. |
| CI/CD | Actions billing, environments, required reviewers, runner authority, and emergency workflow-disable readbacks are missing. |
| DNS / Route53 | Hosted-zone ownership, delegation, change-freeze, rollback, and DNS recovery readbacks are missing. |
| Certificates | Certificate inventory, renewal owner, revocation/reissue, and recovery readbacks are missing. |
| AWS account control | Billing, root/contact posture, IAM Identity Center/MFA, audit/break-glass, and recovery readbacks remain unproven beyond STS account targeting. |
| Deployment operations | Deployment roles, rollback authority, incident audit logging, emergency access, and recovery drill readbacks are missing. |
| Private custody | Redacted company-vault and non-single-founder recovery receipt is missing. |

Machine-readable sidecar row evidence:
`docs/milestones/v0.92.1/evidence/corporate/corp-c/control-plane-denominator.v1.json`.

## Acceptance Status

| #497 acceptance criterion | Status |
| --- | --- |
| Each control plane has corporate owner and rollback | accepted for IP-transfer scope; operational hardening continues in #624 |
| AWS uses the approved business account | accepted for IP-transfer scope through redacted `agent-logic-admin` account-hash readback; broader AWS account hardening continues in #624 |
| Terraform and CI authority are company-controlled | accepted for IP-transfer scope; CI and downstream deployment hardening continues in #624 |
| Availability and recovery readbacks pass | accepted for IP-transfer scope; Runtime origin-smoke/recovery hardening continues in #624 |

## Authority Boundary

No production/provider mutation, billing change, credential transfer, DNS
change, certificate action, workflow mutation, or private custody transfer was
performed by #497.

If satisfying a sidecar hardening row requires mutation, paid provider work, or
private custody access, that work needs explicit operator authorization naming
the exact action. Without that authority, #624 remains incomplete; #497 remains
accepted for corporate IP-transfer scope.

This packet does not mean:

- sidecar issue #624 is complete;
- live provider cutover is complete;
- cloud parity and billing custody are globally proved;
- private legal, diligence, vault, recovery-factor, payment-method, or executed
  instrument material is committed to the repository;
- Sprint 7 #345 AWS GPU execution is performed;
- Sprint 8 #84 Unity work is performed;
- CORP-D #498 diligence acceptance is performed.

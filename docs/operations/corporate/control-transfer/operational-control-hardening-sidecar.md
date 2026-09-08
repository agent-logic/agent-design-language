# Corporate Operational-Control Hardening Sidecar

- Issue: #624
- Parent acceptance issue: #497
- Parent acceptance PR: #613
- Sidecar routing PR: #634
- Machine-readable receipt: `docs/milestones/v0.92.1/evidence/corporate/corp-sidecar-624/operational-control-hardening.v1.json`

## Decision

Issue #624 records the post-move-in operational-control hardening denominator
that was explicitly split out of #497. The #497 corporate IP-transfer
acceptance remains accepted. This sidecar does not reopen it and does not claim
that live GitHub, CI, DNS, certificate, AWS guardrail, custody, billing, or
deployment settings have been changed.

The public repository can safely retain the accounting layer: each hardening row
has either read-only retained evidence or a concrete follow-on owner/action with
an explicit authority gate. Rows that require live account access, private
custody access, paid cloud activity, provider mutation, or administrative
settings remain proposed operational work until the operator separately
authorizes that exact action.

## Denominator

| Row | Category | Status | Owner role | Next action | Authority gate | Closeout condition |
| --- | --- | --- | --- | --- | --- | --- |
| GH-ORG-RECOVERY | GitHub / CI | follow-on required | Corporate platform operator | Read back organization owner roster, billing-plan owner, MFA posture, recovery path, and emergency access without exposing principals beyond redacted role labels. | Explicit operator authorization for GitHub organization/account readback; no mutation by this issue. | Redacted receipt shows company-controlled recovery does not depend on one personal factor. |
| GH-CI-GUARDRAILS | GitHub / CI | follow-on required | Corporate platform operator | Verify Actions policy, required reviewers, environment protection, runner authority, emergency workflow-disable procedure, and billing custody. | Explicit operator authorization for GitHub CI/admin readback; no workflow mutation by this issue. | CI authority receipt lists checks/environments/runners and emergency disable owner. |
| DNS-DELEGATION | DNS / certificate | follow-on required | Domain operations owner | Verify registrar custody, Route53 delegation, change-freeze, rollback owner, and DNS recovery path. | Explicit operator authorization for registrar/DNS readback; no DNS mutation by this issue. | Redacted DNS custody receipt binds registrar, hosted-zone, delegation, rollback, and recovery roles. |
| CERT-RENEWAL | DNS / certificate | follow-on required | Domain operations owner | Verify ACM or issuer renewal owner, validation method, revocation/reissue authority, and recovery route for public certificates. | Explicit operator authorization for certificate inventory/readback; no certificate mutation by this issue. | Certificate receipt covers renewal, revocation/reissue, validation ownership, and recovery roles. |
| AWS-AUDIT-GUARDRAILS | AWS guardrails | follow-on required | Cloud account owner | Verify billing/contact custody, root posture, IAM Identity Center or equivalent MFA, CloudTrail, AWS Config, IAM Access Analyzer, anomaly/budget visibility, recovery, and break-glass. | Explicit operator authorization for AWS account-control readback using approved business profile; no AWS mutation by this issue. | Redacted AWS guardrail receipt records each guardrail pass/fail/deferred status with no account identifiers. |
| DEPLOY-ROLLBACK | Deployment rollback | follow-on required | Release operations owner | Verify deployment roles, rollback authority, incident audit logging, emergency access, and a recovery drill or approved dry-run. | Explicit operator authorization for deployment/rollback readback or rehearsal; no deployment mutation by this issue. | Rollback receipt proves authorized operator can restore an approved prior posture or records a narrower blocker. |
| PRIVATE-CUSTODY | Private custody | follow-on required | Corporate custody owner | Produce a redacted receipt that company vault/recovery is not controlled by one founder-local factor. | Explicit operator authorization for private custody readback; private artifacts remain outside the repository. | Public receipt contains only role-level custody status, digest references, and non-single-factor recovery result. |

## Existing Seed Evidence

The following existing records are retained as read-only seed evidence, not as
proof that #624 hardening has already been applied:

- `docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.md`
- `docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-c/control-plane-denominator.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-c/github-ci-authority-readback.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-c/dns-cert-deployment-readback.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-account-control-readback.v1.json`
- `docs/milestones/v0.92.1/evidence/corporate/corp-c/live-control-plane-readonly-probe.v1.json`
- `docs/operations/corporate/account-custody/corporate-custody-register.v1.json`

## Separation From #497

#497 accepted the corporate IP-transfer boundary. #624 owns the harder
operational-control-plane follow-through. This packet therefore completes the
public sidecar accounting and routing obligation, while preserving the true
state of the operational work:

- proposed readbacks are not performed here;
- required external mutations are not performed here;
- private custody details are not committed here;
- missing proof remains visible as follow-on operational work;
- each future action must name the exact system, actor, authority, expected
  receipt, redaction rule, and rollback or non-mutation posture before it runs.

## Review And Execution Boundary

An implementation review for #624 must verify that this sidecar record is
complete, redacted, and non-overclaiming. After review, the operator may choose
which follow-on rows to execute under separate scoped authority. This issue does
not grant that authority by itself.

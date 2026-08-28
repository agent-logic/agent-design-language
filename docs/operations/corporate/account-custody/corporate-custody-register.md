# Corporate Account Custody Register

Issue: #483
Sprint umbrella: #529
Register artifact: `docs/operations/corporate/account-custody/corporate-custody-register.v1.json`
Readback evidence: `docs/milestones/v0.92.1/evidence/corporate/corp-b/readback-receipts.v1.json`

No external service mutation was performed for this issue. The register is a
read-only custody and action-list surface: no domain transfer, hosted-zone move,
DNS update, account administrator change, billing change, MFA change, recovery
flow, vault move, break-glass use, secret operation, or infrastructure mutation
was performed for #483.

## Boundary

The original #483 issue text asks for live recovery and break-glass proof. The
operator narrowed this issue to a docs-only register and follow-up list. Rows
marked `follow_up_required` are accepted as truthful register rows, not as
custody-complete service rows.

The completed registration transfers are factual evidence only:

| Domain | Registration owner readback | Hosted zone moved | Notes |
| --- | --- | --- | --- |
| `agent-logic.ai` | Agent Logic AWS profile observed | no | auto-renew was preserved |
| `codefriend.ai` | Agent Logic AWS profile observed | no | auto-renew was preserved |
| `agent-logic.net` | Agent Logic AWS profile observed | no | auto-renew was preserved |
| `aptitude-atlas.com` | Agent Logic AWS profile observed | no | auto-renew was preserved |
| `cognitivespacetimemanifold.com` | Agent Logic AWS profile observed | no | auto-renew was preserved |

`v-dev.ai` and all other `v-*.ai` transfer work remains unscheduled backlog and
is not a v0.92.1 gate in this register.

## Custody Rows

| Row | Source class | Status | Later owner | Action |
| --- | --- | --- | --- | --- |
| `corp-b-source-control` | `source_control` | follow-up required | engineering-maintainer-role | Read back GitHub organization owners, billing plan owner, MFA, recovery contacts, and emergency access. |
| `corp-b-ci-cd` | `ci_cd` | follow-up required | release-maintainer-role | Read back Actions billing, environments, required reviewers, runner authority, and emergency workflow-disable procedure. |
| `corp-b-production-domains` | `production_domains` | follow-up required | CORP-C | Keep registration transfer distinct from hosted-zone migration; keep `v-*.ai` backlog unscheduled. |
| `corp-b-dns-route53` | `dns_route53` | follow-up required | CORP-C | Inventory hosted zones, delegation, rollback, and change freeze before any DNS move. |
| `corp-b-certificates` | `certificates` | follow-up required | CORP-C | Read back certificate inventory, renewal ownership, and rollback before production certificate changes. |
| `corp-b-email` | `email` | follow-up required | business-operations-role | Read back email admin, billing, MFA, recovery contacts, and company-controlled vault references. |
| `corp-b-infrastructure-as-code` | `infrastructure_as_code` | accepted readback | infrastructure-maintainer-role | Repository source custody accepted; live backend/provider authority is covered by service rows. |
| `corp-b-aws-infrastructure` | `aws_infrastructure` | follow-up required | AWS-G | Read back AWS business account billing, root/contact posture, IAM, MFA, audit logging, and break-glass policy. |
| `corp-b-deployment-operations` | `deployment_operations` | follow-up required | release-maintainer-role | Bind deployment roles, rollback authority, incident audit logging, and emergency access to CORP-C. |
| `corp-b-brand-trademark` | `brand_trademark` | follow-up required | business-operations-role | Inventory brand-bearing vendor accounts and route legal conclusions outside repository-private material. |
| `corp-b-provenance-license` | `provenance_license` | accepted readback | engineering-maintainer-role | Repository evidence records provenance/license route; private legal conclusions remain outside repo. |
| `corp-b-assignment-acceptance` | `assignment_acceptance` | follow-up required | business-operations-role | Confirm private originals reside in company-controlled custody using redacted receipt identifiers only. |
| `corp-b-private-custody` | `private_custody` | follow-up required | business-operations-role | Produce redacted vault custody receipt and verify recovery does not depend on one personal factor. |
| `corp-b-data-model-media` | `data_model_media` | accepted readback | engineering-maintainer-role | Repository-controlled documentation, diagrams, and media custody accepted from CORP-A. |

## Short Action List

1. CORP-C owns later hosted-zone and operational-control transfer sequencing.
2. AWS-G owns broader AWS billing, root/contact posture, IAM/MFA, audit logging,
   and emergency-access readbacks.
3. Business operations owns private vault and recovery custody receipts without
   exposing private material.
4. Release and engineering maintainers own GitHub organization, Actions,
   environment, runner, repository recovery, and emergency-access readbacks.
5. `v-*.ai` domain transfers remain unscheduled backlog and are not a milestone
   gate here.

# CORP-A Critical-Asset Schedule

Issue: #482  
Sprint umbrella: #529  
Schedule artifact: `docs/operations/corporate/asset-register/critical-asset-schedule.v1.json`  
Redacted custody evidence: `docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json`

This schedule records the corporate critical-asset denominator for v0.92.1 CORP-A readiness. It is intentionally documentation and evidence only: credential values, private instruments, account recovery material, signatures, tax identifiers, payment data, and executed legal documents are outside repository custody.

## Authority Boundary

- The schedule is accepted by the corporate operator role for issue #482.
- Ownership, custody, assignment, license, trademark, provenance, and validation surfaces are represented by roles and redacted evidence references.
- Counsel review routes are recorded where brand, domain, media, assignment, or license assertions would require legal review.
- Private custody remains outside the repository and is represented only through redacted receipt IDs.

## Asset Register

| Asset ID | Class | Business owner | Custodian | Assignment | License/provenance route | Trademark route | Receipt |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `corp-a-source-control` | `source_control` | Agent Logic corporate operator | engineering-maintainer-role | operator-accepted-custody | repository license surface recorded | counsel review before public mark claims | `corp-a-source-control` |
| `corp-a-ci-cd` | `ci_cd` | Agent Logic corporate operator | release-maintainer-role | operator-accepted-custody | third-party action licensing review before production reliance | no new public mark claim | `corp-a-ci-cd` |
| `corp-a-production-domains` | `production_domains` | Agent Logic corporate operator | infrastructure-maintainer-role | operator-accepted-custody | redacted registrar custody evidence only | counsel review for brand-bearing domain use | `corp-a-production-domains` |
| `corp-a-dns-route53` | `dns_route53` | Agent Logic corporate operator | infrastructure-maintainer-role | operator-accepted-custody | AWS service custody recorded without account proofs | no new public mark claim | `corp-a-dns-route53` |
| `corp-a-certificates` | `certificates` | Agent Logic corporate operator | infrastructure-maintainer-role | operator-accepted-custody | certificate authority terms remain external | no new public mark claim | `corp-a-certificates` |
| `corp-a-email` | `email` | Agent Logic corporate operator | business-operations-role | operator-accepted-custody | mailbox provider terms remain private | counsel review before formal public brand correspondence claims | `corp-a-email` |
| `corp-a-infrastructure-as-code` | `infrastructure_as_code` | Agent Logic corporate operator | infrastructure-maintainer-role | operator-accepted-custody | IaC and provider terms route through dependency/infrastructure review | no new public mark claim | `corp-a-infrastructure-as-code` |
| `corp-a-aws-infrastructure` | `aws_infrastructure` | Agent Logic corporate operator | cloud-operations-role | operator-accepted-custody | AWS terms and account proofs are not reproduced | no new public mark claim | `corp-a-aws-infrastructure` |
| `corp-a-deployment-operations` | `deployment_operations` | Agent Logic corporate operator | release-maintainer-role | operator-accepted-custody | operational procedures are repository-governed | no new public mark claim | `corp-a-deployment-operations` |
| `corp-a-brand-trademark` | `brand_trademark` | Agent Logic corporate operator | business-operations-role | operator-accepted-custody | brand license route requires counsel | counsel review before registration, enforcement, or public ownership claims | `corp-a-brand-trademark` |
| `corp-a-provenance-license` | `provenance_license` | Agent Logic corporate operator | engineering-maintainer-role | operator-accepted-custody | inbound/outbound licensing remains a release gate | counsel review for brand-bearing license language | `corp-a-provenance-license` |
| `corp-a-assignment-acceptance` | `assignment_acceptance` | Agent Logic corporate operator | business-operations-role | operator-accepted-custody | executed assignments remain in private custody | counsel review before transferred trademark-right assertions | `corp-a-assignment-acceptance` |
| `corp-a-private-custody` | `private_custody` | Agent Logic corporate operator | business-operations-role | operator-accepted-custody | private instruments are excluded from repository validation | counsel handles brand-bearing private instruments | `corp-a-private-custody` |
| `corp-a-data-model-media` | `data_model_media` | Agent Logic corporate operator | documentation-maintainer-role | operator-accepted-custody | media and diagram provenance route through documentation review | counsel review before brand-bearing public release | `corp-a-data-model-media` |

## Validation Surfaces

The issue-local validators prove that:

- every declared critical asset class has exactly one accepted asset row
- each asset row has owner, custodian, provenance, licensing, trademark, assignment, custody receipt, and validation-surface coverage
- every asset has a matching accepted redacted custody receipt
- repository evidence contains no obvious private key, token, credential-value, or instrument payload
- the Markdown schedule is backed by the machine-readable schedule and custody evidence files

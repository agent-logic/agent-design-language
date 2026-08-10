# v0.92.1 Work Breakdown

## Lane A: Corporate And IP Transfer

| WP | Outcome | Depends on |
| --- | --- | --- |
| CORP-01 | Critical asset, account, ownership, transfer, and exclusion inventory | Setup |
| CORP-02 | Counsel-reviewed founder-to-company assignment and corporate acceptance evidence | CORP-01 |
| CORP-03 | Contributor, third-party, OSS, model, media, and trademark provenance dispositions | CORP-01 |
| CORP-04 | Company billing, MFA, recovery, vault, and administrative custody | CORP-01 |
| CORP-05 | Repository, domain, brand, publishing, and vendor control | CORP-03, CORP-04 |
| CORP-06 | Route53, ACM, SES, CloudFront/S3, monitoring, and workload migration | CORP-04 |
| CORP-07 | Terraform state, CI/CD, deployment identity, rollback, and runbook authority | CORP-05, CORP-06 |
| CORP-08 | Redacted chain-of-title and operational due-diligence closeout | CORP-02, CORP-03, CORP-05, CORP-07 |

## Lane B: C-SDLC v3

V3-01 through V3-16 retain the reviewed architecture's exact responsibilities and dependencies. V3-R01 is deferred. See the machine-readable [issue wave](WP_ISSUE_WAVE_v0.92.1.yaml) and [feature contract](features/CSDLC_V3_v0.92.1.md).

## Lane C: Distributed Multi-Agent Runtime Qualification

| WP | Outcome | Depends on |
| --- | --- | --- |
| DRT-01 | Freeze topology, scenarios, faults, thresholds, receipt schemas, and claims | Setup |
| DRT-02 | Deterministic ACIP, authority, ordering, duplicate, and replay conformance | DRT-01 |
| DRT-03 | Wuji three-voter production multi-agent proof | DRT-01, DRT-02, terminal #142/WP-04.16 |
| DRT-04 | Wuji plus two private AWS voters, continuity, partition, fencing, and healing | DRT-03 |
| DRT-05 | Security, identity, certificate, capability, stale-authority, and provider-failure qualification | DRT-03, DRT-04 |
| DRT-06 | Observatory coherent-cut, causal trace, redaction, and stale-read validation | DRT-03, DRT-04 |
| DRT-07 | Soak, resource bounds, cleanup, replay, and exact-revision synthesis | DRT-05, DRT-06 |

## Integration Tail

| WP | Outcome | Depends on |
| --- | --- | --- |
| INT-01 | Independent integrated review and remediation | CORP-08, V3-16, DRT-07 |
| INT-02 | Release candidate, rollback readiness, and ceremony | INT-01 |
| INT-03 | Next-milestone and V3-R01 handoff | INT-02 |

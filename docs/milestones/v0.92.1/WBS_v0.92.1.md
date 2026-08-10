# v0.92.1 Work Breakdown

## Lifecycle Sequence

The milestone follows the standard ADL sequence. Planning does not create or
start the execution wave.

| Stage | Work package | Required outcome |
| --- | --- | --- |
| Planning | Setup issue `#146` | Review and merge the planning-only milestone package |
| Opening | WP-01 | Create the milestone, labels, umbrellas, child issues, cards, exact live map, readiness proof, and explicit start gate |
| Execution | Lanes A-C | Execute only dependency-ready child packages under their coordination umbrellas |
| Integrated review | INT-01 | Independently review all lane evidence and remediate every blocker |
| Release qualification | INT-02 | Freeze the exact candidate, qualify it, rehearse rollback, and issue a go/no-go recommendation |
| Next-milestone planning | INT-03 | Prepare the downstream milestone and deferred-work handoff |
| Next-milestone review | INT-04 | Independently review and accept or reject that handoff |
| Release ceremony | INT-05 | Release only with explicit operator authorization and exact-candidate readback |
| Terminal closeout | INT-06 | Reconcile and close child issues, umbrellas, milestone records, handoff, and cleanup classifications |

WP-01 is the sole creator of the future live issue wave. Retired issues
`#149-#190` are historical planning mistakes and must not be reopened or used as
execution authority.

## WP-01: Milestone Opening

WP-01 creates all planned coordination umbrellas and work-package issues from
the reviewed [execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml).
It must prove the exact denominator, six-card completeness, issue-specific
designs, owned paths, dependencies, PVF lanes and budgets, stop conditions,
validators, review readiness, and canonical GitHub readback before any child
may bind.

## Lane A: Corporate And IP Transfer

| WP | Outcome | Depends on |
| --- | --- | --- |
| CORP-01 | Critical asset, account, ownership, transfer, and exclusion inventory | WP-01 |
| CORP-02 | Counsel-reviewed founder-to-company assignment and corporate acceptance evidence | CORP-01 |
| CORP-03 | Contributor, third-party, OSS, model, media, and trademark provenance dispositions | CORP-01 |
| CORP-04 | Company billing, MFA, recovery, vault, and administrative custody | CORP-01 |
| CORP-05 | Repository, domain, brand, publishing, and vendor control | CORP-03, CORP-04 |
| CORP-06 | Route53, ACM, SES, CloudFront/S3, monitoring, and workload migration | CORP-04 |
| CORP-07 | Terraform state, CI/CD, deployment identity, rollback, and runbook authority | CORP-05, CORP-06 |
| CORP-08 | Redacted chain-of-title and operational due-diligence closeout | CORP-02, CORP-03, CORP-05, CORP-07 |

## Lane B: C-SDLC v3

V3-01 through V3-16 retain the reviewed architecture's exact responsibilities
and dependencies. V3-R01 remains deferred beyond the rollback window. All
eleven architecture decisions remain explicit gates. See the machine-readable
[issue wave](WP_ISSUE_WAVE_v0.92.1.yaml) and
[feature contract](features/CSDLC_V3_v0.92.1.md).

## Lane C: Distributed Multi-Agent Runtime Qualification

| WP | Outcome | Depends on |
| --- | --- | --- |
| DRT-01 | Freeze topology, scenarios, faults, thresholds, receipt schemas, and claims | WP-01 |
| DRT-02 | Deterministic ACIP, authority, ordering, duplicate, and replay conformance | DRT-01 |
| DRT-03 | Wuji three-voter production multi-agent proof | DRT-01, DRT-02, terminal #142/WP-04.16 |
| DRT-04 | Wuji plus two private AWS voters, continuity, partition, fencing, and healing | DRT-03 |
| DRT-05 | Security, identity, certificate, capability, stale-authority, and provider-failure qualification | DRT-03, DRT-04 |
| DRT-06 | Observatory coherent-cut, causal trace, redaction, and stale-read validation | DRT-03, DRT-04 |
| DRT-07 | Soak, resource bounds, cleanup, replay, and exact-revision synthesis | DRT-05, DRT-06 |

## Integration And Closeout Tail

| WP | Outcome | Depends on |
| --- | --- | --- |
| INT-01 | Independent integrated review and remediation | CORP-08, V3-16, DRT-07 |
| INT-02 | Release-candidate qualification and rollback rehearsal | INT-01 |
| INT-03 | Next-milestone planning and deferred-work handoff | INT-02 |
| INT-04 | Independent next-milestone review and handoff acceptance | INT-03 |
| INT-05 | Operator-authorized release ceremony | INT-02, INT-04 |
| INT-06 | Terminal child, umbrella, milestone, lifecycle, handoff, and cleanup closeout | INT-05 |

## Opening And Closing Invariants

- Planning issue `#146` may publish specifications but may not create the child wave.
- WP-01 creates each planned issue exactly once and publishes the exact live map.
- No child starts before WP-01 is independently reviewed and explicitly authorized.
- Implementation issues close through their merged PR or a reviewed no-PR disposition.
- Coordination umbrellas close only after reconciling their exact child denominator.
- Release occurs only after integrated review, release qualification, next-milestone planning, and independent handoff review.
- Milestone closeout occurs only after release, terminal issue reconciliation, accepted deferred routing, and non-destructive cleanup classification.

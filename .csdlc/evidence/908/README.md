# Current AWS inventory delta — 2026-09-12 UTC

Issue #908; Sprint 8 umbrella #934. **Read-only inventory completed; no cloud mutation or deletion authorization.**

Capture: `2026-09-12T00:20:16.309055+00:00` through `2026-09-12T00:21:06.735126+00:00`. Explicit business profile verified by live STS equality against the approved #484 baseline before resource queries. Account and principal identifiers are omitted.

## Scope and change

Observed 157 global/regional surfaces across 17 enabled regions. #484 had 17 enabled regions and 136 regional readbacks. This refresh retains those eight regional families and adds tagging in each enabled region. Resource families are scoped, not a claim to inventory every AWS service.

| Compared surface | Baseline | Current | New references | Previously seen, not observed now |
| --- | ---: | ---: | ---: | ---: |
| eu-west-2/subnets | 3 | 4 | 1 | 0 |
| global/global-tagged-resources | 59 | 106 | 48 | 1 |
| global/s3-buckets | 11 | 13 | 2 | 0 |
| us-west-2/cloudformation-stacks | 1 | 2 | 1 | 0 |
| us-west-2/security-groups | 31 | 38 | 7 | 0 |

All other comparable surfaces have unchanged resource reference sets. Additional regional tagging surfaces are **new coverage**, not proof of newly created resources. The formerly tagged reference not returned now is not confirmed deleted; untagging or API visibility can also explain that difference. All reference-level sets, capture times and failed/excluded surfaces are retained in [inventory.json](inventory.json).

## SCR, S3 and model maintenance

Thirteen current buckets versus eleven in #484; no baseline bucket is missing. All bucket location and current-object listing reads succeeded. Five tag reads returned `NoSuchTagSet`, retained explicitly as metadata read failures. No arbitrary tag values or object keys were retained.

Purpose hints below derive only from the bucket name and do **not** prove application ownership. The historical frozen-unknown disposition is preserved for all resources; actual bucket custody in the business account does not authorize cross-project cleanup. SCR and model artifacts remain preserved pending an ownership attestation.

| Bucket reference (prefix) | Purpose hint | Current objects | Observed bytes | Newest object | Maintenance classification |
| --- | --- | ---: | ---: | --- | --- |
| `e2c377bfd5fd` | other | 82 | 104661808 | 2026-07-04T00:50:08+00:00 | age >30d; review ownership/use |
| `16582900484c` | other | 1284 | 1279094381 | 2026-07-01T23:19:49+00:00 | age >30d; review ownership/use |
| `56e657ae2ee1` | other | 4459 | 38661270057 | 2026-07-12T04:24:47+00:00 | age >30d; review ownership/use |
| `b3e005129736` | model-artifacts | 158 | 22179635249 | 2026-09-03T08:20:48+00:00 | recent write; ownership unproven |
| `f9890bd0400e` | other | 1 | 633 | 2026-07-06T20:02:08+00:00 | age >30d; review ownership/use |
| `9a466cdaa751` | other | 73 | 36768928 | 2026-09-03T17:59:17+00:00 | recent write; ownership unproven |
| `beb562a0e17f` | other | 6138 | 127866277 | 2026-09-12T00:19:21+00:00 | recent write; ownership unproven |
| `2cc243159224` | other | 2 | 70330 | 2026-09-08T08:57:54+00:00 | recent write; ownership unproven |
| `459ff0278553` | other | 87 | 191910866 | 2026-08-09T02:51:14+00:00 | age >30d; review ownership/use |
| `3a202cc7e346` | scr | 191 | 832350971424 | 2026-06-30T21:11:38+00:00 | age >30d; review ownership/use |
| `617a606cc719` | other | 2 | 2019 | 2026-07-03T21:23:43+00:00 | age >30d; review ownership/use |
| `39dc254066a3` | other | 5 | 248239 | 2026-08-26T17:31:25+00:00 | recent write; ownership unproven |
| `16487500d811` | scr | 0 | 0 | none observed | empty; usage unknown |

All object listings in this capture completed below the 10,000-object bound. Timestamps measure object writes, not accesses or disuse. Object contents and versions were excluded. Empty/old resources stay frozen; no deletion is recommended from this evidence alone.

## Validation and review boundary

The collector source hash and immutable #484 readback/script/inventory hashes are in the packet. [Maintenance](MAINTENANCE.md) defines safe repetition, weekly cadence, 24-hour evidence freshness and retention. [PVF classification](PVF.json) separates live external observations from deterministic local checks.

Run `python3 .csdlc/evidence/908/inventory.py validate` for current freshness and `python3 .csdlc/evidence/908/test_inventory.py` for deterministic negative cases at capture time. Validation results and independent review are recorded in issue cards and PR evidence. No Rust runtime, release-wide coverage, migration or deployment proof is claimed.

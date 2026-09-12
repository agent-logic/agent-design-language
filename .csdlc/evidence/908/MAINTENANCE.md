# Business AWS inventory maintenance

Issue #908; Sprint 8 umbrella #934. Owner role: Agent Logic cloud operations
maintainer. Refresh weekly and after an approved resource change, before any
ownership-dependent decision. This inventory never authorizes deletion or apply.

## Safe refresh

Run from a new issue-bound checkout with Python 3 and the AWS CLI installed:

```sh
python3 .csdlc/evidence/908/inventory.py capture
python3 .csdlc/evidence/908/inventory.py validate
python3 .csdlc/evidence/908/test_inventory.py
```

For a future issue, copy this bounded collector into its new evidence directory
and use a new dated packet, preserving #908 and #484 bytes. The current validator
uses a 24-hour freshness window. Historical negative tests validate against the
recorded capture-end time; their passing result does not make old evidence live.
An independent reviewer must inspect the actual dated readbacks and delta before
publication. Do not refresh an old PR silently after review.

Every API call is an allowlisted get/list/describe command using
`agent-logic-admin`. Ambient AWS environment overrides are removed for each call.
STS account equality with the approved #484 business baseline is checked before
region/resource queries. A mismatch or failed identity read stops the run. Region
discovery failure stops rather than falling back to cached or default regions.
No raw AWS response, credential, account identifier, resource name, object key,
or tag value is written to disk. Public resource references are SHA-256 digests
of AWS identifiers, permitting repeatable comparison against retained #484.
Raw readbacks live only in process memory; retained evidence is an allowlisted
projection. CLI errors retain only a short code, never raw stderr.

## Denominator and limitations

- Discover all regions; inspect every enabled/opted-in region.
- Preserve #484's four global resource families plus eight regional families.
  The historical `global-tagged-resources` is actually the tagging endpoint in
  us-west-2; retain that comparison and add tagging reads in each enabled region.
- CloudFormation retains the historical four active-complete stack statuses.
  Failed/in-progress/deleted stacks are excluded; this is not an all-service AWS
  audit. IAM, billing, object versions/delete markers, snapshots except tagged
  references, and non-enabled regions are outside this bounded census.
- AWS CLI automatic pagination is enabled. S3 object metadata uses an explicit
  10,000-object bound per bucket and records completeness. A truncated result is
  a partial sample, never a bucket total. No object contents are read.
- Bucket location, tag availability and object count/size/time are recorded for
  every observed bucket. `NoSuchTagSet` is retained as a metadata read outcome;
  it does not mean the bucket is missing. Other read failures remain uncertainty.
- Collection is a timestamped sequence, not an atomic snapshot. Resources may
  change during it. A missing previously observed identifier is `not_observed_now`,
  never confirmed deletion. A new tagging surface is new coverage, not proof
  that its resources were newly created.

## Ownership and staleness

Preserve `owned`, `externally-owned`, `frozen-unknown`, `not-observed`, and
`read-failed`. The business account check proves custody of this census, not
application ownership. #484 classified resources as frozen-unknown; #908 does
not upgrade that classification from a name or mere presence. SCR/model labels
are name-derived purpose hints only. Maintainers must supply an ownership record
or approved infrastructure provenance before changing those dispositions.

An object whose newest modification is more than 30 days old is a maintenance
review candidate, not unused or safe to delete. S3 modification timestamps do not
measure reads/access. Empty buckets likewise do not establish disuse. Escalate
unknown ownership and stale candidates into separately authorized issues; never
run cleanup from the inventory collector.

Retain the sanitized packet, collector, source revision, hashes of unchanged
baseline files, validation output, and independent review with the issue. Keep
historical packets immutable in Git. Credentials stay in the approved provider
configuration and are never copied into evidence. If required readback fails,
record the exact affected surface and retry in a new capture after resolving
access; do not convert failure into an empty resource list.

# Issue #594 Design: Bounded Runtime Log Archival to S3

## Decision

Keep live Runtime health and recovery independent from archival. The S3 sink
branches only from `runtime_v3_redacted`, disables its sink health check, and
uses its own 512 MiB disk buffer with `when_full = "drop_newest"`. The existing
Vector process may therefore remain startup-required without making S3
startup-required: `--require-healthy true` evaluates the existing required
sinks, while the archive sink is explicitly excluded from startup health.

The archive flushes at the earlier of 5 MiB or 60 seconds. Delivery retries at
most five times with exponential backoff capped at 30 seconds. Exhaustion emits
a bounded archive-delivery counter/log signal and drops only the newest archive
copy; it never blocks the redaction transform, master log, CloudWatch path, or
Runtime producer.

## Boundaries

- Runtime emits through the existing observability boundary.
- Vector owns buffering, retry, and S3 delivery.
- Terraform owns the bucket, lifecycle, encryption, public-access block, and
  least-privilege publisher policy.
- Object keys identify environment, Polis, Runtime instance, and UTC date.
- Live AWS proof uses the Agent Logic business account and must remain a
  separately authorized paid-cloud action.

## Concrete Storage Contract

- Object key grammar:
  `logs/env=<env>/polis=<polis>/runtime=<runtime>/year=<YYYY>/month=<MM>/day=<DD>/hour=<HH>/<uuid>.json.gz`.
- Identity segments are validated lowercase DNS-safe labels and never contain
  credentials, host paths, query strings, or arbitrary event data.
- The bucket retains current versions for 30 days and noncurrent versions for
  7 days, then expires them; incomplete multipart uploads expire after 1 day.
- SSE-S3 is mandatory by default, public access is fully blocked, ownership is
  bucket-owner-enforced, and versioning is enabled.
- Publisher IAM permits only `s3:GetBucketLocation` on the exact bucket and
  `s3:PutObject` plus abort/list multipart operations on the exact
  `logs/env=<env>/polis=<polis>/runtime=<runtime>/*` prefix.

## Failure Model

The archive queue is capped at 512 MiB. S3 throttling, denial, or outage uses no
more than five delivery attempts per batch and 30 seconds between attempts.
When full, the archive branch drops newest archive events and increments
failure/drop telemetry; it does not backpressure the shared redacted transform.
Tests must start Vector and Runtime with an unreachable S3 endpoint, fill the
archive buffer past its configured bound, and prove the master log and Runtime
readiness continue advancing. No credential, raw sensitive payload, or
machine-local path enters an object key or archived record.

## Live Proof Contract

The live validator compares STS identity with an operator-supplied expected
business account ID, inspects versioning, encryption, public-access block and
lifecycle state, retrieves the exact proof object into `.csdlc/evidence/594`,
and checks that it is nonempty redacted JSON without printing content. Static
Terraform proof checks the exact publisher policy; live proof does not claim
resource cleanup or mutate the deployment.

## Execution Gate

Planning and validation stay under typed C-SDLC v2. The issue may be used as a
C-SDLC v3 lifecycle exercise only after the explicit operator-reviewed v3
cutover changes repository authority.

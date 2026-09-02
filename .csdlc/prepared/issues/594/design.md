# Issue #594 Design: Bounded Runtime Log Archival to S3

## Decision

Keep live Runtime health and recovery independent from archival. Vector buffers
already-redacted Runtime logs locally and uploads them to a dedicated,
Terraform-managed S3 bucket on a bounded cadence. Failure to reach S3 degrades
the archival sink only; it never changes Runtime readiness or the CloudWatch
health path.

## Boundaries

- Runtime emits through the existing observability boundary.
- Vector owns buffering, retry, and S3 delivery.
- Terraform owns the bucket, lifecycle, encryption, public-access block, and
  least-privilege publisher policy.
- Object keys identify environment, Polis, Runtime instance, and UTC date.
- Live AWS proof uses the Agent Logic business account and must remain a
  separately authorized paid-cloud action.

## Failure Model

The archive queue is disk-bounded. S3 throttling, denial, or outage produces
bounded retry and explicit local/CloudWatch telemetry. Exhaustion drops archive
delivery according to documented policy without blocking or terminating the
Runtime. No credential, raw sensitive payload, or machine-local path enters an
object key or archived record.

## Execution Gate

Planning and validation stay under typed C-SDLC v2. The issue may be used as a
C-SDLC v3 lifecycle exercise only after the explicit operator-reviewed v3
cutover changes repository authority.

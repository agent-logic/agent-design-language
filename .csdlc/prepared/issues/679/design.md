# Design: S3 deployable Observatory

## Purpose

Issue #679 is the infrastructure sidecar for the #512 HTML Observatory. It
prepares the static hosting, edge, DNS, certificate, header, rollback, and
readback contract needed to serve the Observatory at
`https://observatory.csm.agent-logic.ai` without absorbing #512 product UI
ownership.

## Boundary

This issue owns deployability and AWS infrastructure readiness for the static
Observatory edge:

- S3 bucket and object-deployment contract.
- CloudFront distribution, OAC, cache policy, response headers, CSP, logging,
  invalidation, and rollback/versioning behavior.
- ACM certificate and Route53 alias record plan for
  `observatory.csm.agent-logic.ai`.
- Redacted AWS readback scripts that fail closed unless the Agent Logic
  business profile `agent-logic-admin` is selected.
- Local validation proving the deployable bundle/config contract avoids
  credentials, absolute dev-only paths, and unsafe persistence examples.

This issue does not implement #512's product UI, multi-polis selector, Runtime
API/WSS behavior, provider workloads, EC2/Spot runtime hosting, or live AWS
mutation without explicit operator authorization.

## Deployment contract

The Observatory remains a static asset bundle. The bundle must use relative
asset and configuration paths so it can be served locally, from S3 website-like
object storage behind CloudFront, and from the production origin without source
rewrites. Runtime HTTPS/WSS endpoints remain per-polis external inputs rather
than bundled secrets.

The deployment plan should describe:

1. S3 ownership, object lock/versioning posture, public access block, and
   CloudFront-only access through OAC.
2. CloudFront alias, certificate, cache policy, response headers policy, CSP,
   logging, invalidation, and rollback strategy.
3. Route53 hosted-zone lookup and A/AAAA alias record for the Observatory host.
4. Redacted readback commands for bucket, distribution, certificate, DNS, and
   deployed object/version state.
5. Exact-origin CORS/WSS compatibility with Runtime endpoints without embedding
   credentials in bundle files, URLs, local storage examples, or retained logs.

## Validation approach

Preparation validation is deliberately local and non-mutating. It proves that
the tracked issue package carries the required boundaries, references the
expected host/profile, and exposes an executable lane for future implementation.

Implementation validation should later add focused checks for Terraform or
deployment-plan syntax, static asset relativity, CSP/header compatibility,
secret redaction, and dry-run/readback behavior. Live AWS apply/readback must be
separately authorized and truthfully classified in SOR.

## Rollback

Rollback is a first-class acceptance surface. The final plan must retain enough
metadata to identify and restore the prior deployed object set or CloudFront
configuration, and it must distinguish local/dry-run proof from any authorized
live AWS readback.

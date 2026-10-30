# Static Observatory edge

This standalone Terraform root serves the #512 static Observatory bundle at
`https://observatory.csm.agent-logic.ai`. It creates a private, versioned S3
origin, CloudFront OAC distribution, 90-day access logging, explicit security headers/CSP, a DNS-
validated us-east-1 ACM certificate, and Route53 A/AAAA aliases. It does not
create or proxy Runtime API or WSS infrastructure.

The template is inert until an operator explicitly authorizes a live AWS run.
Use only the `agent-logic-admin` business profile. Never place bearer tokens,
credentials, private keys, account identifiers, or signed URLs in variables,
the static bundle, Terraform state, command lines, or retained evidence.

## Deployment contract

The #512 build supplies only relative static assets. Runtime HTTPS/WSS origins
are non-secret endpoint metadata and must be explicitly listed in
`runtime_connect_origins`; each Runtime must separately allow the exact browser
origin `https://observatory.csm.agent-logic.ai`. Authentication remains a
browser-to-Runtime concern and is not stored in this template. The validator
rejects durable `localStorage` persistence of the Observatory token; #512's
current session-scoped token is cleared when its browser tab closes.

After separate authorization, the safe sequence is `terraform init`,
`terraform plan`, reviewed apply, immutable asset upload, and a CloudFront
invalidation. Upload hashed assets with long-lived immutable caching and upload
`index.html` plus the public endpoint registry with short/no-cache headers. The
distribution disables caching by default and enables optimized caching only for
CSS and JavaScript paths.

CloudFront standard logging uses the legacy S3 logging path. The log bucket
therefore keeps ACLs enabled with `BucketOwnerPreferred`, Terraform manages the
bucket owner, S3 log-delivery group, and CloudFront `awslogsdelivery` canonical
ACL grants explicitly, and the distribution depends on that ACL setup before it
is created.

`readback.sh` is inert unless passed `--execute` and fails closed unless
`AWS_PROFILE=agent-logic-admin`. Its fixed projections omit ARNs, account IDs,
policies, tags, credentials, signed URLs, and Terraform state while reporting
CloudFront/logging posture, S3 public access and object versions, log retention,
ACM status, and Route53 A/AAAA aliases.

## Rollback

S3 versioning retains prior objects. Record the immutable `artifact_version`
and uploaded object version IDs outside secrets. Roll back by restoring the
previous object versions and invalidating `/*`; Terraform rollback is needed
only for an infrastructure change. Do not delete the active bucket or
distribution as a content rollback mechanism.

All readback evidence must be redacted. A future authorized run should capture
only resource names, configuration posture, certificate status, and HTTP/CSP
results—never credentials, account IDs, tokens, or raw Terraform state.

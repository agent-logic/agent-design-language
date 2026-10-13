> Infrastructure source and execution for the Terraform paths below moved to [agent-logic-infrastructure](https://github.com/agent-logic/agent-logic-infrastructure/tree/61c2806b6274b1003aa42f02adec99faf5f2a74b/infra). Run applicable commands from that owner checkout with its reviewed inputs. These retained procedures and historical outcomes do not establish new cloud or deployment acceptance.

# Observatory S3/CloudFront deployment runbook

Status: deployment-ready plan; live AWS apply and readback are deferred until
explicit operator authorization.

The Terraform root at `infra/aws/observatory` owns only the static web edge for
`observatory.csm.agent-logic.ai`. #512 owns the Observatory product bundle;
Runtime HTTPS and WSS endpoints remain separate infrastructure.

Before an authorized run, confirm the bundle contains relative asset/config
paths, contains no secrets, and names only credential-free Runtime origins.
Confirm each target Runtime permits the exact browser origin
`https://observatory.csm.agent-logic.ai`. Then use `AWS_PROFILE=agent-logic-admin`
for Terraform planning and reviewed application. The template itself also
rejects any other profile value.

Release order: provision and validate the certificate/distribution, upload an
immutable bundle, smoke-test the CloudFront hostname, update/confirm Route53,
and invalidate changed entry/config paths. CloudFront access logs are retained
in a private lifecycle-managed bucket for 90 days. Because the template uses
CloudFront standard logging's legacy S3 destination, the log bucket must keep
ACLs enabled before the distribution is created and Terraform must preserve the
CloudFront `awslogsdelivery` canonical ACL grant. Validate first paint, CSP
headers, HTTPS API access, WSS upgrade, per-polis identity, and failure
isolation.

Rollback restores the prior S3 object versions and invalidates `/*`. Preserve
the distribution, certificate, and DNS record unless the infrastructure change
itself is being rolled back. Redact all evidence; never retain AWS account IDs,
tokens, credentials, signed URLs, private keys, or Terraform state.

The profile-gated `infra/aws/observatory/readback.sh` provides the bounded
readback projection and remains inert without `--execute`.

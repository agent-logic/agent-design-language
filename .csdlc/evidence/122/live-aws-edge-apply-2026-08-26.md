# Issue #122 live AWS edge apply evidence

Date: 2026-08-26

## AWS account and DNS

- Business AWS profile used for edge resources: `agent-logic-admin`
- Approved business account verified by Terraform account guard; account id is not recorded in committed evidence.
- Parent DNS profile used only for `agent-logic.ai` delegation/ACM validation
  records: `default`
- Parent DNS authority note: `default` and `agent-logic-admin` resolve to
  different AWS accounts. The `default` profile was used only for the parent
  hosted zone because the parent `agent-logic.ai` zone lives there, and only
  for bounded parent-zone delegation / ACM DNS-validation records needed after
  the operator explicitly instructed this session to get the CSM domain names
  and certificates working. It was not used for CSM edge resources, Runtime
  origin resources, S3, CloudFront, WAF, API Gateway, EC2, or ALB.
- Parent public hosted zone: `agent-logic.ai` / `Z0105194MPK8MKMH2XAQ`
- Created business hosted zone: `csm.agent-logic.ai` / `Z02742802Q0LW6PL6HQ2D`
- Delegated name servers:
  - `ns-1218.awsdns-24.org`
  - `ns-1544.awsdns-01.co.uk`
  - `ns-62.awsdns-07.com`
  - `ns-819.awsdns-38.net`

## ACM certificates

Requested/validated CloudFront viewer certificate:

- `*.wuji.dev.csm.agent-logic.ai`
- ARN: retained in local ignored Terraform inputs; not recorded in committed evidence.
- Status observed: `ISSUED`

Additional namespace certificates requested for future naming variants:

- `*.csm.agent-logic.ai`: `ISSUED`
- `*.dev.csm.agent-logic.ai`: `ISSUED`
- `*.wuji.csm.agent-logic.com`: `PENDING_VALIDATION` because no accessible Route53 hosted zone for `agent-logic.com` was found in the available AWS profiles.

Review remediation: these extra namespace certificates were unused and were
deleted from ACM after review identified them as certificate churn. Post-delete
ACM inventory retained only the active CloudFront viewer certificate
`*.wuji.dev.csm.agent-logic.ai` in `us-east-1` plus the reusable regional
origin certificate `origin-smoke.wuji.dev.csm.agent-logic.ai` in `us-west-2`.
No disposable ALB stack owns or recreates those certificates by default.

Review remediation also removed the two stale parent-zone ACM validation CNAMEs
that corresponded to the deleted `*.csm.agent-logic.ai` and
`*.dev.csm.agent-logic.ai` certificates. The parent `agent-logic.ai` zone now
retains only:

- the `csm.agent-logic.ai` NS delegation; and
- the active ACM validation CNAME for `*.wuji.dev.csm.agent-logic.ai`.

Route53 cleanup change:

```text
/change/C0169968O3N8DSCH7LIG
```

## Terraform apply

Terraform worktree:

```text
/Volumes/FastWork/adl-worktrees/adl-issue-122-csm-public-edge-terraform
```

Applied in two steps:

```text
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-public-edge apply -auto-approve issue122-zone.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-public-edge apply -auto-approve issue122-full.tfplan
```

Final apply result:

```text
Apply complete! Resources: 22 added, 0 changed, 0 destroyed.
```

Terraform outputs:

```text
observatory_fqdn = "observatory.wuji.dev.csm.agent-logic.ai"
api_fqdn = "api.wuji.dev.csm.agent-logic.ai"
wss_fqdn = "wss.wuji.dev.csm.agent-logic.ai"
origin_fqdn = "wuji.dev.csm.agent-logic.ai"
origin_cname_target = "wuji.agent-logic.ai"
observatory_bucket = "csm-wuji-dev-observatory-assets"
observatory_cloudfront_domain = "d31sm5j4e5rraf.cloudfront.net"
api_cloudfront_domain = "d2rj6kchzz22y5.cloudfront.net"
wss_cloudfront_domain = "dnpwjw41tm26s.cloudfront.net"
runtime_http_api_endpoint = "https://tb485bn6j4.execute-api.us-west-2.amazonaws.com"
```

## Observatory asset deployment

Uploaded repo HTML Observatory assets to:

```text
s3://csm-wuji-dev-observatory-assets/
```

Excluded:

```text
demos/html-observatory/tests/*
```

Overrode deployed `runtime-v3.config.json` with:

```json
{
  "schema": "adl.html_observatory.runtime_v3_config.v1",
  "api_base": "https://api.wuji.dev.csm.agent-logic.ai",
  "health_endpoint": "/v1/health",
  "observatory_endpoint": "/v1/observatory",
  "readiness_endpoint": "/v1/ready",
  "observatory_websocket_endpoint": "/v1/observatory/ws",
  "signed_command_endpoint": "/v1/control",
  "observatory_docs_endpoint": "/v1/observatory/docs/"
}
```

CloudFront invalidation:

```text
IDW9XOJ9EU817UOR969NCTYMZF
Status: Completed
```

## Live endpoint checks

Public DNS resolved:

```text
observatory.wuji.dev.csm.agent-logic.ai -> CloudFront A records
api.wuji.dev.csm.agent-logic.ai -> CloudFront A records
wss.wuji.dev.csm.agent-logic.ai -> CloudFront A records
wuji.dev.csm.agent-logic.ai -> CNAME wuji.agent-logic.ai
```

Observatory root:

```text
curl -I https://observatory.wuji.dev.csm.agent-logic.ai/
HTTP/2 200
content-type: text/html
server: AmazonS3
x-cache: Miss from cloudfront
```

Deployed config:

```text
curl https://observatory.wuji.dev.csm.agent-logic.ai/runtime-v3.config.json
api_base = https://api.wuji.dev.csm.agent-logic.ai
```

Allowed CORS origin:

```text
curl -i -X OPTIONS https://api.wuji.dev.csm.agent-logic.ai/v1/health \
  -H 'Origin: https://observatory.wuji.dev.csm.agent-logic.ai' \
  -H 'Access-Control-Request-Method: GET'

access-control-allow-origin: https://observatory.wuji.dev.csm.agent-logic.ai
access-control-allow-methods: GET,OPTIONS,POST
access-control-allow-headers: authorization,content-type,origin,traceparent,tracestate,x-csm-correlation-id,x-request-id
access-control-allow-credentials: true
```

Rejected CORS origin:

```text
curl -i -X OPTIONS https://api.wuji.dev.csm.agent-logic.ai/v1/health \
  -H 'Origin: https://evil.example.com' \
  -H 'Access-Control-Request-Method: GET'

No access-control-allow-origin header emitted.
```

## Remaining live Runtime gate

The Runtime is healthy locally through `CSMctl`:

```text
./CSMctl status
CSMctl probe /v1/ready http=200
CSMctl probe /v1/observatory http=200
CSMctl probe /v1/health http=200
CSMctl status=pass runtime_base=https://localhost:20997
```

But the selected public Runtime origin is not reachable externally:

```text
curl --resolve wuji.agent-logic.ai:443:47.146.81.109 https://wuji.agent-logic.ai/v1/health
curl: (28) Connection timed out

curl --resolve wuji.agent-logic.ai:20997:47.146.81.109 https://wuji.agent-logic.ai:20997/v1/health
curl: (7) Failed to connect
```

Therefore API and WSS Runtime proof remain blocked on exposing the Runtime at a
public TLS origin that CloudFront/API Gateway can reach. The edge DNS, ACM,
CloudFront, WAF, API Gateway CORS allowlist, Observatory asset serving, and
Observatory config hookup are live-proven.

## Runtime origin follow-up in same issue

After the public-origin blocker was observed, #122 was widened to include two
separate quick-create/quick-destroy AWS origin stacks:

- `infra/aws/csm-runtime-spot`: one disposable small Spot EC2 Runtime host.
- `infra/aws/csm-runtime-alb`: one public HTTPS ALB origin with reusable ACM
  certificate lookup and optional target attachment.

These are deliberately separate from `infra/aws/csm-public-edge`, so the
permanent CloudFront/WAF/API Gateway/DNS edge can stay up while the Runtime host
or ALB is killed and recreated.

## Disposable AWS Runtime origin smoke

The disposable Runtime origin stacks were live-tested after the permanent edge
proof:

1. Created one public HTTPS ALB origin for
   `origin-smoke.wuji.dev.csm.agent-logic.ai`.
2. Created one small Spot EC2 instance with user-data that served
   `/v1/health` over HTTPS on port `20997`.
3. Attached the instance to the ALB target group.
4. Waited for the ALB target to become healthy.
5. Called the public ALB origin from outside the instance.
6. Detached and destroyed the Spot stack.
7. Destroyed the disposable ALB stack.

The proof call returned HTTP 200 from the public origin and included the EC2
instance id in the response body:

```text
curl -sS --max-time 20 -D - https://origin-smoke.wuji.dev.csm.agent-logic.ai/v1/health

HTTP/2 200
content-type: application/json
server: nginx/1.30.4

{"schema":"adl.csm_runtime_origin_smoke.v1","status":"ok","origin":"ec2-smoke","instance_id":"i-027183bbc454a62e3"}
```

The matching target-health check reported the same instance healthy on port
`20997`.

Teardown proof:

```text
terraform -chdir=infra/aws/csm-runtime-spot destroy ... -> Destroy complete! Resources: 4 destroyed.
terraform -chdir=infra/aws/csm-runtime-alb destroy ...  -> Destroy complete! Resources: 7 destroyed.
terraform -chdir=infra/aws/csm-runtime-spot show        -> The state file is empty.
terraform -chdir=infra/aws/csm-runtime-alb show         -> The state file is empty.
```

The ALB stack was corrected after the smoke so it does not mint a fresh ACM
certificate every run. Normal behavior is:

- `certificate_arn = null`
- `reuse_existing_certificate = true`
- `create_certificate = false`

With that configuration, Terraform looks up an existing ISSUED regional ACM
certificate for `origin_fqdn` and fails closed if none exists. The first-time
certificate bootstrap path remains explicit:

- `reuse_existing_certificate = false`
- `create_certificate = true`

The smoke-created regional ACM certificate for
`origin-smoke.wuji.dev.csm.agent-logic.ai` remains in AWS and is intentionally
outside the disposable ALB Terraform state so future ALB create/destroy cycles
reuse the same certificate instead of creating a new one.

The empty-state ALB recreate plan proved the lookup-first behavior without
applying new resources:

```text
terraform -chdir=infra/aws/csm-runtime-alb plan \
  -out=issue122-alb-recreate-lookup-smoke.tfplan \
  -var origin_fqdn_override=origin-smoke.wuji.dev.csm.agent-logic.ai \
  ...

module.runtime_alb.data.aws_acm_certificate.existing_origin[0]: Read complete
Plan: 7 to add, 0 to change, 0 to destroy.
certificate_arn = existing regional ACM certificate
```

No ACM certificate resource appeared in the plan.

The local wuji path is being coordinated through Caddy/Let's Encrypt: Caddy
should terminate public origin TLS for `wuji.dev.csm.agent-logic.ai` (or the
selected origin hostname) and proxy to the CSMctl-managed Runtime on localhost.
CSMctl remains responsible for Runtime liveness; Caddy is only the public TLS
and reverse-proxy layer.

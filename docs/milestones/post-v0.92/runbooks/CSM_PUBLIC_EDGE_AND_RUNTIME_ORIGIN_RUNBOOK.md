# CSM Public Edge and Runtime Origin Runbook

Issue: #122

This runbook is the practical operator path for bringing up a CSM public edge
and, when needed, a disposable AWS Runtime origin. It is intentionally simple:
one permanent edge stack, one optional Spot instance stack, and one optional ALB
origin stack.

## What this creates

Permanent, non-ephemeral resources:

- Route53 CSM namespace zone, when one is not supplied.
- ACM viewer certificate for the CloudFront-facing CSM hostnames.
- S3 bucket for the static HTML Observatory.
- CloudFront distribution with WAF.
- API Gateway HTTP/WebSocket front doors for Runtime API and WSS routing.

Disposable proof resources:

- `infra/aws/csm-runtime-spot`: one small Spot EC2 Runtime host.
- `infra/aws/csm-runtime-alb`: one public HTTPS ALB origin that can attach to
  the Spot host.

The disposable Spot and ALB stacks are separate so either can be destroyed and
recreated quickly without disturbing the permanent edge.

## Naming

Development and staging use:

```text
<function>.<csm_name>.<environment>.csm.agent-logic.ai
```

Example:

```text
observatory.wuji.dev.csm.agent-logic.ai
api.wuji.dev.csm.agent-logic.ai
wss.wuji.dev.csm.agent-logic.ai
origin-smoke.wuji.dev.csm.agent-logic.ai
```

Production uses:

```text
<function>.<csm_name>.csm.agent-logic.com
```

## Accounts and certificates

- Use the Agent Logic business AWS profile for CSM edge and runtime resources.
- Parent-domain delegation or ACM validation records may require the account
  that owns the parent hosted zone.
- CloudFront viewer certificates are ACM certificates in `us-east-1`.
- ALB origin certificates are regional ACM certificates.
- Reuse existing regional ALB certificates by default. Do not create a new ALB
  certificate for every test run.

## 1. Permanent public edge

Work from:

```text
infra/aws/csm-public-edge
```

Prepare a `terraform.tfvars` from `terraform.tfvars.example`. The important
values are:

- `csm_name`
- `environment`
- `domain_name`
- `hosted_zone_id` or first-time hosted-zone creation settings
- `edge_acm_certificate_arn`, when reusing an issued CloudFront viewer cert
- `origin_https_url`
- `wss_origin_https_url`
- `additional_allowed_origins`
- `origin_cname_target`, when pointing the base CSM hostname at DDNS or another
  operator-owned origin

Validate:

```bash
terraform -chdir=infra/aws/csm-public-edge init
terraform -chdir=infra/aws/csm-public-edge validate
bash adl/tools/validate_csm_public_edge_static.sh
```

Apply only after confirming the account and planned resource names:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-public-edge plan -out=issue122-edge.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-public-edge apply issue122-edge.tfplan
```

## 2. Optional disposable ALB origin

Use this when the CSM Runtime is not directly reachable from the public edge or
when you need an AWS proof endpoint.

Work from:

```text
infra/aws/csm-runtime-alb
```

Set:

- `csm_name`
- `environment`
- `origin_fqdn`
- `vpc_id`
- `public_subnet_ids`
- `certificate_lookup_domain`, normally a reusable wildcard such as
  `*.wuji.dev.csm.agent-logic.ai`
- `target_instance_id = null` for the first ALB creation pass

Validate and apply:

```bash
terraform -chdir=infra/aws/csm-runtime-alb init
terraform -chdir=infra/aws/csm-runtime-alb validate
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-alb plan -out=issue122-alb.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-alb apply issue122-alb.tfplan
```

Save the `alb_security_group_id` output for the Spot stack.

## 3. Optional disposable Spot Runtime host

Work from:

```text
infra/aws/csm-runtime-spot
```

Set:

- `csm_name`
- `environment`
- `vpc_id`
- `subnet_id`
- `alb_security_group_id`
- `user_data_file`, when running a smoke responder or bootstrap script

Validate and apply:

```bash
terraform -chdir=infra/aws/csm-runtime-spot init
terraform -chdir=infra/aws/csm-runtime-spot validate
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-spot plan -out=issue122-spot.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-spot apply issue122-spot.tfplan
```

Save the `instance_id` output.

## 4. Attach the Spot host to the ALB

Return to:

```text
infra/aws/csm-runtime-alb
```

Set `target_instance_id` to the Spot stack `instance_id`, then re-apply the ALB
stack:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-alb plan -out=issue122-alb-attach.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-alb apply issue122-alb-attach.tfplan
```

## 5. Prove the path

For a disposable ALB smoke endpoint:

```bash
curl -sS --max-time 20 -D - "https://origin-smoke.wuji.dev.csm.agent-logic.ai/v1/health"
```

For the permanent edge, run the live validator with the configured URLs:

```bash
bash adl/tools/validate_csm_public_edge_live.sh \
  --csm wuji \
  --environment dev \
  --observatory-url https://observatory.wuji.dev.csm.agent-logic.ai \
  --api-url https://api.wuji.dev.csm.agent-logic.ai \
  --wss-url wss://wss.wuji.dev.csm.agent-logic.ai/v1/observatory/ws \
  --wss-origin-hostname wuji.dev.csm.agent-logic.ai
```

The proof should show:

- Observatory HTTPS returns 200.
- Configured CORS origins are accepted exactly.
- Unconfigured browser origins are rejected.
- API/WSS origin routing reaches the configured Runtime origin.
- Disposable ALB smoke reaches the actual instance when that path is used.

## 6. Tear down disposable resources

Destroy disposable resources when the proof is complete:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-alb destroy
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/csm-runtime-spot destroy
```

After teardown, both disposable Terraform states should contain zero resources.
The reusable regional ACM certificate should remain.

Do not destroy the permanent public edge unless the CSM namespace itself is
being retired.

## 7. Common failure points

- Wrong AWS profile: verify the CSM edge/runtime resources are in the Agent
  Logic business account.
- Missing parent delegation: the CSM hosted zone name servers must be delegated
  from the parent zone.
- Missing ACM validation: CloudFront needs an issued `us-east-1` cert; ALB
  needs an issued regional cert.
- Origin TLS mismatch: the origin certificate must match the hostname used by
  CloudFront/API Gateway/ALB.
- CORS mismatch: browser origins must be exact origins; no paths, wildcards,
  credentials, queries, or fragments.
- Stale disposable target: reattach the ALB after recreating the Spot instance.

## 8. What this does not do

- It does not create NAT, GPU, Kubernetes, CodeBuild, or containers.
- It does not issue a new certificate on every run.
- It does not manage local Caddy or Let's Encrypt for wuji.
- It does not replace CSMctl Runtime process management.
- It does not make disposable Spot/ALB infrastructure permanent.

# CSM Public Edge Terraform

Issue #122 implements the Terraform-owned permanent public edge for CSM
instances. It replaces the earlier CloudFormation-oriented idea with Terraform
modules that can expose many named CSMs and can also stand up a disposable AWS
Runtime origin when local/GCP origins are unavailable.

The canonical development/staging hostname pattern is:

```text
<function>.<csm_name>.<environment>.csm.agent-logic.ai
```

The canonical production hostname pattern omits the environment segment and
uses the production CSM namespace:

```text
<function>.<csm_name>.csm.agent-logic.com
```

The first implementation creates three public edge surfaces:

- `observatory.<csm>.<env>` for static HTML Observatory assets.
- `api.<csm>.<env>` for HTTPS Runtime API traffic through API Gateway HTTP API.
- `wss.<csm>.<env>` for native WebSocket traffic through CloudFront to a
  WSS-capable host-only HTTPS Runtime origin. Path-bearing WSS origin URLs are
  rejected unless a future change adds explicit CloudFront `origin_path`
  support.

The module can either use an existing Route53 CSM namespace zone or create it
in the Agent Logic business AWS account and output name servers for one-time
parent-domain delegation. That keeps the permanent public edge in the business
account while still allowing wuji, nessus, GCP, or public ALB origins behind it.

Additional browser origins are supported through exact-origin allowlist values
for operator/development workflows. The Terraform module rejects wildcard,
pattern, path, and query-bearing origins before they can become API Gateway CORS
configuration.

All AWS-managed public certificates come from ACM. CloudFront viewer
certificates are in `us-east-1`; any regional AWS listener certificates are
regional ACM certificates. Non-AWS hosts that terminate origin TLS directly need
an explicitly approved origin certificate source.

The implementation supports pre-issued CloudFront viewer certificates through
`edge_acm_certificate_arn` and Runtime origin aliases through
`origin_cname_target`. The live wuji/dev proof used an issued
`*.wuji.dev.csm.agent-logic.ai` ACM certificate and published
`wuji.dev.csm.agent-logic.ai` as a CNAME to the existing wuji DDNS hostname.

The permanent public edge stack does not create Runtime compute, NAT gateways,
GPU, CodeBuild, Kubernetes, or containers. Disposable Runtime origin
infrastructure is split into separate quick-create/quick-destroy stacks:

- `infra/aws/csm-runtime-spot`: one small Spot EC2 instance with no NAT.
- `infra/aws/csm-runtime-alb`: one public HTTPS ALB origin that looks up and
  reuses an existing regional ACM certificate before any explicit first-time
  certificate creation path, with optional target attachment.

Local wuji testing may instead use Caddy as the Let's Encrypt TLS/reverse-proxy
layer in front of the existing CSMctl-managed Runtime. In that mode, Caddy owns
public origin TLS and CSMctl owns Runtime liveness.

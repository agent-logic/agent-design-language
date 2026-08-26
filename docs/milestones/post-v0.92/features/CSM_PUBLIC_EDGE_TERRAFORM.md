# CSM Public Edge Terraform

Issue #122 implements the Terraform-owned permanent public edge for CSM
instances. It replaces the earlier CloudFormation-oriented idea with a
Terraform module that can expose many named CSMs without creating compute.

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
  WSS-capable Runtime origin.

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

This issue does not create Runtime compute, EC2, Spot, GPU, CodeBuild,
Kubernetes, or NAT gateways.

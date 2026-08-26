# CSM Public Edge Terraform

Issue #122 implements the Terraform-owned permanent public edge for CSM
instances. It replaces the earlier CloudFormation-oriented idea with a
Terraform module that can expose many named CSMs without creating compute.

The canonical hostname pattern is:

```text
<service>.<csm_name>.<environment>.csm.agent-logic.ai
```

The first implementation creates three public edge surfaces:

- `observatory.<csm>.<env>` for static HTML Observatory assets.
- `api.<csm>.<env>` for HTTPS Runtime API traffic through API Gateway HTTP API.
- `wss.<csm>.<env>` for native WebSocket traffic through CloudFront to a
  WSS-capable Runtime origin.

All AWS-managed public certificates come from ACM. CloudFront viewer
certificates are in `us-east-1`; any regional AWS listener certificates are
regional ACM certificates. Non-AWS hosts that terminate origin TLS directly need
an explicitly approved origin certificate source.

This issue does not create Runtime compute, EC2, Spot, GPU, CodeBuild,
Kubernetes, or NAT gateways.

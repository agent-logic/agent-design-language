# CSM Public Edge Terraform

Issue #122 owns the permanent, Terraform-managed CSM public edge. This stack
does not create Runtime compute. It creates DNS, ACM-backed CloudFront edges,
WAF, S3 static Observatory hosting, API Gateway HTTP routing, and a native WSS
edge that can point at wuji, nessus, GCP, or a public AWS ALB.

Hostnames follow:

```text
<service>.<csm_name>.<environment>.csm.agent-logic.ai
```

For example:

```text
observatory.axioma.dev.csm.agent-logic.ai
api.axioma.dev.csm.agent-logic.ai
wss.axioma.dev.csm.agent-logic.ai
```

## Public routing

- `observatory.*` -> CloudFront + WAF -> private S3 static assets
- `api.*` -> CloudFront + WAF -> API Gateway HTTP API -> Runtime HTTPS origin
- `wss.*` -> CloudFront + WAF -> native WSS-capable HTTPS origin

API Gateway HTTP API is not used for WSS. API Gateway WebSocket API is a later
adapter-mode option only after Runtime connection-id and `@connections`
semantics are designed and reviewed.

## Validation

Static validation:

```sh
terraform fmt -check -recursive infra/aws/csm-public-edge
terraform -chdir=infra/aws/csm-public-edge init -backend=false
terraform -chdir=infra/aws/csm-public-edge validate
bash adl/tools/validate_csm_public_edge_static.sh
```

Live validation requires an operator-approved AWS account and selected Runtime
origins. Do not run `terraform apply` without an operator-reviewed plan.

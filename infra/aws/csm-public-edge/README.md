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

## Operator-configurable origins

`additional_allowed_origins` is intentionally supported for same-day operator
workflows where the Observatory UI is served from a separate exact browser
origin. Values must be exact origins only:

- `https://host[:port]`
- `http://localhost[:port]` for local development

Wildcards, paths, query strings, and origin patterns fail Terraform variable
validation before reaching API Gateway CORS.

`websocket_path_pattern` controls the ordered CloudFront WSS behavior path, with
`/v1/observatory/ws*` as the default. The WSS distribution still has a default
WSS origin behavior so the dedicated `wss.*` hostname can be used without
sharing the HTTP API distribution.

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

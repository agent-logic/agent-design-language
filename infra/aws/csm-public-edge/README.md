# CSM Public Edge Terraform

Issue #122 owns the permanent, Terraform-managed CSM public edge. This stack
does not create Runtime compute. It creates DNS, ACM-backed CloudFront edges,
WAF, S3 static Observatory hosting, API Gateway HTTP routing, and a native WSS
edge that can point at wuji, nessus, GCP, or a public AWS ALB.

Hostnames follow the CSM namespace convention. Non-production hostnames include
the environment segment:

```text
<function>.<csm_name>.<environment>.csm.agent-logic.ai
```

Production hostnames omit the environment segment and use the production CSM
zone:

```text
<function>.<csm_name>.csm.agent-logic.com
```

For example:

```text
observatory.wuji.dev.csm.agent-logic.ai
api.wuji.dev.csm.agent-logic.ai
wss.wuji.dev.csm.agent-logic.ai
wuji.dev.csm.agent-logic.ai

observatory.wuji.csm.agent-logic.com
api.wuji.csm.agent-logic.com
wss.wuji.csm.agent-logic.com
wuji.csm.agent-logic.com
```

## Public routing

- `observatory.*` -> CloudFront + WAF -> private S3 static assets
- `api.*` -> CloudFront + WAF -> API Gateway HTTP API -> Runtime HTTPS origin
- `wss.*` -> CloudFront + WAF -> native WSS-capable HTTPS origin

## DNS setup

For first-time setup in the Agent Logic business AWS account, set
`create_hosted_zone = true`. Terraform creates the CSM namespace zone, such as
`csm.agent-logic.ai`, and outputs `hosted_zone_name_servers`. Delegate that zone
from the parent domain once; subsequent applies can reuse the created zone or
set `hosted_zone_id` explicitly.

If the CSM namespace zone already exists in the target AWS account, leave
`create_hosted_zone = false` and let Terraform look it up by `zone_name`, or
provide `hosted_zone_id` to avoid ambiguity.

CloudFront can either use a Terraform-requested exact SAN ACM certificate or a
pre-issued `us-east-1` ACM wildcard through `edge_acm_certificate_arn`. The live
wuji/dev proof used `*.wuji.dev.csm.agent-logic.ai`, which covers
`observatory.wuji.dev.csm.agent-logic.ai`, `api.wuji.dev.csm.agent-logic.ai`,
and `wss.wuji.dev.csm.agent-logic.ai`.

`origin_cname_target` can publish the Runtime origin alias, for example
`wuji.dev.csm.agent-logic.ai -> wuji.agent-logic.ai`, without hardcoding the
current DDNS IP address.

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
origins. The edge can be applied before Runtime public exposure is live, but
API/WSS proof remains incomplete until the selected origin accepts public TLS
traffic from CloudFront/API Gateway.

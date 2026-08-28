# #122 Additional Allowed Origins Validation

Date: 2026-08-26

Purpose: prove the #122 Terraform module still supports additional exact browser
origins while rejecting unsafe wildcard/pattern CORS input.

## Positive smoke

Command shape:

```sh
terraform -chdir=infra/aws/csm-public-edge console \
  -var='environment=dev' \
  -var='csm_name=axioma' \
  -var='approved_aws_account_id=000000000000' \
  -var='observatory_asset_source=demos/html-observatory' \
  -var='runtime_origin_mode=external_https' \
  -var='runtime_origin_url=https://runtime-origin.example.com' \
  -var='wss_origin_mode=external_wss' \
  -var='wss_origin_https_url=https://wss-origin.example.com' \
  -var='wss_origin_hostname=wss-origin.example.com' \
  -var='additional_allowed_origins=["https://operator.example.com","http://localhost:5173"]'
```

Console expression:

```hcl
local.allowed_origins
```

Observed result:

```hcl
tolist([
  "https://observatory.axioma.dev.csm.agent-logic.ai",
  "https://operator.example.com",
  "http://localhost:5173",
])
```

Outcome: PASS. Exact additional origins are accepted and combined with the
default Observatory origin.

## Negative smoke

Command shape:

```sh
terraform -chdir=infra/aws/csm-public-edge console \
  -var='environment=dev' \
  -var='csm_name=axioma' \
  -var='approved_aws_account_id=000000000000' \
  -var='observatory_asset_source=demos/html-observatory' \
  -var='runtime_origin_mode=external_https' \
  -var='runtime_origin_url=https://runtime-origin.example.com' \
  -var='wss_origin_mode=external_wss' \
  -var='wss_origin_https_url=https://wss-origin.example.com' \
  -var='wss_origin_hostname=wss-origin.example.com' \
  -var='additional_allowed_origins=["*"]'
```

Observed result: Terraform rejected the variable value with the declared
validation error:

```text
additional_allowed_origins must contain exact https://host[:port] origins, or
http://localhost[:port] for local development; wildcards, paths, and patterns
are not allowed.
```

Outcome: PASS. Wildcard origins fail closed before API Gateway CORS
configuration.

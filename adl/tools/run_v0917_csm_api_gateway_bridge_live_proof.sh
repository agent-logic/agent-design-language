#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_v0917_csm_api_gateway_bridge_live_proof.sh --out <dir> --expected-account-sha256 <sha256> [options]

Creates or updates the WP-07 #5039 per-polis API Gateway bridge in the Agent
Logic AWS account, routes it through Lambda + SSM to the local loopback CSM
runtime API, and runs the runtime-owned CSM cloud-control proof.

Options:
  --out <dir>                       Required proof output directory.
  --expected-account-sha256 <sha>   Required approved Agent Logic account SHA-256.
  --profile <name>                  AWS profile. Default: agent-logic-admin.
  --region <region>                 AWS region. Default: us-west-2.
  --run-id <id>                     Run id. Default: wp07-5039-<utc>.
  --polis-id <id>                   Polis id. Default: csm-liveness-4976-full.
  --runtime-port <port>             Loopback CSM API port on the SSM node. Default: 19998.
  --ssm-node-name <name>            SSM ComputerName/Name to target. Default: wuji.local.
  --csm-bin <path>                  csm binary. Default: ADL_CSM_BIN or adl/target/debug/csm.
  --api-name <name>                 API Gateway name. Default: adl-csm-5039-polis-api.
  --lambda-name <name>              Lambda name. Default: adl-csm-5039-api-gateway-bridge.
  --role-name <name>                IAM role name. Default: adl-csm-5039-api-gateway-bridge-role.
  --event-bus <name>                EventBridge bus. Default: adl-csm.
  --stage <name>                    API stage. Default: prod.
  --operator-token-file <path>      Existing token file. Default: generated under --out/private.
  --help                            Show this help.
USAGE
}

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT=""
EXPECTED_ACCOUNT_SHA256="${ADL_AWS_CSM_API_GATEWAY_ACCOUNT_SHA256:-}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
RUN_ID="wp07-5039-$(date -u +%Y%m%dT%H%M%SZ)"
POLIS_ID="${ADL_CSM_POLIS_ID:-csm-liveness-4976-full}"
RUNTIME_PORT="${ADL_CSM_RUNTIME_API_PORT:-19998}"
SSM_NODE_NAME="${ADL_CSM_API_GATEWAY_SSM_NODE:-wuji.local}"
CSM_BIN="${ADL_CSM_BIN:-adl/target/debug/csm}"
API_NAME="${ADL_CSM_API_GATEWAY_NAME:-adl-csm-5039-polis-api}"
LAMBDA_NAME="${ADL_CSM_API_GATEWAY_LAMBDA_NAME:-adl-csm-5039-api-gateway-bridge}"
ROLE_NAME="${ADL_CSM_API_GATEWAY_ROLE_NAME:-adl-csm-5039-api-gateway-bridge-role}"
EVENT_BUS="${ADL_CSM_API_GATEWAY_EVENT_BUS:-adl-csm}"
STAGE="${ADL_CSM_API_GATEWAY_STAGE:-prod}"
OPERATOR_TOKEN_FILE="${ADL_CSM_API_GATEWAY_OPERATOR_TOKEN_FILE:-}"
AWS_BIN="${AWS_BIN:-aws}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="${2:?--out requires a value}"; shift ;;
    --expected-account-sha256) EXPECTED_ACCOUNT_SHA256="${2:?--expected-account-sha256 requires a value}"; shift ;;
    --profile) PROFILE="${2:?--profile requires a value}"; shift ;;
    --region) REGION="${2:?--region requires a value}"; shift ;;
    --run-id) RUN_ID="${2:?--run-id requires a value}"; shift ;;
    --polis-id) POLIS_ID="${2:?--polis-id requires a value}"; shift ;;
    --runtime-port) RUNTIME_PORT="${2:?--runtime-port requires a value}"; shift ;;
    --ssm-node-name) SSM_NODE_NAME="${2:?--ssm-node-name requires a value}"; shift ;;
    --csm-bin) CSM_BIN="${2:?--csm-bin requires a value}"; shift ;;
    --api-name) API_NAME="${2:?--api-name requires a value}"; shift ;;
    --lambda-name) LAMBDA_NAME="${2:?--lambda-name requires a value}"; shift ;;
    --role-name) ROLE_NAME="${2:?--role-name requires a value}"; shift ;;
    --event-bus) EVENT_BUS="${2:?--event-bus requires a value}"; shift ;;
    --stage) STAGE="${2:?--stage requires a value}"; shift ;;
    --operator-token-file) OPERATOR_TOKEN_FILE="${2:?--operator-token-file requires a value}"; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
  shift
done

if [ -z "$OUT" ] || [ -z "$EXPECTED_ACCOUNT_SHA256" ]; then
  usage >&2
  exit 2
fi
if ! command -v "$AWS_BIN" >/dev/null 2>&1; then
  echo "aws CLI not found; set AWS_BIN or install aws CLI" >&2
  exit 2
fi
if [ ! -x "$CSM_BIN" ]; then
  echo "csm binary not executable: $CSM_BIN" >&2
  exit 2
fi

mkdir -p "$OUT"

ACCOUNT="$("$AWS_BIN" sts get-caller-identity --profile "$PROFILE" --region "$REGION" --query Account --output text)"
ACCOUNT_SHA256="$(printf '%s' "$ACCOUNT" | shasum -a 256 | awk '{print $1}')"
ACCOUNT_HASH="$(printf '%s' "$ACCOUNT_SHA256" | cut -c1-16)"
if [ "$ACCOUNT_SHA256" != "$EXPECTED_ACCOUNT_SHA256" ]; then
  echo "AWS profile account hash does not match expected Agent Logic account hash" >&2
  exit 1
fi
printf 'PASS account_profile_resolved profile=%s account_matches_expected=true account_hash=%s\n' "$PROFILE" "$ACCOUNT_HASH" >&2

if [ -z "$OPERATOR_TOKEN_FILE" ]; then
  OPERATOR_TOKEN_FILE="$ROOT/.adl/local-artifacts/csm_api_gateway_bridge_5039/operator-token"
  if [ ! -s "$OPERATOR_TOKEN_FILE" ]; then
    mkdir -p "$(dirname "$OPERATOR_TOKEN_FILE")"
    chmod 700 "$(dirname "$OPERATOR_TOKEN_FILE")"
    python3 - "$OPERATOR_TOKEN_FILE" <<'PY'
import secrets, sys
from pathlib import Path
path = Path(sys.argv[1])
path.write_text(secrets.token_hex(32) + "\n")
path.chmod(0o600)
PY
  fi
fi
OPERATOR_TOKEN="$(tr -d '\n\r' <"$OPERATOR_TOKEN_FILE")"
if [ -z "$OPERATOR_TOKEN" ]; then
  echo "operator token file is empty" >&2
  exit 2
fi

SSM_INSTANCE_ID="$("$AWS_BIN" ssm describe-instance-information \
  --profile "$PROFILE" \
  --region "$REGION" \
  --output json \
  --query "InstanceInformationList[?PingStatus=='Online' && (ComputerName=='${SSM_NODE_NAME}' || Name=='${SSM_NODE_NAME}')].InstanceId | [0]" \
  | tr -d '"')"
if [ -z "$SSM_INSTANCE_ID" ] || [ "$SSM_INSTANCE_ID" = "null" ]; then
  echo "no online SSM managed node matched $SSM_NODE_NAME" >&2
  exit 1
fi
SSM_INSTANCE_HASH="$(printf '%s' "$SSM_INSTANCE_ID" | shasum -a 256 | awk '{print substr($1,1,16)}')"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

cat >"$WORK/trust-policy.json" <<'JSON'
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {"Service": "lambda.amazonaws.com"},
      "Action": "sts:AssumeRole"
    }
  ]
}
JSON

if ! "$AWS_BIN" iam get-role --profile "$PROFILE" --role-name "$ROLE_NAME" >/dev/null 2>&1; then
  "$AWS_BIN" iam create-role \
    --profile "$PROFILE" \
    --role-name "$ROLE_NAME" \
    --assume-role-policy-document "file://$WORK/trust-policy.json" \
    --tags Key=adl:milestone,Value=v0.91.7 Key=adl:issue,Value=5039 Key=adl:purpose,Value=csm-api-gateway-bridge >/dev/null
fi
ROLE_ARN="$("$AWS_BIN" iam get-role --profile "$PROFILE" --role-name "$ROLE_NAME" --query Role.Arn --output text)"

cat >"$WORK/role-policy.json" <<JSON
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": ["logs:CreateLogGroup", "logs:CreateLogStream", "logs:PutLogEvents"],
      "Resource": "arn:aws:logs:${REGION}:${ACCOUNT}:*"
    },
    {
      "Effect": "Allow",
      "Action": ["ssm:SendCommand"],
      "Resource": [
        "arn:aws:ssm:${REGION}:${ACCOUNT}:managed-instance/${SSM_INSTANCE_ID}",
        "arn:aws:ssm:${REGION}::document/AWS-RunShellScript"
      ]
    },
    {
      "Effect": "Allow",
      "Action": ["ssm:GetCommandInvocation"],
      "Resource": "*"
    }
  ]
}
JSON

"$AWS_BIN" iam put-role-policy \
  --profile "$PROFILE" \
  --role-name "$ROLE_NAME" \
  --policy-name adl-csm-api-gateway-bridge \
  --policy-document "file://$WORK/role-policy.json" >/dev/null

cat >"$WORK/lambda_function.py" <<'PY'
import json
import os
import time
import urllib.request

import boto3

ssm = boto3.client("ssm")


def _header(event, name):
    headers = event.get("headers") or {}
    lower = name.lower()
    for key, value in headers.items():
        if key.lower() == lower:
            return value
    return ""


def _response(status, body):
    return {
        "statusCode": status,
        "headers": {"content-type": "application/json"},
        "body": json.dumps(body, sort_keys=True),
    }


def handler(event, context):
    correlation_id = _header(event, "x-adl-correlation-id") or context.aws_request_id
    print(json.dumps({
        "schema": "adl.csm.api_gateway_bridge.lambda_event.v1",
        "stage": "received",
        "correlation_id": correlation_id,
        "path": event.get("rawPath") or event.get("path") or "/",
    }, sort_keys=True))
    expected = "Bearer " + os.environ["OPERATOR_TOKEN"]
    if _header(event, "authorization") != expected:
        print(json.dumps({
            "schema": "adl.csm.api_gateway_bridge.lambda_event.v1",
            "stage": "authorization",
            "result": "denied",
            "correlation_id": correlation_id,
        }, sort_keys=True))
        return _response(403, {
            "schema": "adl.csm.api_gateway_bridge.denied.v1",
            "status": "denied",
            "correlation_id": correlation_id,
        })
    path = event.get("rawPath") or event.get("path") or "/api-gateway-bridge"
    if not path.startswith("/"):
        path = "/" + path
    stage_prefix = "/" + os.environ.get("API_STAGE", "").strip("/")
    if stage_prefix != "/" and path.startswith(stage_prefix + "/"):
        path = path[len(stage_prefix):]
    url = f"http://127.0.0.1:{os.environ['RUNTIME_PORT']}{path}"
    command = f"/usr/bin/curl --silent --show-error --max-time 8 {url!r}"
    try:
        sent = ssm.send_command(
            InstanceIds=[os.environ["SSM_INSTANCE_ID"]],
            DocumentName="AWS-RunShellScript",
            Parameters={"commands": [command]},
            TimeoutSeconds=30,
        )
    except Exception as exc:
        print(json.dumps({
            "schema": "adl.csm.api_gateway_bridge.lambda_event.v1",
            "stage": "ssm_send_command",
            "status": "failed",
            "correlation_id": correlation_id,
            "error_class": type(exc).__name__,
        }, sort_keys=True))
        return _response(502, {
            "schema": "adl.csm.api_gateway_bridge.upstream_failure.v1",
            "status": "upstream_failure",
            "correlation_id": correlation_id,
            "send_command_failed": True,
        })
    command_id = sent["Command"]["CommandId"]
    last = None
    for _ in range(20):
        try:
            last = ssm.get_command_invocation(
                CommandId=command_id,
                InstanceId=os.environ["SSM_INSTANCE_ID"],
            )
        except Exception as exc:
            last = {"Status": "Pending", "StandardErrorContent": str(exc)}
        if last.get("Status") in {"Success", "Cancelled", "TimedOut", "Failed", "Cancelling"}:
            break
        time.sleep(0.5)
    status = last.get("Status") if last else "Unknown"
    stdout = (last or {}).get("StandardOutputContent") or ""
    stderr = (last or {}).get("StandardErrorContent") or ""
    print(json.dumps({
        "schema": "adl.csm.api_gateway_bridge.lambda_event.v1",
        "stage": "ssm_loopback",
        "status": status,
        "correlation_id": correlation_id,
    }, sort_keys=True))
    if status != "Success":
        return _response(502, {
            "schema": "adl.csm.api_gateway_bridge.upstream_failure.v1",
            "status": "upstream_failure",
            "correlation_id": correlation_id,
            "ssm_status": status,
            "stderr_present": bool(stderr),
        })
    try:
        body = json.loads(stdout)
    except Exception:
        body = {
            "schema": "adl.csm.api_gateway_bridge.upstream_malformed.v1",
            "status": "upstream_malformed",
            "correlation_id": correlation_id,
        }
        return _response(502, body)
    return _response(200, body)
PY

python3 - "$WORK/lambda.zip" "$WORK/lambda_function.py" <<'PY'
import sys
import zipfile
zip_path, source_path = sys.argv[1:]
with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    zf.write(source_path, "lambda_function.py")
PY

if "$AWS_BIN" lambda get-function --profile "$PROFILE" --region "$REGION" --function-name "$LAMBDA_NAME" >/dev/null 2>&1; then
  "$AWS_BIN" lambda update-function-code \
    --profile "$PROFILE" \
    --region "$REGION" \
    --function-name "$LAMBDA_NAME" \
    --zip-file "fileb://$WORK/lambda.zip" >/dev/null
  "$AWS_BIN" lambda wait function-updated \
    --profile "$PROFILE" \
    --region "$REGION" \
    --function-name "$LAMBDA_NAME"
  for attempt in 1 2 3 4 5 6; do
    if "$AWS_BIN" lambda update-function-configuration \
      --profile "$PROFILE" \
      --region "$REGION" \
      --function-name "$LAMBDA_NAME" \
      --role "$ROLE_ARN" \
      --timeout 20 \
      --environment "Variables={OPERATOR_TOKEN=$OPERATOR_TOKEN,SSM_INSTANCE_ID=$SSM_INSTANCE_ID,RUNTIME_PORT=$RUNTIME_PORT,API_STAGE=$STAGE}" >/dev/null; then
      break
    fi
    if [ "$attempt" -eq 6 ]; then
      echo "lambda update-function-configuration failed after IAM propagation retries" >&2
      exit 1
    fi
    sleep 5
  done
else
  for attempt in 1 2 3 4 5 6; do
    if "$AWS_BIN" lambda create-function \
      --profile "$PROFILE" \
      --region "$REGION" \
      --function-name "$LAMBDA_NAME" \
      --runtime python3.12 \
      --role "$ROLE_ARN" \
      --handler lambda_function.handler \
      --timeout 20 \
      --zip-file "fileb://$WORK/lambda.zip" \
      --environment "Variables={OPERATOR_TOKEN=$OPERATOR_TOKEN,SSM_INSTANCE_ID=$SSM_INSTANCE_ID,RUNTIME_PORT=$RUNTIME_PORT,API_STAGE=$STAGE}" \
      --tags adl:milestone=v0.91.7,adl:issue=5039,adl:purpose=csm-api-gateway-bridge >/dev/null; then
      break
    fi
    if [ "$attempt" -eq 6 ]; then
      echo "lambda create-function failed after IAM propagation retries" >&2
      exit 1
    fi
    sleep 5
  done
fi
"$AWS_BIN" lambda wait function-active \
  --profile "$PROFILE" \
  --region "$REGION" \
  --function-name "$LAMBDA_NAME"
LAMBDA_ARN="$("$AWS_BIN" lambda get-function --profile "$PROFILE" --region "$REGION" --function-name "$LAMBDA_NAME" --query Configuration.FunctionArn --output text)"
LAMBDA_INTEGRATION_URI="$LAMBDA_ARN"

API_ID="$("$AWS_BIN" apigatewayv2 get-apis \
  --profile "$PROFILE" \
  --region "$REGION" \
  --query "Items[?Name=='${API_NAME}'].ApiId | [0]" \
  --output text)"
if [ -z "$API_ID" ] || [ "$API_ID" = "None" ]; then
  API_ID="$("$AWS_BIN" apigatewayv2 create-api \
    --profile "$PROFILE" \
    --region "$REGION" \
    --name "$API_NAME" \
    --protocol-type HTTP \
    --tags adl:milestone=v0.91.7,adl:issue=5039,adl:purpose=csm-api-gateway-bridge \
    --query ApiId \
    --output text)"
fi
API_ID_HASH="$(printf '%s' "$API_ID" | shasum -a 256 | awk '{print substr($1,1,16)}')"

INTEGRATION_ID="$("$AWS_BIN" apigatewayv2 get-integrations \
  --profile "$PROFILE" \
  --region "$REGION" \
  --api-id "$API_ID" \
  --query "Items[?IntegrationUri=='${LAMBDA_INTEGRATION_URI}'].IntegrationId | [0]" \
  --output text)"
if [ -z "$INTEGRATION_ID" ] || [ "$INTEGRATION_ID" = "None" ]; then
  INTEGRATION_ID="$("$AWS_BIN" apigatewayv2 create-integration \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --integration-type AWS_PROXY \
    --integration-uri "$LAMBDA_INTEGRATION_URI" \
    --payload-format-version 2.0 \
    --query IntegrationId \
    --output text)"
else
  "$AWS_BIN" apigatewayv2 update-integration \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --integration-id "$INTEGRATION_ID" \
    --integration-uri "$LAMBDA_INTEGRATION_URI" \
    --payload-format-version 2.0 >/dev/null
fi

ROUTE_ID="$("$AWS_BIN" apigatewayv2 get-routes \
  --profile "$PROFILE" \
  --region "$REGION" \
  --api-id "$API_ID" \
  --query "Items[?RouteKey=='\$default'].RouteId | [0]" \
  --output text)"
if [ -z "$ROUTE_ID" ] || [ "$ROUTE_ID" = "None" ]; then
  "$AWS_BIN" apigatewayv2 create-route \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --route-key '$default' \
    --target "integrations/$INTEGRATION_ID" >/dev/null
else
  "$AWS_BIN" apigatewayv2 update-route \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --route-id "$ROUTE_ID" \
    --target "integrations/$INTEGRATION_ID" >/dev/null
fi

if ! "$AWS_BIN" apigatewayv2 get-stage --profile "$PROFILE" --region "$REGION" --api-id "$API_ID" --stage-name "$STAGE" >/dev/null 2>&1; then
  "$AWS_BIN" apigatewayv2 create-stage \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --stage-name "$STAGE" \
    --auto-deploy >/dev/null
else
  "$AWS_BIN" apigatewayv2 update-stage \
    --profile "$PROFILE" \
    --region "$REGION" \
    --api-id "$API_ID" \
    --stage-name "$STAGE" \
    --auto-deploy >/dev/null
fi

STATEMENT_ID="adl-csm-5039-api-gateway-bridge"
SOURCE_ARN="arn:aws:execute-api:${REGION}:${ACCOUNT}:${API_ID}/*"
"$AWS_BIN" lambda remove-permission \
  --profile "$PROFILE" \
  --region "$REGION" \
  --function-name "$LAMBDA_NAME" \
  --statement-id "$STATEMENT_ID" >/dev/null 2>&1 || true
"$AWS_BIN" lambda add-permission \
  --profile "$PROFILE" \
  --region "$REGION" \
  --function-name "$LAMBDA_NAME" \
  --statement-id "$STATEMENT_ID" \
  --action lambda:InvokeFunction \
  --principal apigateway.amazonaws.com \
  --source-arn "$SOURCE_ARN" >/dev/null

if ! "$AWS_BIN" events describe-event-bus --profile "$PROFILE" --region "$REGION" --name "$EVENT_BUS" >/dev/null 2>&1; then
  "$AWS_BIN" events create-event-bus \
    --profile "$PROFILE" \
    --region "$REGION" \
    --name "$EVENT_BUS" \
    --tags Key=adl:milestone,Value=v0.91.7 Key=adl:issue,Value=5039 Key=adl:purpose,Value=csm-api-gateway-bridge >/dev/null
fi
"$AWS_BIN" events put-rule \
  --profile "$PROFILE" \
  --region "$REGION" \
  --event-bus-name "$EVENT_BUS" \
  --name adl-csm-api-gateway-bridge-5039 \
  --event-pattern '{"source":["adl.csm"],"detail-type":["csm.api_gateway_bridge"]}' \
  --state ENABLED >/dev/null

INVOKE_URL="https://${API_ID}.execute-api.${REGION}.amazonaws.com/${STAGE}"
LOG_GROUP="/aws/lambda/${LAMBDA_NAME}"
SETUP_SUMMARY="$OUT/api_gateway_bridge_resource_summary.json"
python3 - "$SETUP_SUMMARY" "$RUN_ID" "$PROFILE" "$REGION" "$ACCOUNT_HASH" "$POLIS_ID" "$API_NAME" "$API_ID_HASH" "$LAMBDA_NAME" "$SSM_INSTANCE_HASH" "$EVENT_BUS" "$STAGE" "$RUNTIME_PORT" <<'PY'
import json
import sys
from pathlib import Path

(
    path,
    run_id,
    profile,
    region,
    account_hash,
    polis_id,
    api_name,
    api_id_hash,
    lambda_name,
    ssm_instance_hash,
    event_bus,
    stage,
    runtime_port,
) = sys.argv[1:]
summary = {
    "schema": "adl.csm.api_gateway_bridge_resource.v1",
    "issue": 5039,
    "run_id": run_id,
    "aws_profile": profile,
    "aws_region": region,
    "aws_account_hash": account_hash,
    "polis_id": polis_id,
    "ingress_model": "one_api_gateway_api_per_polis",
    "api_gateway": {
        "api_name": api_name,
        "api_id_hash": api_id_hash,
        "stage": stage,
        "route": "$default",
    },
    "lambda": {
        "function_name": lambda_name,
        "loopback_runtime_port": runtime_port,
        "ssm_instance_hash": ssm_instance_hash,
    },
    "eventbridge": {
        "bus": event_bus,
        "rule_name": "adl-csm-api-gateway-bridge-5039",
    },
    "redaction": {
        "raw_account_id_recorded": False,
        "raw_api_id_recorded": False,
        "raw_instance_id_recorded": False,
        "operator_token_recorded": False,
    },
}
Path(path).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
PY

ADL_AWS_PROFILE="$PROFILE" \
AWS_PROFILE="$PROFILE" \
ADL_AWS_REGION="$REGION" \
"$CSM_BIN" cloud-control api-gateway-bridge \
  --out "$OUT" \
  --run-id "$RUN_ID" \
  --profile "$PROFILE" \
  --region "$REGION" \
  --expected-account-sha256 "$EXPECTED_ACCOUNT_SHA256" \
  --polis-id "$POLIS_ID" \
  --api-id "$API_ID" \
  --stage "$STAGE" \
  --invoke-url "$INVOKE_URL" \
  --operator-token-file "$OPERATOR_TOKEN_FILE" \
  --cloudwatch-log-group "$LOG_GROUP" \
  --eventbridge-bus "$EVENT_BUS" \
  --json >"$OUT/csm_api_gateway_bridge_command_result.json"

python3 "$ROOT/adl/tools/validate_v0917_csm_api_gateway_bridge_proof.py" \
  "$OUT/api_gateway_bridge_summary.json" >/dev/null

printf 'PASS v0917_csm_api_gateway_bridge_live_proof out=%s run_id=%s api_hash=%s ssm_node_hash=%s\n' "$OUT" "$RUN_ID" "$API_ID_HASH" "$SSM_INSTANCE_HASH"

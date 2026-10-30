#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

BIN="$TMP/bin"
mkdir -p "$BIN"

cat >"$BIN/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "aws $*" >>"${FAKE_AWS_LOG:?}"
case "$1 $2" in
  "sts get-caller-identity")
    cat <<'JSON'
{"Account":"123456789012","Arn":"arn:aws:iam::123456789012:user/test","UserId":"test"}
JSON
    ;;
  "ssm describe-instance-information")
    cat <<'JSON'
{
  "InstanceInformationList": [
    {"InstanceId":"mi-11111111111111111","ComputerName":"wuji.local","PingStatus":"Online","PlatformType":"MacOS","PlatformName":"macOS"},
    {"InstanceId":"mi-22222222222222222","ComputerName":"nessus.WORKGROUP","PingStatus":"Online","PlatformType":"Windows","PlatformName":"Microsoft Windows 10 Home"},
    {"InstanceId":"mi-33333333333333333","ComputerName":"Opticon","PingStatus":"Online","PlatformType":"Linux","PlatformName":"QTS"}
  ]
}
JSON
    ;;
  "ssm send-command")
    host="unknown"
    case "$*" in
      *mi-11111111111111111*) host="wuji" ;;
      *mi-22222222222222222*) host="nessus" ;;
      *mi-33333333333333333*) host="opticon" ;;
    esac
    printf '{"Command":{"CommandId":"cmd-%s"}}\n' "$host"
    ;;
  "ssm get-command-invocation")
    host="unknown"
    case "$*" in
      *cmd-wuji*) host="wuji"; os="macOS"; label="wuji"; extra='"repo_present":true,"git_branch":"main","git_commit_short":"abc1234","ssm_agent_status":"not_reported"' ;;
      *cmd-nessus*) host="nessus"; os="Windows"; label="NESSUS"; extra='"repo_present":false,"git_branch":"unknown","git_commit_short":"unknown","ssm_agent_status":"Running"' ;;
      *cmd-opticon*) host="opticon"; os="QTS"; label="Opticon"; extra='"repo_present":false,"git_branch":"unknown","git_commit_short":"unknown","ssm_agent_status":"not_reported"' ;;
    esac
    stdout=$(printf '{"schema_version":"adl.local_polis_status.v1","generated_at_utc":"2026-07-06T00:00:00Z","host_label":"%s","os_name":"%s","os_version":"test","repo_name":"test",%s,"ssm_agent_installed":true}' "$label" "$os" "$extra")
    python3 - "$host" "$stdout" <<'PY'
import json, sys
host, stdout = sys.argv[1], sys.argv[2]
print(json.dumps({
  "CommandId": f"cmd-{host}",
  "InstanceId": "redacted-by-test",
  "Status": "Success",
  "StandardOutputContent": stdout,
  "StandardErrorContent": "",
  "CloudWatchOutputConfig": {
    "CloudWatchLogGroupName": "/adl/local-polis-ssm/4687",
    "CloudWatchOutputEnabled": True,
  },
}))
PY
    ;;
  "logs describe-log-streams")
    cat <<'JSON'
{
  "logStreams": [
    {"logStreamName":"mi-11111111111111111/cmd-wuji/awsrunShellScript/stdout"},
    {"logStreamName":"mi-22222222222222222/cmd-nessus/awsrunPowerShellScript/stdout"},
    {"logStreamName":"mi-33333333333333333/cmd-opticon/awsrunShellScript/stdout"}
  ]
}
JSON
    ;;
  *)
    echo "unexpected fake aws call: $*" >&2
    exit 1
    ;;
esac
SH
chmod +x "$BIN/aws"

export AWS_BIN="$BIN/aws"
export FAKE_AWS_LOG="$TMP/aws.log"
EXPECTED="$(printf '123456789012' | shasum -a 256 | awk '{print $1}')"

bash "$ROOT/adl/tools/run_wp08_local_polis_ssm_proof.sh" \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --expected-account-sha256 "$EXPECTED" \
  --run-id fixture-run >/tmp/wp08-local-polis-ssm-test.out

python3 "$ROOT/adl/tools/validate_wp08_local_polis_ssm_proof.py" \
  "$TMP/proof/local_polis_ssm_summary.json" >/dev/null

for expected_call in \
  "sts get-caller-identity" \
  "ssm describe-instance-information" \
  "ssm send-command" \
  "ssm get-command-invocation" \
  "logs describe-log-streams"
do
  grep -F "$expected_call" "$FAKE_AWS_LOG" >/dev/null
done

if grep -E '123456789012|mi-[0-9a-f]+|cmd-(wuji|nessus|opticon)' "$TMP/proof/local_polis_ssm_summary.json" >/dev/null; then
  echo "summary leaked raw identifier" >&2
  exit 1
fi

: >"$FAKE_AWS_LOG"
if bash "$ROOT/adl/tools/run_wp08_local_polis_ssm_proof.sh" \
  --out "$TMP/mismatch" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --expected-account-sha256 0000000000000000000000000000000000000000000000000000000000000000 \
  --run-id mismatch >/tmp/wp08-local-polis-ssm-mismatch.out 2>/tmp/wp08-local-polis-ssm-mismatch.err
then
  echo "expected account mismatch to fail" >&2
  exit 1
fi
if grep -E 'ssm describe-instance-information|ssm send-command|ssm get-command-invocation|logs describe-log-streams' "$FAKE_AWS_LOG" >/dev/null; then
  echo "account mismatch performed AWS mutation or post-account discovery" >&2
  cat "$FAKE_AWS_LOG" >&2
  exit 1
fi
grep -F "sts get-caller-identity" "$FAKE_AWS_LOG" >/dev/null

echo "PASS test_run_wp08_local_polis_ssm_proof"

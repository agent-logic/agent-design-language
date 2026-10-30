#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEFAULT_UTS_ROOT="$(cd "$ROOT/.." && pwd)/universal-tool-schema"
if [ ! -d "$DEFAULT_UTS_ROOT" ] && [ "$(basename "$(dirname "$ROOT")")" = ".worktrees" ]; then
  DEFAULT_UTS_ROOT="$(cd "$ROOT/../../.." && pwd)/universal-tool-schema"
fi
UTS_ROOT="${UTS_ROOT:-$DEFAULT_UTS_ROOT}"
ARTIFACT_DIR="$ROOT/.adl/local-artifacts/provider-fable5-5044"
MODEL_ID="claude-fable-5"
MAX_OUTPUT_TOKENS="${UTS_FABLE5_MAX_OUTPUT_TOKENS:-1024}"
RUN_ID="${UTS_FABLE5_RUN_ID:-issue-5044-fable5-uts}"
KEY_FILE="${ADL_ANTHROPIC_API_KEY_FILE:-}"
RUN_PROBE=1
RUN_PANEL=1
OVERWRITE=1

usage() {
  cat <<'USAGE'
Usage: run_fable5_uts_acceptance.sh [options]

Options:
  --uts-root <path>          universal-tool-schema checkout path.
  --artifact-dir <path>      Output artifact directory.
  --key-file <path>          Approved Anthropic key file to map to ANTHROPIC_API_KEY.
  --model <id>               Anthropic model id. Default: claude-fable-5.
  --max-output-tokens <n>    ADL provider-adapter output token budget. Default: 1024.
  --run-id <id>              Stable UTS run id. Default: issue-5044-fable5-uts.
  --skip-probe               Skip hosted availability probe.
  --skip-panel               Skip full regular,uts_only panel.
  --no-overwrite             Do not pass --overwrite to the UTS runner.
  -h, --help                 Show this help.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --uts-root) UTS_ROOT="$2"; shift 2 ;;
    --artifact-dir) ARTIFACT_DIR="$2"; shift 2 ;;
    --key-file) KEY_FILE="$2"; shift 2 ;;
    --model) MODEL_ID="$2"; shift 2 ;;
    --max-output-tokens) MAX_OUTPUT_TOKENS="$2"; shift 2 ;;
    --run-id) RUN_ID="$2"; shift 2 ;;
    --skip-probe) RUN_PROBE=0; shift ;;
    --skip-panel) RUN_PANEL=0; shift ;;
    --no-overwrite) OVERWRITE=0; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

case "$MAX_OUTPUT_TOKENS" in
  ''|*[!0-9]*) echo "--max-output-tokens must be a positive integer" >&2; exit 2 ;;
esac
if [ "$MAX_OUTPUT_TOKENS" -le 0 ]; then
  echo "--max-output-tokens must be a positive integer" >&2
  exit 2
fi

if [ ! -d "$UTS_ROOT" ]; then
  echo "missing UTS checkout: $UTS_ROOT" >&2
  exit 2
fi

mkdir -p "$ARTIFACT_DIR"
SELECTOR="$ARTIFACT_DIR/fable5_selector.txt"
printf 'hosted:adl-anthropic:%s\n' "$MODEL_ID" >"$SELECTOR"

ADAPTER_BIN="${ADL_PROVIDER_ADAPTER_BIN:-$ROOT/adl/target/debug/adl-provider-adapter}"
if [ ! -x "$ADAPTER_BIN" ]; then
  echo "missing executable provider adapter: $ADAPTER_BIN" >&2
  exit 2
fi

if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
  if [ -z "$KEY_FILE" ]; then
    echo "ANTHROPIC_API_KEY is unset and --key-file was not provided" >&2
    exit 2
  fi
  if [ ! -r "$KEY_FILE" ]; then
    echo "Anthropic key file is not readable" >&2
    exit 2
  fi
  ANTHROPIC_API_KEY="$(tr -d '\r\n' < "$KEY_FILE")"
  export ANTHROPIC_API_KEY
fi

export ADL_HOME="$ROOT"
export UTS_ADL_PROVIDER_ADAPTER_COMMAND="python3 $ROOT/adl/tools/adl_provider_adapter_with_budget.py --adapter $ADAPTER_BIN --max-output-tokens $MAX_OUTPUT_TOKENS --"

OVERWRITE_ARGS=()
if [ "$OVERWRITE" -eq 1 ]; then
  OVERWRITE_ARGS=(--overwrite)
fi

python3 "$UTS_ROOT/tools/benchmark/deterministic_self_check.py" >"$ARTIFACT_DIR/fable5_self_check.json"

if [ "$RUN_PROBE" -eq 1 ]; then
  python3 "$UTS_ROOT/tools/uts_benchmark_runner.py" \
    probe-hosted \
    "$SELECTOR" \
    "$ARTIFACT_DIR/fable5_probe_results.json" \
    --run-id "${RUN_ID}-probe" \
    "${OVERWRITE_ARGS[@]}"
fi

if [ "$RUN_PANEL" -eq 1 ]; then
  python3 "$UTS_ROOT/tools/uts_benchmark_runner.py" \
    hosted \
    "$SELECTOR" \
    "$ARTIFACT_DIR/fable5_uts_results.json" \
    --lanes regular,uts_only \
    --run-id "$RUN_ID" \
    "${OVERWRITE_ARGS[@]}"
fi

echo "PASS fable5_uts_acceptance model=$MODEL_ID artifact_dir=$ARTIFACT_DIR max_output_tokens=$MAX_OUTPUT_TOKENS"

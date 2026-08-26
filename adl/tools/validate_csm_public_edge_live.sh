#!/usr/bin/env bash
set -euo pipefail

OBSERVATORY_URL=""
API_URL=""
WSS_URL=""
WSS_ORIGIN_HOSTNAME=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --observatory-url) OBSERVATORY_URL="${2:-}"; shift 2 ;;
    --api-url) API_URL="${2:-}"; shift 2 ;;
    --wss-url) WSS_URL="${2:-}"; shift 2 ;;
    --wss-origin-hostname) WSS_ORIGIN_HOSTNAME="${2:-}"; shift 2 ;;
    --csm|--environment) shift 2 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done

fail() {
  printf 'validate_csm_public_edge_live: FAIL: %s\n' "$*" >&2
  exit 1
}

contains_header() {
  local headers="$1"
  local expected="$2"
  printf '%s\n' "$headers" | grep -F "$expected" >/dev/null
}

test -n "$OBSERVATORY_URL" || fail "missing --observatory-url"
test -n "$API_URL" || fail "missing --api-url"
test -n "$WSS_URL" || fail "missing --wss-url"
test -n "$WSS_ORIGIN_HOSTNAME" || fail "missing --wss-origin-hostname"

case "$OBSERVATORY_URL" in https://*) ;; *) fail "observatory URL must be https" ;; esac
case "$API_URL" in https://*) ;; *) fail "api URL must be https" ;; esac
case "$WSS_URL" in wss://*) ;; *) fail "wss URL must be wss" ;; esac

curl -fsSI "$OBSERVATORY_URL" >/dev/null || fail "Observatory HTTPS endpoint did not return 2xx/3xx: $OBSERVATORY_URL"

ALLOWED_PREFLIGHT="$(
  curl -sSI -X OPTIONS \
    -H "Origin: $OBSERVATORY_URL" \
    -H "Access-Control-Request-Method: GET" \
    "$API_URL/v1/health"
)" || fail "allowed-origin CORS preflight failed at $API_URL"

contains_header "$ALLOWED_PREFLIGHT" "access-control-allow-origin: $OBSERVATORY_URL" \
  || fail "allowed-origin CORS preflight did not echo $OBSERVATORY_URL"

REJECTED_PREFLIGHT="$(
  curl -sSI -X OPTIONS \
    -H "Origin: https://evil.example.com" \
    -H "Access-Control-Request-Method: GET" \
    "$API_URL/v1/health"
)" || true

if contains_header "$REJECTED_PREFLIGHT" "access-control-allow-origin:"; then
  fail "rejected-origin CORS preflight unexpectedly emitted access-control-allow-origin"
fi

if ! curl -fsSI "$API_URL/v1/openapi.json" >/dev/null; then
  fail "Runtime API origin is not currently reachable through the edge at $API_URL/v1/openapi.json; edge DNS/TLS/CORS passed, but the selected Runtime origin must be live before API/WSS proof can pass"
fi

printf 'validate_csm_public_edge_live: partial PASS (HTTPS endpoints reachable; WSS handshake probe must be run with the approved Runtime probe client for %s / origin %s)\n' "$WSS_URL" "$WSS_ORIGIN_HOSTNAME"

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

test -n "$OBSERVATORY_URL" || fail "missing --observatory-url"
test -n "$API_URL" || fail "missing --api-url"
test -n "$WSS_URL" || fail "missing --wss-url"
test -n "$WSS_ORIGIN_HOSTNAME" || fail "missing --wss-origin-hostname"

case "$OBSERVATORY_URL" in https://*) ;; *) fail "observatory URL must be https" ;; esac
case "$API_URL" in https://*) ;; *) fail "api URL must be https" ;; esac
case "$WSS_URL" in wss://*) ;; *) fail "wss URL must be wss" ;; esac

curl -fsSI "$OBSERVATORY_URL" >/dev/null
curl -fsSI "$API_URL/v1/openapi.json" >/dev/null

printf 'validate_csm_public_edge_live: partial PASS (HTTPS endpoints reachable; WSS handshake probe must be run with the approved Runtime probe client for %s / origin %s)\n' "$WSS_URL" "$WSS_ORIGIN_HOSTNAME"

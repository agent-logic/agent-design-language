#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
CLASSIFIER="$ROOT/adl/tools/mechanical_coverage_fallout.py"
MAPPING="$ROOT/adl/config/mechanical_coverage_fallout.v1.json"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT
write_diff() {
  local added="$1"
  printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,2 @@' ' context' "$added" >"$TEMP_ROOT/change.diff"
}
write_proof() {
  printf '%s\n' '{"compile_hunks":{"adl-runtime/src/distributed/transport/core.rs:new-20":{"command":["cargo","check","--manifest-path","adl-runtime/Cargo.toml"],"outcome":"passed"}},"behavioral_tests":{"EstablishedRuntimeAuthority":["distributed::transport::tests::established_runtime_authority_rejects_unauthorized_member"],"TransportAuthorization":["distributed::transport::tests::transport_authorization_requires_authority_bound_access"]}}' >"$TEMP_ROOT/proof.json"
}
accept() {
  python3 "$CLASSIFIER" --diff "$TEMP_ROOT/change.diff" --mapping "$MAPPING" --proof "$TEMP_ROOT/proof.json" --receipt "$TEMP_ROOT/receipt.json" >/dev/null
  jq -e '.file == "adl-runtime/src/distributed/transport/core.rs" and .token == "AUTHORITY_BOUND_CERTIFICATE_ACCESS" and (.owner | index("EstablishedRuntimeAuthority")) != null and (.tests.EstablishedRuntimeAuthority | length) > 0 and (.hunks | length) == 1 and .rationale != "" and .coverage_authority == "pr_fast_non_authoritative"' "$TEMP_ROOT/receipt.json" >/dev/null
}
reject() {
  if python3 "$CLASSIFIER" --diff "$TEMP_ROOT/change.diff" --mapping "$MAPPING" --proof "$TEMP_ROOT/proof.json" --receipt "$TEMP_ROOT/receipt.json" >/dev/null 2>&1; then echo "expected rejection: $1" >&2; exit 1; fi
}
write_proof
printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,4 @@' '-use super::certificates::{AuthorityCertificate, CertificatePurpose};' '+use super::certificates::{' '+    AuthorityCertificate, CertificatePurpose,' '+    AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '+};' >"$TEMP_ROOT/change.diff"
accept
jq -e '.hunks[0].kind == "import_only"' "$TEMP_ROOT/receipt.json" >/dev/null
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
accept
jq -e '.hunks[0].kind == "argument_pass_through"' "$TEMP_ROOT/receipt.json" >/dev/null
for fixture in '+                    authorize_with_new_semantics(),' '+                    if authorized { allow() }' '+                    match route { Some(v) => v, None => deny() }' '+                    self.authorized = true;' '+                    return Err(AuthorizationError::Denied);'; do write_diff "$fixture"; reject "$fixture"; done
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
jq '.compile_hunks = {}' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/no-compile.json" && mv "$TEMP_ROOT/no-compile.json" "$TEMP_ROOT/proof.json"
reject "missing hunk compile proof"
write_proof
jq 'del(.behavioral_tests.EstablishedRuntimeAuthority)' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/no-owner.json" && mv "$TEMP_ROOT/no-owner.json" "$TEMP_ROOT/proof.json"
reject "missing EstablishedRuntimeAuthority behavioral proof"
write_proof
sed 's#adl-runtime/src/distributed/transport/core.rs#adl-runtime/src/distributed/transport/unmapped.rs#g' "$TEMP_ROOT/change.diff" >"$TEMP_ROOT/unmapped.diff" && mv "$TEMP_ROOT/unmapped.diff" "$TEMP_ROOT/change.diff"
reject "unmapped file"

# End-to-end gate integration: an exact mapped diff below 80% is accepted only
# with both hunk compile proofs and both owning API behavioral proofs.
GATE_ROOT="$TEMP_ROOT/gate-repo"
mkdir -p "$GATE_ROOT/adl/tools" "$GATE_ROOT/adl/config" "$GATE_ROOT/adl-runtime/src/distributed/transport"
cp "$ROOT/adl/tools/check_coverage_impact.sh" "$CLASSIFIER" "$GATE_ROOT/adl/tools/"
cp "$MAPPING" "$GATE_ROOT/adl/config/"
git -C "$GATE_ROOT" init -q
git -C "$GATE_ROOT" config user.name fixture
git -C "$GATE_ROOT" config user.email fixture@example.invalid
{
  echo 'use super::certificates::{AuthorityCertificate, CertificatePurpose};'
  for _ in $(seq 1 20); do echo '// context'; done
  echo 'authorize('
  echo '    holder,'
  echo ');'
} >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
git -C "$GATE_ROOT" add .
git -C "$GATE_ROOT" commit -qm baseline
{
  echo 'use super::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};'
  for _ in $(seq 1 20); do echo '// context'; done
  echo 'authorize('
  echo '    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
  echo '    holder,'
  echo ');'
} >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
git -C "$GATE_ROOT" diff >"$TEMP_ROOT/gate.diff"
first_hunk="$(sed -n 's/^@@[^+]*+\([0-9][0-9]*\).*$/adl-runtime\/src\/distributed\/transport\/core.rs:new-\1/p' "$TEMP_ROOT/gate.diff" | sed -n '1p')"
second_hunk="$(sed -n 's/^@@[^+]*+\([0-9][0-9]*\).*$/adl-runtime\/src\/distributed\/transport\/core.rs:new-\1/p' "$TEMP_ROOT/gate.diff" | sed -n '2p')"
jq -n --arg first "$first_hunk" --arg second "$second_hunk" '{compile_hunks:{($first):{command:["cargo","check","--manifest-path","adl-runtime/Cargo.toml"],outcome:"passed"},($second):{command:["cargo","check","--manifest-path","adl-runtime/Cargo.toml"],outcome:"passed"}},behavioral_tests:{EstablishedRuntimeAuthority:["established_runtime_authority_behavior"],TransportAuthorization:["transport_authorization_behavior"]}}' >"$TEMP_ROOT/gate-proof.json"
printf '%s\n' '{"data":[{"files":[{"filename":"adl-runtime/src/distributed/transport/core.rs","summary":{"lines":{"covered":1,"count":10}}}]}]}' >"$TEMP_ROOT/summary.json"
git -C "$GATE_ROOT" -c core.fileMode=false diff --quiet -- adl/tools/check_coverage_impact.sh || true
(cd "$GATE_ROOT" && bash adl/tools/check_coverage_impact.sh --base HEAD --include-working-tree --summary "$TEMP_ROOT/summary.json" --mechanical-proof "$TEMP_ROOT/gate-proof.json" --mechanical-receipt-dir "$TEMP_ROOT/receipts") >/dev/null
jq -e '.classification == "mechanical_compile_fallout" and (.hunks | length) == 2' "$TEMP_ROOT/receipts/adl-runtime__src__distributed__transport__core.rs.json" >/dev/null

echo "PASS: mechanical compile-fallout classifier"

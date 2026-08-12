#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
CLASSIFIER="$ROOT/adl/tools/mechanical_coverage_fallout.py"
MAPPING="$ROOT/adl/config/mechanical_coverage_fallout.v1.json"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT
BASE_REV="1111111111111111111111111111111111111111"
HEAD_REV="2222222222222222222222222222222222222222"

digest() { shasum -a 256 "$1" | awk '{print $1}'; }
write_diff() {
  printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,2 @@' ' context' "$1" >"$TEMP_ROOT/change.diff"
}
write_result() {
  local kind="$1" subject="$2" stem="$3" diff_digest="$4"
  printf 'verified %s evidence for %s\n' "$kind" "$subject" >"$TEMP_ROOT/$stem.log"
  local evidence_digest
  evidence_digest="$(digest "$TEMP_ROOT/$stem.log")"
  jq -n --arg kind "$kind" --arg subject "$subject" --arg base "$BASE_REV" --arg head "$HEAD_REV" --arg diff "$diff_digest" --arg evidence "$stem.log" --arg evidence_digest "$evidence_digest" '{schema:"adl.mechanical_proof_result.v1",kind:$kind,subject:$subject,base_revision:$base,head_revision:$head,diff_sha256:$diff,command:["verified-runner",$kind,$subject],exit_code:0,evidence:$evidence,evidence_sha256:$evidence_digest}' >"$TEMP_ROOT/$stem.json"
}
write_proof() {
  local diff_digest mapping_digest
  diff_digest="$(digest "$TEMP_ROOT/change.diff")"
  mapping_digest="$(digest "$MAPPING")"
  jq -n --arg base "$BASE_REV" --arg head "$HEAD_REV" --arg diff "$diff_digest" --arg mapping "$mapping_digest" '{schema:"adl.mechanical_coverage_proof.v1",base_revision:$base,head_revision:$head,diff_sha256:$diff,mapping_sha256:$mapping,compile_results:{},behavioral_results:{}}' >"$TEMP_ROOT/proof.json"
  while IFS= read -r hunk; do
    [ -n "$hunk" ] || continue
    local stem result_digest
    stem="compile-$(printf %s "$hunk" | shasum -a 256 | cut -c1-12)"
    write_result compile "$hunk" "$stem" "$diff_digest"
    result_digest="$(digest "$TEMP_ROOT/$stem.json")"
    jq --arg hunk "$hunk" --arg artifact "$stem.json" --arg digest "$result_digest" '.compile_results[$hunk]={artifact:$artifact,sha256:$digest}' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/proof.next" && mv "$TEMP_ROOT/proof.next" "$TEMP_ROOT/proof.json"
  done < <(sed -n 's/^@@[^+]*+\([0-9][0-9]*\).*$/adl-runtime\/src\/distributed\/transport\/core.rs:new-\1/p' "$TEMP_ROOT/change.diff")
  for owner in EstablishedRuntimeAuthority TransportAuthorization; do
    local stem result_digest
    stem="behavior-$owner"
    write_result behavior "$owner" "$stem" "$diff_digest"
    result_digest="$(digest "$TEMP_ROOT/$stem.json")"
    jq --arg owner "$owner" --arg artifact "$stem.json" --arg digest "$result_digest" '.behavioral_results[$owner]={artifact:$artifact,sha256:$digest}' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/proof.next" && mv "$TEMP_ROOT/proof.next" "$TEMP_ROOT/proof.json"
  done
}
run_classifier() {
  python3 "$CLASSIFIER" --diff "$TEMP_ROOT/change.diff" --mapping "$MAPPING" --proof "$TEMP_ROOT/proof.json" --receipt "$TEMP_ROOT/receipt.json" --base-revision "$BASE_REV" --head-revision "$HEAD_REV"
}
accept() {
  write_proof
  run_classifier >/dev/null
  jq -e '.base_revision != "" and .head_revision != "" and .diff_sha256 != "" and .mapping_sha256 != "" and .proof_manifest_sha256 != "" and .file == "adl-runtime/src/distributed/transport/core.rs" and .token == "AUTHORITY_BOUND_CERTIFICATE_ACCESS" and (.owner | index("EstablishedRuntimeAuthority")) != null and .tests.EstablishedRuntimeAuthority.result_sha256 != "" and (.hunks | length) > 0 and (.hunks | all(.content_sha256 != "" and .compile_result_sha256 != "" and .compile_evidence_sha256 != "")) and .rationale != "" and .coverage_authority == "pr_fast_non_authoritative"' "$TEMP_ROOT/receipt.json" >/dev/null
}
reject() {
  write_proof
  if run_classifier >/dev/null 2>&1; then echo "expected rejection: $1" >&2; exit 1; fi
}

printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,4 @@' '-use super::certificates::{AuthorityCertificate, CertificatePurpose};' '+use super::certificates::{' '+    AuthorityCertificate, CertificatePurpose,' '+    AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '+};' >"$TEMP_ROOT/change.diff"
accept
jq -e '.hunks[0].kind == "import_only"' "$TEMP_ROOT/receipt.json" >/dev/null
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
accept
jq -e '.hunks[0].kind == "argument_pass_through"' "$TEMP_ROOT/receipt.json" >/dev/null

for fixture in '+                    authorize_with_new_semantics(),' '+                    if authorized { allow() }' '+                    match route { Some(v) => v, None => deny() }' '+                    self.authorized = true;' '+                    return Err(AuthorizationError::Denied);'; do write_diff "$fixture"; reject "$fixture"; done
for changed_import in \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{CertificatePurpose, AuthorityCertificate, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use other::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate as AC, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate, AUTHORITY_BOUND_CERTIFICATE_ACCESS};'; do
  old="${changed_import%%|*}"; new="${changed_import#*|}"
  printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,1 @@' "$old" "$new" >"$TEMP_ROOT/change.diff"
  reject "semantic import rewrite: $changed_import"
done

write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
write_proof
jq '.compile_results = {}' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/proof.next" && mv "$TEMP_ROOT/proof.next" "$TEMP_ROOT/proof.json"
if run_classifier >/dev/null 2>&1; then echo 'expected missing compile artifact rejection' >&2; exit 1; fi
write_proof
artifact="$(jq -r '.compile_results[] | .artifact' "$TEMP_ROOT/proof.json" | head -1)"
printf 'tamper\n' >>"$TEMP_ROOT/$artifact"
if run_classifier >/dev/null 2>&1; then echo 'expected compile result substitution rejection' >&2; exit 1; fi
write_proof
jq 'del(.behavioral_results.EstablishedRuntimeAuthority)' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/proof.next" && mv "$TEMP_ROOT/proof.next" "$TEMP_ROOT/proof.json"
if run_classifier >/dev/null 2>&1; then echo 'expected owner proof rejection' >&2; exit 1; fi
write_proof
jq '.head_revision="3333333333333333333333333333333333333333"' "$TEMP_ROOT/proof.json" >"$TEMP_ROOT/proof.next" && mv "$TEMP_ROOT/proof.next" "$TEMP_ROOT/proof.json"
if run_classifier >/dev/null 2>&1; then echo 'expected revision replay rejection' >&2; exit 1; fi
write_proof
sed 's#adl-runtime/src/distributed/transport/core.rs#adl-runtime/src/distributed/transport/unmapped.rs#g' "$TEMP_ROOT/change.diff" >"$TEMP_ROOT/change.next" && mv "$TEMP_ROOT/change.next" "$TEMP_ROOT/change.diff"
reject "unmapped file"

# Gate integration binds the exact worktree diff identity into the receipt and
# accepts a 10% file only when verified artifacts match that exact diff.
GATE_ROOT="$TEMP_ROOT/gate-repo"
mkdir -p "$GATE_ROOT/adl/tools" "$GATE_ROOT/adl/config" "$GATE_ROOT/adl-runtime/src/distributed/transport"
cp "$ROOT/adl/tools/check_coverage_impact.sh" "$CLASSIFIER" "$GATE_ROOT/adl/tools/"
cp "$MAPPING" "$GATE_ROOT/adl/config/"
git -C "$GATE_ROOT" init -q
git -C "$GATE_ROOT" config user.name fixture
git -C "$GATE_ROOT" config user.email fixture@example.invalid
{ printf '%s\n' 'use super::certificates::{AuthorityCertificate, CertificatePurpose};'; for _ in $(seq 1 20); do echo '// unchanged context'; done; printf '%s\n' 'authorize(' '    holder,' ');'; } >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
git -C "$GATE_ROOT" add .
git -C "$GATE_ROOT" commit -qm baseline
{ printf '%s\n' 'use super::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};'; for _ in $(seq 1 20); do echo '// unchanged context'; done; printf '%s\n' 'authorize(' '    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '    holder,' ');'; } >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
git -C "$GATE_ROOT" diff -- adl-runtime/src/distributed/transport/core.rs >"$TEMP_ROOT/change.diff"
BASE_REV="$(git -C "$GATE_ROOT" rev-parse HEAD)"
HEAD_REV="worktree:$(digest "$TEMP_ROOT/change.diff")"
write_proof
printf '%s\n' '{"data":[{"files":[{"filename":"adl-runtime/src/distributed/transport/core.rs","summary":{"lines":{"covered":1,"count":10}}}]}]}' >"$TEMP_ROOT/summary.json"
(cd "$GATE_ROOT" && bash adl/tools/check_coverage_impact.sh --base HEAD --include-working-tree --summary "$TEMP_ROOT/summary.json" --mechanical-proof "$TEMP_ROOT/proof.json" --mechanical-receipt-dir "$TEMP_ROOT/receipts") >/dev/null
jq -e --arg base "$BASE_REV" --arg head "$HEAD_REV" '.base_revision == $base and .head_revision == $head and .diff_sha256 != "" and (.hunks | length) == 2' "$TEMP_ROOT/receipts/adl-runtime__src__distributed__transport__core.rs.json" >/dev/null

echo "PASS: mechanical compile-fallout classifier"

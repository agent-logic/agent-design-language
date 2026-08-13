#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
CLASSIFIER="$ROOT/adl/tools/mechanical_coverage_fallout.py"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT
BASE_REV="1111111111111111111111111111111111111111"
HEAD_REV="2222222222222222222222222222222222222222"
MAPPING="$TEMP_ROOT/mapping.json"

write_mapping() {
  jq -n --arg compile "${1:-/usr/bin/true}" '{schema:"adl.mechanical_coverage_fallout_mapping.v1",mappings:[{file:"adl-runtime/src/distributed/transport/core.rs",token:"AUTHORITY_BOUND_CERTIFICATE_ACCESS",import_path:"super::certificates",callee:"authorize",owners:["EstablishedRuntimeAuthority","TransportAuthorization"],compile_command:[$compile],behavior_commands:{EstablishedRuntimeAuthority:["/usr/bin/true"],TransportAuthorization:["/usr/bin/true"]},rationale:"fixture governed token pass-through"}]}' >"$MAPPING"
}
write_diff() {
  local context="${2:-            .authorize(}"
  printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,2 @@' " $context" "$1" >"$TEMP_ROOT/change.diff"
}
run_classifier() {
  python3 "$CLASSIFIER" --diff "$TEMP_ROOT/change.diff" --mapping "$MAPPING" --receipt "$TEMP_ROOT/receipt.json" --repo-root "$ROOT" --evidence-dir "$TEMP_ROOT/results" --base-revision "$BASE_REV" --head-revision "$HEAD_REV"
}
accept() {
  rm -rf "$TEMP_ROOT/results"
  run_classifier >/dev/null
  jq -e '.schema == "adl.mechanical_coverage_fallout.v2" and .execution_provenance == "classifier_executed_governed_commands" and .base_revision != "" and .head_revision != "" and .diff_sha256 != "" and .mapping_sha256 != "" and .execution_results_sha256 != "" and .callee == "authorize" and (.owner | index("EstablishedRuntimeAuthority")) != null and .tests.EstablishedRuntimeAuthority.result_sha256 != "" and (.hunks | all(.content_sha256 != "" and .compile_result_sha256 != "" and .compile_evidence_sha256 != "")) and .coverage_authority == "pr_fast_non_authoritative"' "$TEMP_ROOT/receipt.json" >/dev/null
  jq -e '.producer == "mechanical_coverage_fallout.py:subprocess" and .exit_code == 0' "$TEMP_ROOT/results"/*.json >/dev/null
}
reject() {
  rm -rf "$TEMP_ROOT/results"
  if run_classifier >/dev/null 2>&1; then echo "expected rejection: $1" >&2; exit 1; fi
}

write_mapping
printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,4 @@' '-use super::certificates::{AuthorityCertificate, CertificatePurpose};' '+use super::certificates::{' '+    AuthorityCertificate, CertificatePurpose,' '+    AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '+};' >"$TEMP_ROOT/change.diff"
accept
jq -e '.hunks[0].kind == "import_only"' "$TEMP_ROOT/receipt.json" >/dev/null
printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,2 @@' ' use super::lease::VoterAuthority;' '+use super::certificates::AUTHORITY_BOUND_CERTIFICATE_ACCESS;' >"$TEMP_ROOT/change.diff"
accept
jq -e '.hunks[0].kind == "import_only"' "$TEMP_ROOT/receipt.json" >/dev/null
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
accept
jq -e '.hunks[0].kind == "argument_pass_through"' "$TEMP_ROOT/receipt.json" >/dev/null

for fixture in '+                    authorize_with_new_semantics(),' '+                    if authorized { allow() }' '+                    match route { Some(v) => v, None => deny() }' '+                    self.authorized = true;' '+                    return Err(AuthorizationError::Denied);'; do write_diff "$fixture"; reject "$fixture"; done
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '            .unrelated_call('
reject 'token passed to unrelated call'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '            authorize_value = ('
reject 'callee name outside invocation'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '            // .authorize('
reject 'callee-shaped comment'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '            ".authorize('
reject 'callee-shaped string'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '            authorize!('
reject 'callee-shaped macro'
printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,3 +20,4 @@' '             .authorize(' '             .unrelated_call(' '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '             holder,' >"$TEMP_ROOT/change.diff"
reject 'governed call elsewhere in hunk'

for changed_import in \
  '-use attacker_controlled::{AuthorityCertificate, CertificatePurpose};|+use attacker_controlled::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{CertificatePurpose, AuthorityCertificate, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use other::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate as AC, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS, AUTHORITY_BOUND_CERTIFICATE_ACCESS};' \
  '-use super::certificates::{AuthorityCertificate, CertificatePurpose};|+use super::certificates::{AuthorityCertificate, AUTHORITY_BOUND_CERTIFICATE_ACCESS};'; do
  old="${changed_import%%|*}"; new="${changed_import#*|}"
  printf '%s\n' 'diff --git a/adl-runtime/src/distributed/transport/core.rs b/adl-runtime/src/distributed/transport/core.rs' '--- a/adl-runtime/src/distributed/transport/core.rs' '+++ b/adl-runtime/src/distributed/transport/core.rs' '@@ -20,1 +20,1 @@' "$old" "$new" >"$TEMP_ROOT/change.diff"
  reject "semantic import rewrite"
done

write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
sed 's#--- a/adl-runtime/src/distributed/transport/core.rs#--- a/wrong.rs#' "$TEMP_ROOT/change.diff" >"$TEMP_ROOT/change.next" && mv "$TEMP_ROOT/change.next" "$TEMP_ROOT/change.diff"
reject 'mismatched old header'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
sed 's/@@ -20,1 +20,2 @@/@@ -20,2 +20,2 @@/' "$TEMP_ROOT/change.diff" >"$TEMP_ROOT/change.next" && mv "$TEMP_ROOT/change.next" "$TEMP_ROOT/change.diff"
reject 'incorrect hunk count'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
printf '%s\n' 'trailing junk' >>"$TEMP_ROOT/change.diff"
reject 'trailing junk'
write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
printf '%s\n' '\ No newline at end of file' >>"$TEMP_ROOT/change.diff"
reject 'unsupported hunk marker'

write_diff '+                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,'
write_mapping /usr/bin/false
reject 'governed compile command failure'
write_mapping
jq '.mappings[0].behavior_commands.EstablishedRuntimeAuthority=["/usr/bin/false"]' "$MAPPING" >"$TEMP_ROOT/mapping.next" && mv "$TEMP_ROOT/mapping.next" "$MAPPING"
reject 'governed owner behavior command failure'
write_mapping
sed 's#adl-runtime/src/distributed/transport/core.rs#adl-runtime/src/distributed/transport/unmapped.rs#g' "$TEMP_ROOT/change.diff" >"$TEMP_ROOT/change.next" && mv "$TEMP_ROOT/change.next" "$TEMP_ROOT/change.diff"
reject 'unmapped file'

# End-to-end gate uses a governed fixture mapping and proves the 80% threshold
# is bypassed only after the classifier itself executes every required command.
GATE_ROOT="$TEMP_ROOT/gate-repo"
mkdir -p "$GATE_ROOT/adl/tools" "$GATE_ROOT/adl/config" "$GATE_ROOT/adl-runtime/src/distributed/transport"
cp "$ROOT/adl/tools/check_coverage_impact.sh" "$CLASSIFIER" "$GATE_ROOT/adl/tools/"
write_mapping
cp "$MAPPING" "$GATE_ROOT/adl/config/mechanical_coverage_fallout.v1.json"
printf '%s\n' '#!/usr/bin/env bash' 'exit 0' >"$GATE_ROOT/proof-command.sh"
chmod +x "$GATE_ROOT/proof-command.sh"
jq '.mappings[0].compile_command=["bash","proof-command.sh"] | .mappings[0].behavior_commands.EstablishedRuntimeAuthority=["bash","proof-command.sh"] | .mappings[0].behavior_commands.TransportAuthorization=["bash","proof-command.sh"]' "$GATE_ROOT/adl/config/mechanical_coverage_fallout.v1.json" >"$GATE_ROOT/adl/config/mechanical.next" && mv "$GATE_ROOT/adl/config/mechanical.next" "$GATE_ROOT/adl/config/mechanical_coverage_fallout.v1.json"
git -C "$GATE_ROOT" init -q
git -C "$GATE_ROOT" config user.name fixture
git -C "$GATE_ROOT" config user.email fixture@example.invalid
{ printf '%s\n' 'use super::certificates::{AuthorityCertificate, CertificatePurpose};'; for _ in $(seq 1 20); do echo '// context'; done; printf '%s\n' 'authorize(' '    holder,' ');'; } >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
git -C "$GATE_ROOT" add . && git -C "$GATE_ROOT" commit -qm baseline
# Mutable unrelated proof input must not influence execution; the gate archives
# the exact base and overlays only the classified source diff.
printf '%s\n' '#!/usr/bin/env bash' 'exit 1' >"$GATE_ROOT/proof-command.sh"
printf '%s\n' '#!/usr/bin/env python3' 'raise SystemExit(1)' >"$GATE_ROOT/adl/tools/mechanical_coverage_fallout.py"
jq '.mappings[0].compile_command=["/usr/bin/false"]' "$GATE_ROOT/adl/config/mechanical_coverage_fallout.v1.json" >"$GATE_ROOT/adl/config/mechanical.next" && mv "$GATE_ROOT/adl/config/mechanical.next" "$GATE_ROOT/adl/config/mechanical_coverage_fallout.v1.json"
{ printf '%s\n' 'use super::certificates::{AuthorityCertificate, CertificatePurpose, AUTHORITY_BOUND_CERTIFICATE_ACCESS};'; for _ in $(seq 1 20); do echo '// context'; done; printf '%s\n' 'authorize(' '    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,' '    holder,' ');'; } >"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
printf '%s\n' '{"data":[{"files":[{"filename":"adl-runtime/src/distributed/transport/core.rs","summary":{"lines":{"covered":1,"count":10}}}]}]}' >"$TEMP_ROOT/summary.json"
(cd "$GATE_ROOT" && bash adl/tools/check_coverage_impact.sh --base HEAD --include-working-tree --summary "$TEMP_ROOT/summary.json" --mechanical-receipt-dir "$TEMP_ROOT/receipts") >/dev/null
jq -e '.execution_provenance == "classifier_executed_governed_commands" and (.hunks | length) == 2' "$TEMP_ROOT/receipts"/mechanical-*.json >/dev/null

# A rejected rerun in the same directory must remove the earlier exact-diff
# receipt and its per-path result artifacts instead of leaving stale proof.
printf '%s\n' 'semantic_change();' >>"$GATE_ROOT/adl-runtime/src/distributed/transport/core.rs"
if (cd "$GATE_ROOT" && bash adl/tools/check_coverage_impact.sh --base HEAD --include-working-tree --summary "$TEMP_ROOT/summary.json" --mechanical-receipt-dir "$TEMP_ROOT/receipts") >/dev/null 2>&1; then
  echo 'expected semantic rerun to fail the coverage gate' >&2
  exit 1
fi
if compgen -G "$TEMP_ROOT/receipts/mechanical-*.json" >/dev/null; then
  echo 'rejected rerun retained stale mechanical receipt' >&2
  exit 1
fi
if find "$TEMP_ROOT/receipts/results" -type f -print -quit 2>/dev/null | grep -q .; then
  echo 'rejected rerun retained stale mechanical result artifacts' >&2
  exit 1
fi

# A revision disappearing after changed-file discovery must use the same
# fail-closed cleanup path. Force the helper's base rev-parse to fail after the
# top-level repository discovery and changed-file diff have already succeeded.
REAL_GIT="$(command -v git)"
mkdir -p "$TEMP_ROOT/bin" "$TEMP_ROOT/gate-tmp"
printf '%s\n' '#!/usr/bin/env bash' \
  'if [ "$1" = "-C" ] && [ "$3" = "rev-parse" ] && [ "$4" = "HEAD" ]; then exit 1; fi' \
  'exec "'"$REAL_GIT"'" "$@"' >"$TEMP_ROOT/bin/git"
chmod +x "$TEMP_ROOT/bin/git"
if (cd "$GATE_ROOT" && PATH="$TEMP_ROOT/bin:$PATH" TMPDIR="$TEMP_ROOT/gate-tmp" bash adl/tools/check_coverage_impact.sh --base HEAD --include-working-tree --summary "$TEMP_ROOT/summary.json" --mechanical-receipt-dir "$TEMP_ROOT/receipts") >/dev/null 2>&1; then
  echo 'expected stale base revision to fail the coverage gate' >&2
  exit 1
fi
if find "$TEMP_ROOT/gate-tmp" -mindepth 1 -print -quit | grep -q .; then
  echo 'revision-resolution failure retained temporary proof artifacts' >&2
  exit 1
fi
if compgen -G "$TEMP_ROOT/receipts/mechanical-*.json" >/dev/null || find "$TEMP_ROOT/receipts/results" -type f -print -quit 2>/dev/null | grep -q .; then
  echo 'revision-resolution failure retained stale receipt or results' >&2
  exit 1
fi

echo "PASS: mechanical compile-fallout classifier"

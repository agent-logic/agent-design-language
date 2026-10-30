#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
ADL_REVIEW_BIN="${ADL_REVIEW_BIN:-}"
ADL_PACKAGE_VERSION="${ADL_PACKAGE_VERSION:-}"

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

assert_not_contains() {
  local pattern="$1" text="$2" label="$3"
  if grep -Fq "$pattern" <<<"$text"; then
    echo "assertion failed ($label): did not expect to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  fi
}

assert_status_nonzero() {
  local status="$1" label="$2"
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed ($label): expected nonzero exit" >&2
    exit 1
  }
}

run_review() {
  if [[ -n "$ADL_REVIEW_BIN" ]]; then
    "$ADL_REVIEW_BIN" "$@"
    return
  fi
  cargo run --quiet --manifest-path "$MANIFEST" --bin adl-review -- "$@"
}

package_version() {
  if [[ -n "$ADL_PACKAGE_VERSION" ]]; then
    printf '%s\n' "$ADL_PACKAGE_VERSION"
    return
  fi
  cargo metadata --quiet --no-deps --format-version 1 --manifest-path "$MANIFEST" | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])'
}

help_output="$(run_review --help)"
assert_contains "adl-review - ADL review tooling compatibility binary" "$help_output" "review help title"
assert_contains "adl-review code-review --out <dir>" "$help_output" "review code-review help"
assert_contains "adl-review verify-repo-contract --review <review.md>" "$help_output" "review contract help"
assert_not_contains "adl-review card-surface" "$help_output" "stale card-surface help"
assert_not_contains "adl-review runtime-surface" "$help_output" "stale runtime-surface help"
assert_not_contains "adl-review verify-output-provenance" "$help_output" "stale provenance help"
assert_contains "C-SDLC issue work resolves through csdlc-install and the independent typed v2 binaries" "$help_output" "csdlc handoff help"

version_output="$(run_review --version)"
expected_version="$(package_version)"
assert_contains "$expected_version" "$version_output" "review version"

review_fixture="$TMP_DIR/review.md"
cat >"$review_fixture" <<'EOF'
# Repository Review

## Metadata

- Review Type: fixture
- Subject: adl-review compatibility
- Reviewer: fixture

## Scope

- Reviewed: review compatibility surface
- Not Reviewed: runtime behavior
- Review Mode: fixture
- Gate: non-blocking

## Findings

No material findings.

## System-Level Assessment

The review packet is structurally valid for compatibility smoke coverage.

## Recommended Action Plan

- Fix now: none
- Fix before milestone closeout: none
- Defer: none

## Follow-ups / Deferred Work

None.

## Final Assessment

Pass.
EOF

review_out="$TMP_DIR/review-contract.txt"
run_review verify-repo-contract --review "$review_fixture" >"$review_out"
assert_contains "repo-review-contract: ok" "$(cat "$review_out")" "review contract success"

bad_review="$TMP_DIR/bad-review.md"
cat >"$bad_review" <<'EOF'
## Metadata
- Review Type: fixture
- Reviewer: fixture

## Scope
- Reviewed: review compatibility surface
- Not Reviewed: runtime behavior
- Review Mode: fixture

## Findings
- vague issue with no severity

## Final Assessment
Looks okay.
EOF

set +e
bad_output="$(run_review verify-repo-contract --review "$bad_review" 2>&1)"
bad_status=$?
set -e
assert_status_nonzero "$bad_status" "bad review contract"
assert_contains "repo review contract violation" "$bad_output" "bad review contract diagnostic"

fenced_review="$TMP_DIR/fenced-review.md"
cat >"$fenced_review" <<'EOF'
This is not a review packet; the required markers below are example text only.

```markdown
## Metadata
- Review Type: fixture
- Reviewer: fixture

## Scope
- Reviewed: review compatibility surface
- Not Reviewed: runtime behavior
- Review Mode: fixture

## Findings
No material findings.

## System-Level Assessment
Example only.

## Recommended Action Plan
Example only.

## Follow-ups / Deferred Work
Example only.

## Final Assessment
Example only.
```
EOF

set +e
fenced_output="$(run_review verify-repo-contract --review "$fenced_review" 2>&1)"
fenced_status=$?
set -e
assert_status_nonzero "$fenced_status" "fenced review contract"
assert_contains "repo review contract violation" "$fenced_output" "fenced review contract diagnostic"

long_fenced_review="$TMP_DIR/long-fenced-review.md"
cat >"$long_fenced_review" <<'EOF'
This is not a review packet; the required markers below are example text only.

````markdown
The shorter fence below is literal text inside the four-backtick block.

```markdown
## Metadata
- Review Type: fixture
- Reviewer: fixture

## Scope
- Reviewed: review compatibility surface
- Not Reviewed: runtime behavior
- Review Mode: fixture

## Findings
No material findings.

## System-Level Assessment
Example only.

## Recommended Action Plan
Example only.

## Follow-ups / Deferred Work
Example only.

## Final Assessment
Example only.
```
````
EOF

set +e
long_fenced_output="$(run_review verify-repo-contract --review "$long_fenced_review" 2>&1)"
long_fenced_status=$?
set -e
assert_status_nonzero "$long_fenced_status" "long fenced review contract"
assert_contains "repo review contract violation" "$long_fenced_output" "long fenced review contract diagnostic"

code_review_out="$TMP_DIR/code-review-smoke"
code_review_output="$(run_review code-review --out "$code_review_out" --backend fixture --visibility read-only-repo)"
assert_contains "code-review fixture: ok" "$code_review_output" "code-review fixture"
python3 "$ROOT_DIR/adl/tools/validate_codebuddy_review_showcase_demo.py" "$code_review_out"

for stale_command in card-surface runtime-surface verify-output-provenance; do
  set +e
  stale_output="$(run_review "$stale_command" --help 2>&1)"
  stale_status=$?
  set -e
  assert_status_nonzero "$stale_status" "stale hidden review command: $stale_command"
  assert_contains "not implemented in this compatibility binary" "$stale_output" "stale hidden command diagnostic: $stale_command"
  assert_not_contains "v1 tooling" "$stale_output" "stale hidden command avoids v1 wording: $stale_command"
  assert_not_contains "multiplexer" "$stale_output" "stale hidden command avoids multiplexer wording: $stale_command"
done

set +e
issue_output="$(run_review pr run 3599 2>&1)"
issue_status=$?
set -e
assert_status_nonzero "$issue_status" "review pr ownership"
assert_contains "review tooling only" "$issue_output" "review issue handoff"

set +e
runtime_output="$(run_review run workflow.adl.yaml 2>&1)"
runtime_status=$?
set -e
assert_status_nonzero "$runtime_status" "review runtime ownership"
assert_contains "does not run ADL runtime commands" "$runtime_output" "review runtime rejection"

echo "PASS test_adl_review_compatibility"

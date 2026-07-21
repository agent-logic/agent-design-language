#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
repo_root="$(git rev-parse --show-toplevel)"
manifest="${repo_root}/adl-v2/crates/adl-language/Cargo.toml"
crate_root="${repo_root}/adl-v2/crates/adl-language"

if [[ ! -f "${manifest}" ]]; then
  echo "BLOCKED: #5339 language manifest does not exist; implementation remains dependency-gated" >&2
  exit 20
fi

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/Volumes/FastWork/adl-wp-5339-target}"

case "${mode}" in
  focused)
    exec cargo test --manifest-path "${manifest}" --all-targets
    ;;
  quality)
    exec cargo clippy --manifest-path "${manifest}" --all-targets -- -D warnings
    ;;
  parity)
    exec cargo test --manifest-path "${manifest}" --test characterization_parity
    ;;
  budgets)
    implementation_lines="$({ find "${crate_root}/src" -type f -name '*.rs' -print0 2>/dev/null | sort -z | xargs -0 wc -l; } | awk 'END {print $1 + 0}')"
    test_lines="$({ find "${crate_root}/tests" "${crate_root}/fixtures" -type f -print0 2>/dev/null | sort -z | xargs -0 wc -l; } | awk 'END {print $1 + 0}')"
    if (( implementation_lines > 4000 )); then
      echo "implementation LoC ${implementation_lines} exceeds 4000" >&2
      exit 21
    fi
    if (( test_lines > 4000 )); then
      echo "test/fixture LoC ${test_lines} exceeds 4000" >&2
      exit 22
    fi
    direct_dependencies="$(cargo metadata --manifest-path "${manifest}" --no-deps --format-version 1 | jq -r '.packages[0].dependencies[].name' | sort -u)"
    reviewed_dependencies="$(printf '%s\n' jsonschema schemars serde serde_json yaml_serde | sort)"
    if [[ "${direct_dependencies}" != "${reviewed_dependencies}" ]]; then
      echo "direct dependencies do not exactly match the reviewed COTS set" >&2
      echo "observed:" >&2
      printf '%s\n' "${direct_dependencies}" >&2
      echo "required:" >&2
      printf '%s\n' "${reviewed_dependencies}" >&2
      exit 23
    fi
    dependencies="$(cargo tree --manifest-path "${manifest}" --edges normal,build,dev --prefix none | tail -n +2)"
    if grep -Eiq '(^|[-_])(adl|runtime|csdlc|tokio|async-std|reqwest|hyper|aws|sqlx|diesel)([-_]|$)' <<<"${dependencies}"; then
      echo "forbidden dependency family detected" >&2
      printf '%s\n' "${dependencies}" >&2
      exit 24
    fi
    start="$(date +%s)"
    cargo test --manifest-path "${manifest}" --all-targets
    elapsed="$(( $(date +%s) - start ))"
    if (( elapsed > 600 )); then
      echo "deterministic validation ${elapsed}s exceeds 600s" >&2
      exit 25
    fi
    printf '{"implementation_lines":%s,"test_fixture_lines":%s,"full_validation_seconds":%s}\n' "${implementation_lines}" "${test_lines}" "${elapsed}"
    ;;
  *)
    echo "usage: $0 focused|quality|parity|budgets" >&2
    exit 64
    ;;
esac

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
candidate="${ADL_CARGO_BUILD_ROOT:-}"

if [[ -z "$candidate" ]]; then
  candidate="${ADL_FASTWORK_ROOT:-/Volumes/FastWork}"
fi

if [[ ! -d "$candidate" || ! -w "$candidate" ]]; then
  echo "Cargo validation requires a writable external build root: $candidate" >&2
  exit 2
fi

build_root="$(cd "$candidate" && pwd -P)"
case "$build_root/" in
  "$ROOT_DIR/"|"$ROOT_DIR/"*)
    echo "Cargo validation build root must be outside the repository: $build_root" >&2
    exit 2
    ;;
esac

export CARGO_HOME="$build_root/cargo-home"
export CARGO_TARGET_DIR="$build_root/cargo-target"
mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR"

# Gate 10A installer proofs resolve binaries through the crate-local target
# path. Keep that compatibility name as a symlink while all build bytes remain
# on the declared external target.
manifest_path=""
previous=""
for argument in "$@"; do
  if [[ "$previous" == "--manifest-path" ]]; then
    manifest_path="$argument"
    break
  fi
  previous="$argument"
done
if [[ -n "$manifest_path" ]]; then
  manifest_dir="$(cd "$(dirname "$manifest_path")" && pwd -P)"
  if [[ "$manifest_dir" == "$ROOT_DIR/csdlc-v2" ]]; then
    compatibility_target="$manifest_dir/target"
    if [[ -L "$compatibility_target" ]]; then
      linked_target="$(cd "$compatibility_target" && pwd -P)"
      if [[ "$linked_target" != "$CARGO_TARGET_DIR" ]]; then
        echo "C-SDLC target compatibility link points outside the selected build root" >&2
        exit 2
      fi
    elif [[ -e "$compatibility_target" ]]; then
      echo "C-SDLC target compatibility path must not contain local build output" >&2
      exit 2
    else
      ln -s "$CARGO_TARGET_DIR" "$compatibility_target"
    fi
  fi
fi

if [[ $# -eq 0 ]]; then
  echo "usage: run_cargo_validation.sh <command> [args...]" >&2
  exit 2
fi

exec "$@"

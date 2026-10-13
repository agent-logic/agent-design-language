#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# PVF: deterministic local public policy regression, small YAML fixtures; no builds/network.
# Original private-job/nextest/backend assertions are retained in #1223 custody;
# they are explicitly retired, not counted as passing this public contract.

ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" >/dev/null
ruby "$ROOT_DIR/adl/tools/test_workspace_coverage_deadline.rb"

mkdir -p "$ROOT_DIR/.adl/tmp"
POLICY_FIXTURE_ROOT="$(mktemp -d "$ROOT_DIR/.adl/tmp/ci-policy.XXXXXX")"
trap 'rm -rf "$POLICY_FIXTURE_ROOT"' EXIT
mkdir -p "$POLICY_FIXTURE_ROOT/.github" "$POLICY_FIXTURE_ROOT/adl/tools" "$POLICY_FIXTURE_ROOT/tools/public_adl"
cp -R "$ROOT_DIR/.github/workflows" "$POLICY_FIXTURE_ROOT/.github/workflows"
cp "$ROOT_DIR/adl/tools/ci_path_policy.sh" "$POLICY_FIXTURE_ROOT/adl/tools/ci_path_policy.sh"
cp "$ROOT_DIR/tools/public_adl/ci_policy.py" "$POLICY_FIXTURE_ROOT/tools/public_adl/ci_policy.py"
cp "$ROOT_DIR/rust-toolchain.toml" "$POLICY_FIXTURE_ROOT/rust-toolchain.toml"
ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null
ruby -e 'path = ARGV.fetch(0); text = File.read(path); File.write(path, text.sub("on:\n", "on:\n  workflow_run:\n"))' "$POLICY_FIXTURE_ROOT/.github/workflows/wp08-native-birthday.yml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "workflow_run fixture escaped automatic-fanout enforcement" >&2
  exit 1
fi
cp -R "$ROOT_DIR/.github/workflows/." "$POLICY_FIXTURE_ROOT/.github/workflows/"
ruby -e 'path = ARGV.fetch(0); text = File.read(path); old = %q{toolchain: 1.92.0}; new = %q{toolchain: stable}; abort "pinned rust fixture source missing" unless text.include?(old); File.write(path, text.sub(old, new))' "$POLICY_FIXTURE_ROOT/.github/workflows/ci.yaml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "floating Rust toolchain fixture escaped required-CI enforcement" >&2
  exit 1
fi
cp -R "$ROOT_DIR/.github/workflows/." "$POLICY_FIXTURE_ROOT/.github/workflows/"
ruby -e 'path = ARGV.fetch(0); text = File.read(path); old = "          toolchain: 1.92.0\n"; abort "pinned rust fixture source missing" unless text.include?(old); File.write(path, text.sub(old, ""))' "$POLICY_FIXTURE_ROOT/.github/workflows/ci.yaml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "missing Rust toolchain fixture escaped required-CI enforcement" >&2
  exit 1
fi
cp -R "$ROOT_DIR/.github/workflows/." "$POLICY_FIXTURE_ROOT/.github/workflows/"
ruby -e 'path = ARGV.fetch(0); text = File.read(path); old = %q{runs-on: ubuntu-latest}; new = %q{runs-on: ${{ vars.ADL_HEAVY_RUNNER || '\''adl-ubuntu-24.04-16core'\'' }}}; abort "standard runner fixture source missing" unless text.include?(old); File.write(path, text.sub(old, new))' "$POLICY_FIXTURE_ROOT/.github/workflows/ci.yaml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "runner-bypass fixture escaped standard-runner enforcement" >&2
  exit 1
fi
cp -R "$ROOT_DIR/.github/workflows/." "$POLICY_FIXTURE_ROOT/.github/workflows/"
ruby -e 'path = ARGV.fetch(0); text = File.read(path); marker = "  public_adl_validation:\n"; abort "public ADL job missing" unless text.include?(marker); head, tail = text.split(marker, 2); tail = tail.sub("runs-on: macos-latest", "runs-on: ubuntu-latest"); abort "public ADL macOS runner missing" unless tail.include?("runs-on: ubuntu-latest"); File.write(path, head + marker + tail)' "$POLICY_FIXTURE_ROOT/.github/workflows/ci.yaml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "public ADL Ubuntu fixture escaped sandbox-capable runner enforcement" >&2
  exit 1
fi
cp -R "$ROOT_DIR/.github/workflows/." "$POLICY_FIXTURE_ROOT/.github/workflows/"
ruby -e 'path = ARGV.fetch(0); text = File.read(path); marker = "  public_adl_validation:\n"; abort "public ADL job missing" unless text.include?(marker); head, tail = text.split(marker, 2); tail = tail.sub("runs-on: macos-latest", "runs-on: self-hosted"); abort "public ADL macOS runner missing" unless tail.include?("runs-on: self-hosted"); File.write(path, head + marker + tail)' "$POLICY_FIXTURE_ROOT/.github/workflows/ci.yaml"
if ruby "$ROOT_DIR/adl/tools/validate_ci_workflow_policy.rb" "$POLICY_FIXTURE_ROOT" >/dev/null 2>&1; then
  echo "public ADL self-hosted fixture escaped sandbox-capable runner enforcement" >&2
  exit 1
fi

echo "PASS public CI runtime policy: six mutation refusals plus valid fixture; seven deadline cases reported separately"

ruby -ryaml - "$ROOT_DIR/.github/workflows/ci.yaml" <<'RUBY'
workflow = YAML.safe_load(File.read(ARGV.fetch(0)), permitted_classes: [], permitted_symbols: [], aliases: true)
jobs = workflow.fetch("jobs")
%w[adl-ci adl-coverage].each do |name|
  abort "CI must expose exactly one stable required #{name} status" unless jobs.values.count { |job| job["name"] == name } == 1
end
%w[public_adl_validation public_adl_coverage].each do |name|
  runs = jobs.fetch(name).fetch("steps").map { |step| step["run"] }.compact.join("\n")
  abort "public hosted producers must not invoke AWS execution" if runs.match?(/(?:^|\s)aws(?:\s|$)/i)
end
RUBY

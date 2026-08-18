#!/usr/bin/env bash
set -euo pipefail

issue_dir=".csdlc/evidence/287"
manifest="$issue_dir/evidence-manifest.json"
observations="$issue_dir/live-observations.json"
report="$issue_dir/adr0071-provider-neutral-multi-agent-reconciliation.md"
git_common_dir="$(git rev-parse --git-common-dir)"
terminal_cache_logical=".git/csdlc-v2/derived-terminal/341.json"
terminal_cache_resolved="$git_common_dir/csdlc-v2/derived-terminal/341.json"
supporting_terminal_issues=(283 284 285 286)

for required in "$manifest" "$observations" "$report"; do
  if [[ ! -s "$required" ]]; then
    echo "missing required ADR 0071 evidence artifact: $required" >&2
    exit 1
  fi
done

python3 - "$manifest" "$observations" "$report" "$terminal_cache_logical" "$terminal_cache_resolved" "$git_common_dir" "${supporting_terminal_issues[@]}" <<'PY'
import json
import pathlib
import re
import sys

manifest_path = pathlib.Path(sys.argv[1])
observations_path = pathlib.Path(sys.argv[2])
report_path = pathlib.Path(sys.argv[3])
terminal_cache_logical = sys.argv[4]
terminal_cache_resolved = pathlib.Path(sys.argv[5])
git_common_dir = pathlib.Path(sys.argv[6])
supporting_issues = [int(value) for value in sys.argv[7:]]

manifest = json.loads(manifest_path.read_text())
observations = json.loads(observations_path.read_text())
report = report_path.read_text()

errors = []

def expect(condition, message):
    if not condition:
        errors.append(message)

expect(manifest.get("schema") == "adl.csdlc.evidence.adr0071_provider_neutral_multi_agent_manifest.v1", "manifest schema mismatch")
expect(manifest.get("issue") == 287, "manifest issue must be 287")
expect(manifest.get("parent_issue") == 207, "manifest parent_issue must be 207")
expect(manifest.get("adr") == "ADR 0071", "manifest ADR must be ADR 0071")
expect(manifest.get("dependency_umbrella", {}).get("issue") == 341, "dependency umbrella must be #341")
expect(manifest.get("dependency_umbrella", {}).get("state") == "OPEN", "dependency umbrella state must be OPEN while #341 is non-terminal")
expect(manifest.get("dependency_umbrella", {}).get("terminal_cache_present") is False, "manifest must record missing #341 terminal cache")
expect(manifest.get("provider_neutral_multi_agent_proof", {}).get("terminal") is False, "terminal proof must be false")
expect(manifest.get("provider_neutral_multi_agent_proof", {}).get("classification") == "residual_gap", "proof classification must be residual_gap")

non_claims = set(manifest.get("non_claims", []))
for required in [
    "ADR 0071 acceptance",
    "#207 closeout",
    "#288 final ADR serialization",
    "provider execution",
    "credential access",
    "WP-18B terminal provider-neutral proof",
]:
    expect(required in non_claims, f"missing non-claim: {required}")

residual_gaps = manifest.get("residual_gaps", [])
expect(any("#341" in gap and "open" in gap.lower() for gap in residual_gaps), "residual gaps must mention open #341")

expect(observations.get("issue_341", {}).get("state") == "OPEN", "live observations must record issue #341 OPEN")
expect(observations.get("derived_terminal", {}).get("341_present") is False, "live observations must record missing #341 terminal cache")
expect(not terminal_cache_resolved.exists(), f"actual #341 terminal cache exists; residual-gap packet must be refreshed: {terminal_cache_resolved}")
expect(observations.get("derived_terminal", {}).get("341_path") == terminal_cache_logical, "live observations must record the checked #341 logical terminal cache path")
expect(observations.get("derived_terminal", {}).get("341_resolved_path") == str(terminal_cache_resolved), "live observations must record the checked #341 resolved terminal cache path")
expect(manifest.get("dependency_umbrella", {}).get("terminal_cache_path") == terminal_cache_logical, "manifest must record the checked #341 logical terminal cache path")
expect(observations.get("credentials_read") is False, "observations must record credentials_read false")
expect(observations.get("provider_execution_run") is False, "observations must record provider_execution_run false")
expect(observations.get("shared_adr_docs_updated") is False, "observations must record shared_adr_docs_updated false")

supporting = manifest.get("supporting_terminal_caches", [])
supporting_by_issue = {entry.get("issue"): entry for entry in supporting}
for issue in supporting_issues:
    entry = supporting_by_issue.get(issue)
    cache_path = git_common_dir / "csdlc-v2" / "derived-terminal" / f"{issue}.json"
    expect(entry is not None, f"missing supporting cache entry for #{issue}")
    expect(cache_path.exists(), f"supporting terminal cache missing on disk for #{issue}: {cache_path}")
    if cache_path.exists():
        cache = json.loads(cache_path.read_text())
        expect(cache.get("issue") == issue, f"supporting cache issue mismatch for #{issue}")
        expect(cache.get("disposition") == "merged", f"supporting cache disposition must be merged for #{issue}")
        expect(bool(cache.get("merge_sha")), f"supporting cache merge_sha missing for #{issue}")
        expect(bool(cache.get("head_sha")), f"supporting cache head_sha missing for #{issue}")
        expect(entry.get("path") == f".git/csdlc-v2/derived-terminal/{issue}.json", f"supporting manifest path mismatch for #{issue}")
        expect(entry.get("disposition") == cache.get("disposition"), f"supporting manifest disposition mismatch for #{issue}")
        expect(entry.get("merge_sha") == cache.get("merge_sha"), f"supporting manifest merge_sha mismatch for #{issue}")
        expect(entry.get("head_sha") == cache.get("head_sha"), f"supporting manifest head_sha mismatch for #{issue}")
        expect(entry.get("digest") == cache.get("digest"), f"supporting manifest digest mismatch for #{issue}")
        expect(entry.get("canonical_digest") == cache.get("canonical_digest"), f"supporting manifest canonical_digest mismatch for #{issue}")
        expect(entry.get("classification") == "supporting_only", f"supporting manifest classification must be supporting_only for #{issue}")

for phrase in [
    "Residual gap",
    "#341 remains open",
    "does not accept ADR 0071",
    "does not execute providers",
    "does not update shared ADR docs",
]:
    expect(phrase in report, f"report missing phrase: {phrase}")

for forbidden in [
    r"adr\s*0071.{0,80}\b(accepted|acceptance|approved|final)\b",
    r"\bwp-18b\b.{0,80}\b(terminal|complete|completed|accepted|proven)\b",
    r"\bprovider[- ]neutral\b.{0,80}\b(terminal|complete|completed|accepted|proven)\b",
    r"#341.{0,80}\b(closed|complete|completed|terminal|accepted|merged)\b",
    r"\bprovider execution\b.{0,80}\b(run|ran|complete|completed|succeeded|passed)\b",
    r"\bcredentials?(_read| read| were read| accessed| loaded| used)\b",
    r"\bshared adr docs\b.{0,80}\b(updated|changed|modified|serialized)\b",
    r"\baccepted\b.{0,80}\badr\s*0071\b",
    r"\bterminal\b.{0,80}\bwp-18b\b",
    r"credentials_read\s*[:=]\s*true",
    r"provider_execution_run\s*[:=]\s*true",
]:
    expect(re.search(forbidden, report, flags=re.IGNORECASE) is None, f"report contains forbidden contradictory pattern: {forbidden}")

if errors:
    for error in errors:
        print(f"ADR0071 evidence validation failed: {error}", file=sys.stderr)
    sys.exit(1)

print("ADR0071 provider-neutral multi-agent evidence packet PASS")
PY

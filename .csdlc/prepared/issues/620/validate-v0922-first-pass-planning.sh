#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
root="$(git rev-parse --show-toplevel)"
cd "$root"

required=(
  docs/milestones/v0.92.2/README.md
  docs/milestones/v0.92.2/VISION_v0.92.2.md
  docs/milestones/v0.92.2/DESIGN_v0.92.2.md
  docs/milestones/v0.92.2/DECISIONS_v0.92.2.md
  docs/milestones/v0.92.2/WBS_v0.92.2.md
  docs/milestones/v0.92.2/SPRINT_v0.92.2.md
  docs/milestones/v0.92.2/PLANNED_ISSUE_CATALOG_v0.92.2.md
  docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml
  docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
  docs/milestones/v0.92.2/CANONICAL_DOC_INVENTORY_v0.92.2.md
  docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md
  docs/milestones/v0.92.2/TBD_SOURCE_AUDIT_MANIFEST_v0.92.2.txt
  docs/milestones/v0.92.2/WP_EXECUTION_READINESS_v0.92.2.md
  docs/milestones/v0.92.2/FEATURE_PROOF_COVERAGE_v0.92.2.md
  docs/milestones/v0.92.2/QUALITY_GATE_v0.92.2.md
  docs/milestones/v0.92.2/DEMO_MATRIX_v0.92.2.md
  docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md
  docs/milestones/v0.92.2/ADR_PLAN_v0.92.2.md
  docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md
  docs/milestones/v0.92.2/RELEASE_NOTES_v0.92.2.md
  docs/milestones/v0.92.2/NEXT_MILESTONE_HANDOFF_v0.92.2.md
  docs/milestones/v0.92.2/features/README.md
  docs/milestones/v0.92.2/features/PRODUCT_SHELL_AND_OPERATOR_CONTROLS_v0.92.2.md
  docs/milestones/v0.92.2/features/PORTABLE_ADAPTER_V2_v0.92.2.md
  docs/milestones/v0.92.2/features/EVIDENCE_CORE_v0.92.2.md
  docs/milestones/v0.92.2/features/ARCHITECTURE_COGNITION_v0.92.2.md
  docs/milestones/v0.92.2/features/EXECUTABLE_GOVERNANCE_v0.92.2.md
  docs/milestones/v0.92.2/features/MULTI_PERSPECTIVE_REVIEW_v0.92.2.md
  docs/milestones/v0.92.2/features/LONGITUDINAL_REVIEW_MEMORY_v0.92.2.md
  docs/milestones/v0.92.2/features/GOVERNED_PUBLICATION_v0.92.2.md
  docs/milestones/v0.92.2/features/BETA1_QUALIFICATION_v0.92.2.md
  docs/milestones/v0.92.2/features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md
)

structure() {
  for path in "${required[@]}"; do test -f "$path" || { echo "missing: $path" >&2; return 1; }; done
  ruby -e 'require "yaml"; ARGV.each { |p| YAML.safe_load(File.read(p), aliases: true) }' \
    docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml \
    docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
  ruby -e '
    require "yaml"
    wave = YAML.safe_load(File.read(ARGV[0]), aliases: true)
    specs = YAML.safe_load(File.read(ARGV[1]), aliases: true)
    wave_ids = wave.fetch("work_packages").map { |row| row.fetch("id") }
    spec_ids = specs.fetch("specifications").map { |row| row.fetch("id") }
    abort "duplicate work-package id" unless wave_ids.uniq == wave_ids
    abort "wave/spec identity or order mismatch" unless wave_ids == spec_ids
    abort "conductor must remain number-free" unless wave["conductor_issue"].nil?
    abort "planned issue number assigned" unless wave.fetch("work_packages").all? { |row| row["issue"].nil? }
    tail = (1..10).map { |n| format("TAIL-%02d", n) }
    abort "release tail mismatch" unless wave.fetch("canonical_release_tail") == tail
    abort "spec release tail mismatch" unless specs.fetch("release_tail").fetch("order") == tail
    known = wave_ids.to_h { |id| [id, true] }
    wave.fetch("work_packages").each do |row|
      row.fetch("depends_on", []).each { |dep| abort "unknown dependency #{dep}" unless known[dep] }
    end
    provider = wave.fetch("work_packages").find { |row| row.fetch("id") == "PLAT-PROVIDER" }
    abort "provider external dependency mismatch" unless provider.fetch("external_dependencies", []) == ["v0.92.1-issue-622"]
    mlx = wave.fetch("work_packages").find { |row| row.fetch("id") == "PLAT-MLX" }
    abort "MLX dependency mismatch" unless mlx.fetch("depends_on") == ["PLAT-PROVIDER"]
    wave.fetch("work_packages").select { |row| row.fetch("id").start_with?("TAIL-") }.each_with_index do |row, index|
      expected = index.zero? ? ["CF-INTEGRATE"] : [tail[index - 1]]
      abort "release-tail edge mismatch at #{row.fetch("id")}" unless row.fetch("depends_on") == expected
    end
    catalog = File.read(ARGV[2])
    catalog_ids = catalog.lines.map { |line| line[/^\|\s*\d+\s*\|\s*([A-Z][A-Z0-9-]+)\s*\|/, 1] }.compact
    abort "catalog identity or order mismatch" unless catalog_ids == wave_ids
    issue_refs = catalog.scan(/#\d+/).uniq
    abort "unexpected concrete issue number in catalog: #{issue_refs.join(", ")}" unless issue_refs == ["#622", "#484"]
    ops = wave.fetch("work_packages").find { |row| row.fetch("id") == "OPS-AWS" }
    abort "AWS #484 baseline mismatch" unless ops.fetch("external_dependencies", []) == ["completed-issue-484-baseline"]
    tail2 = wave.fetch("work_packages").find { |row| row.fetch("id") == "TAIL-02" }
    abort "TAIL-02 semantic drift" unless tail2.fetch("title") == "Documentation review and external-review handoff"
    tail2_spec = specs.fetch("specifications").find { |row| row.fetch("id") == "TAIL-02" }
    abort "TAIL-02 handoff acceptance missing" unless tail2_spec.fetch("acceptance").include?("external_review_handoff_complete")
  ' docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml \
    docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml \
    docs/milestones/v0.92.2/PLANNED_ISSUE_CATALOG_v0.92.2.md

  ruby -e '
    require "yaml"
    wave = YAML.safe_load(File.read(ARGV[0]), aliases: true)
    wbs = File.read(ARGV[1])
    rows = {}
    wbs.each_line do |line|
      next unless line.start_with?("|")
      cells = line.split("|").map(&:strip)
      id = cells[1]
      rows[id] = cells[4] if id&.match?(/\A(?:WP-01|CF-[A-Z]+|PLAT-[A-Z]+|OPS-AWS|PUB-[A-Z]+|SPEC-RETEST)\z/)
    end
    wave.fetch("work_packages").reject { |row| row.fetch("id").start_with?("TAIL-") }.each do |row|
      abort "WBS missing #{row.fetch("id")}" unless rows.key?(row.fetch("id"))
    end
    required = {
      "PLAT-PROVIDER" => ["WP-01", "#622"], "PLAT-MLX" => ["PLAT-PROVIDER"],
      "PLAT-MEMORY" => ["CF-EVIDENCE", "CF-MEMORY"], "CF-EVIDENCE" => ["CF-ADAPTER"],
      "CF-UX" => ["CF-SHELL", "CF-EVIDENCE"], "OPS-AWS" => ["WP-01", "#484"]
    }
    required.each { |id, deps| deps.each { |dep| abort "WBS dependency drift #{id} -> #{dep}" unless rows.fetch(id).include?(dep) } }
    sprint = File.read(ARGV[2]); readiness = File.read(ARGV[3])
    [sprint, readiness].each do |text|
      abort "missing #622 provider gate" unless text.include?("PLAT-PROVIDER") && text.include?("#622")
      abort "missing MLX serial gate" unless text.include?("PLAT-MLX") && text.include?("PLAT-PROVIDER")
    end
    abort "sprint omits exact release-tail order" unless sprint.include?("TAIL-01 through TAIL-10 in exact order")
    decisions = File.read(ARGV[4]); catalog = File.read(ARGV[5])
    abort "MLX operator admission decision missing" unless decisions.include?("CF-D11") && decisions.include?("explicitly admitted") && decisions.include?("PLAT-MLX")
    abort "#484 baseline decision missing" unless decisions.include?("CF-D12") && decisions.include?("#484") && decisions.include?("not work to repeat")
    abort "one-result issue rule missing" unless wbs.include?("exactly one bounded issue per row") && catalog.include?("exactly one bounded issue for each catalog row")
    abort "issue-combination escape hatch present" if wbs.match?(/create fewer|may (?:responsibly )?combine/i) || catalog.match?(/create fewer|may (?:responsibly )?combine/i)
  ' docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml \
    docs/milestones/v0.92.2/WBS_v0.92.2.md \
    docs/milestones/v0.92.2/SPRINT_v0.92.2.md \
    docs/milestones/v0.92.2/WP_EXECUTION_READINESS_v0.92.2.md \
    docs/milestones/v0.92.2/DECISIONS_v0.92.2.md \
    docs/milestones/v0.92.2/PLANNED_ISSUE_CATALOG_v0.92.2.md

  ruby -e '
    require "pathname"
    root = Pathname.new(ARGV.fetch(0))
    errors = []
    Dir.glob(root.join("**/*.md")).sort.each do |file|
      text = File.read(file)
      text.scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |raw|
        target = raw.split(/[?#]/, 2).first
        next if target.empty? || target.match?(/\A(?:https?:|mailto:)/)
        resolved = Pathname.new(file).dirname.join(target).cleanpath
        errors << "#{file}: unresolved link #{raw}" unless resolved.exist?
      end
    end
    abort errors.join("\n") unless errors.empty?
  ' docs/milestones/v0.92.2
}

scheduling() {
  test -f docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md
  reconciliation=docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md
  manifest=docs/milestones/v0.92.2/TBD_SOURCE_AUDIT_MANIFEST_v0.92.2.txt
  rg -q '^\| Source' "$reconciliation"
  rg -q '#484.*OPS-AWS\|OPS-AWS.*#484' "$reconciliation"
  for source in \
    TBD_DOC_STATUS_INVENTORY.md LOCAL_BACKLOG.md NEW_FEATURE_MILESTONE_ASSIGNMENT_PLAN.md \
    PROVIDER_INFERENCE_PROFILES_PLAN_v0.92.1.md MLX_APPLE_METAL_PROVIDER_PLAN.md OCI_MODEL_PACKAGING_METHOD_PLAN.md \
    UTS_STANDARDIZATION_PLAN.md ADL_REPOSITORY_CODE_REDUCTION_PLAN_v0.91.8.md ADL_STRATEGIC_COGNITIVE_RESERVE.md \
    ADL_MEDIUM_ARTICLE_LIST.md ARXIV_PAPER_PROGRAM_PLAN.md ADL_MEMORY_PALACE_ARCHITECTURE.md \
    ADL_AND_GENERIC_SPECULATIVE_DECODING.md CODE_FRIEND_v0.5_BETA.md PORTABLE_ADL_PROJECT_ADAPTER_V2_PLAN.md; do
    rg -Fq "$source" "$reconciliation"
  done
  for id in PLAT-PROVIDER PLAT-MLX PLAT-UTS PLAT-RUST OPS-AWS PUB-MEDIUM PUB-CSDLC PLAT-MEMORY SPEC-RETEST; do
    rg -q "$id" "$reconciliation"
    rg -q "id: $id" docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml
    rg -q "id: $id" docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
    for surface in PLANNED_ISSUE_CATALOG_v0.92.2.md WBS_v0.92.2.md WP_EXECUTION_READINESS_v0.92.2.md; do
      rg -q "$id" "docs/milestones/v0.92.2/$surface"
    done
  done
  ruby -e '
    reconciliation = File.read(ARGV[0])
    declared = reconciliation.scan(/`(\.adl\/docs\/TBD\/[^`]+)`/).flatten
    abort "wildcard TBD source path" if declared.any? { |path| path.include?("*") }
    manifest = File.readlines(ARGV[1], chomp: true).reject(&:empty?)
    abort "duplicate TBD source declaration" unless declared.uniq == declared
    abort "duplicate TBD manifest path" unless manifest.uniq == manifest
    missing = declared - manifest
    unused = manifest - declared
    abort "TBD source manifest mismatch; missing=#{missing.inspect}; unused=#{unused.inspect}" unless missing.empty? && unused.empty?
  ' "$reconciliation" "$manifest"
}

consistency() {
  if rg -n '/Users/|/Volumes/|<this-issue>|TBD-ISSUE|TODO-ISSUE' docs/milestones/v0.92.2; then
    echo "machine-local path or unresolved issue placeholder found" >&2
    return 1
  fi
  git diff --check origin/main...HEAD
  git diff --check
}

case "$mode" in
  structure) structure ;;
  scheduling) scheduling ;;
  consistency) consistency ;;
  all) structure; scheduling; consistency ;;
  *) echo "usage: $0 {structure|scheduling|consistency|all}" >&2; exit 2 ;;
esac

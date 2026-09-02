#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
case "${mode}" in
  denominator|gaps|decision|all) ;;
  *) echo "usage: $0 [denominator|gaps|decision|all]" >&2; exit 64 ;;
esac

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="${repo_root}/docs/milestones/v0.92.1/evidence/integration"
admission="${evidence_root}/release-tail-admission.json"
gap_json="${evidence_root}/gap_analysis_report.json"
gap_md="${evidence_root}/gap_analysis_report.md"

ruby -rjson -e '
  mode, admission_path, gap_json_path, gap_md_path = ARGV

  def load_json(path)
    abort("missing artifact: #{path}") unless File.file?(path)
    JSON.parse(File.read(path))
  rescue JSON::ParserError => error
    abort("invalid JSON #{path}: #{error.message}")
  end

  admission = load_json(admission_path) if %w[denominator decision all].include?(mode)
  gaps = load_json(gap_json_path) if %w[gaps decision all].include?(mode)

  if admission
    abort("wrong admission schema") unless admission["schema"] == "adl.v0921.release_tail_admission.v1"
    denominator = admission["denominator"]
    abort("admission denominator missing") unless denominator.is_a?(Array) && !denominator.empty?
    ids = denominator.map { |row| row["issue"] }
    abort("duplicate denominator issue") unless ids.compact.uniq.length == ids.compact.length
    denominator.each do |row|
      %w[issue acceptance_authority revision merge_ancestry artifacts implementation_evidence validation_evidence review_evidence closeout_evidence disposition].each do |key|
        abort("denominator row missing #{key}") unless row.key?(key)
      end
    end
  end

  if gaps
    abort("wrong gap-analysis mode") unless gaps["mode"] == "compare_milestone_to_evidence"
    abort("expected baseline missing") unless gaps["expected_baseline"].is_a?(Array) && !gaps["expected_baseline"].empty?
    abort("observed evidence missing") unless gaps["observed_evidence"].is_a?(Array)
    abort("findings missing") unless gaps["findings"].is_a?(Array)
    allowed_types = %w[missing_evidence implementation_gap docs_drift test_gap closeout_drift scope_ambiguity]
    allowed_classes = %w[release_blockers durable_proof_gaps routed_work stale_release_readiness_docs non_blocking_quality_concerns]
    gaps["findings"].each do |finding|
      abort("invalid finding type") unless allowed_types.include?(finding["type"])
      abort("invalid severity") unless %w[P0 P1 P2 P3].include?(finding["severity"])
      abort("invalid finding classification") unless allowed_classes.include?(finding["classification"])
      %w[evidence uncertainty disposition owner].each do |key|
        abort("finding missing #{key}") unless finding.key?(key)
      end
    end
    abort("missing Markdown gap report") unless File.file?(gap_md_path)
    markdown = File.read(gap_md_path)
    %w[Findings Denominator Limitations Decision].each do |heading|
      abort("gap report missing heading #{heading}") unless markdown.match?(/^## #{Regexp.escape(heading)}\s*$/)
    end
  end

  if %w[decision all].include?(mode)
    decision = admission["decision"]
    abort("invalid admission decision") unless %w[admitted blocked].include?(decision)
    blockers = gaps["findings"].select do |finding|
      %w[P0 P1].include?(finding["severity"]) && finding["disposition"] != "resolved"
    end
    unowned = gaps["findings"].select do |finding|
      finding["classification"] != "non_blocking_quality_concerns" && finding["owner"].to_s.strip.empty?
    end
    if decision == "admitted" && (!blockers.empty? || !unowned.empty?)
      abort("admitted decision contains unresolved blocker or unowned material gap")
    end
    abort("report decision disagrees with admission") unless gaps["decision"] == decision
    abort("report denominator disagrees with admission") unless gaps["expected_baseline"].length == admission["denominator"].length
  end

  puts JSON.generate({schema: "adl.v0921.release_tail_validation.v1", mode: mode, status: "pass"})
' "${mode}" "${admission}" "${gap_json}" "${gap_md}"

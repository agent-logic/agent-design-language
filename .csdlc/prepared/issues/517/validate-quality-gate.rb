#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
SOURCE = ROOT / "docs/milestones/v0.92.1/evidence/integration/release-tail-admission.json"
OUT = ROOT / "docs/milestones/v0.92.1/evidence/release/tail-01"
PREDECESSOR = OUT / "predecessor-516.json"
DENOMINATOR = OUT / "required-lane-denominator.json"
BLOCKERS = OUT / "blockers.json"
GATE = OUT / "quality-gate.json"
REMEDIATIONS = OUT / "candidate-remediations.json"
QUALITY_DOC = ROOT / "docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md"

SOURCE_SCHEMA = "adl.v0921.release_tail_admission.v2"
PASS_EVIDENCE = %w[proven accepted accepted_recordless accepted_amendment].freeze
FAIL_EVIDENCE = %w[implementation_gap failed].freeze
NON_PROVING_EVIDENCE = %w[proof_gap partial insufficient].freeze
REQUIRED_CHECKS = %w[adl-path-policy adl-ci adl-coverage].freeze

def read_json(path)
  JSON.parse(path.read)
rescue Errno::ENOENT, JSON::ParserError => error
  abort("#{path.relative_path_from(ROOT)}: #{error.message}")
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

def git_success?(*args)
  _stdout, _stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *args)
  status.success?
end

def git_output(*args)
  stdout, _stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *args)
  status.success? ? stdout.strip : nil
end

def source_rows(admission)
  %w[execution_issues release_tail_stages retained_predecessors].flat_map do |collection|
    Array(admission[collection]).flat_map do |unit|
      Array(unit["acceptance_rows"]).each_with_index.map do |row, index|
        [collection, unit, row, index]
      end
    end
  end
end

def lifecycle_metadata_tail?(issue, paths)
  prefix = ".csdlc/issues/#{issue}/"
  Array(paths).all? do |path|
    path == "#{prefix}audit.jsonl" ||
      path == "#{prefix}index.json" ||
      path.match?(%r{\A\.csdlc/evidence/#{issue}/review-(?:assignment-request|record-pass)-[0-9a-f]+\.json\z}) ||
      path.match?(%r{\A#{Regexp.escape(prefix)}cards/(?:sip|stp|spp|vpp|srp|sor)\.values\.json\z}) ||
      path.match?(%r{\A#{Regexp.escape(prefix)}cards/(?:srp|sor)\.md\z})
  end
end

def remediation_entries
  @remediation_entries ||= read_json(REMEDIATIONS).fetch("entries")
end

def remediation_proven?(candidate, criterion_id)
  entry = remediation_entries.find { |item| Array(item["criterion_ids"]).include?(criterion_id) }
  return false unless entry
  return false unless read_json(REMEDIATIONS)["candidate"] == candidate
  return false unless entry["issue_state"] == "closed" && entry["pull_request_state"] == "merged"
  return false unless git_success?("merge-base", "--is-ancestor", entry["merge_revision"], candidate)
  return false unless git_success?("merge-base", "--is-ancestor", entry["reviewed_revision"], entry["head_revision"])
  merge_identity_valid = if entry["merge_mode"] == "merge_commit"
                           git_output("rev-parse", "#{entry['merge_revision']}^2") == entry["head_revision"]
                         elsif entry["merge_mode"] == "squash"
                           true
                         else
                           false
                         end
  return false unless merge_identity_valid

  Array(entry["artifacts"]).all? do |artifact|
    git_output("rev-parse", "#{candidate}:#{artifact['path']}") == artifact["candidate_blob"] &&
      git_output("rev-parse", "#{entry['reviewed_revision']}:#{artifact['path']}") == artifact["reviewed_blob"]
  end
end

def current_review_truth?(unit)
  truth = unit["review_truth"] || {}
  reviewed_revision = truth["reviewed_revision"].to_s
  implementation_revision = unit["revision"].to_s
  cache_key = [unit["issue"], reviewed_revision, implementation_revision, Array(truth["post_review_paths"])]
  @current_review_truth_cache ||= {}
  return @current_review_truth_cache[cache_key] if @current_review_truth_cache.key?(cache_key)

  return false unless truth["result"] == "pass"
  return false unless reviewed_revision.match?(/\A[0-9a-f]{40}\z/) && implementation_revision.match?(/\A[0-9a-f]{40}\z/)
  return false unless git_success?("merge-base", "--is-ancestor", reviewed_revision, implementation_revision)

  @current_review_truth_cache[cache_key] = lifecycle_metadata_tail?(unit["issue"], truth["post_review_paths"])
end

def lane_result(collection, unit, row, candidate)
  evidence_status = row["evidence_status"].to_s
  proof_status = row.dig("proof", "classification").to_s
  observed_status = row["observed_status"].to_s
  status = if collection == "execution_issues" && remediation_proven?(candidate, row["id"])
             "proven"
           elsif collection == "execution_issues" && proof_status == "proven"
             current_review_truth?(unit) ? "proven" : "proof_gap"
           elsif !evidence_status.empty?
             evidence_status
           elsif !proof_status.empty?
             proof_status
           else
             observed_status
           end
  return "pass" if PASS_EVIDENCE.include?(status)
  return "fail" if FAIL_EVIDENCE.include?(status)
  return "non_proving" if NON_PROVING_EVIDENCE.include?(status)

  "absent"
end

def build_denominator(admission)
  lanes = source_rows(admission).map do |collection, unit, row, index|
    planned_id = unit["planned_id"] || "retained-#{unit['issue']}"
    required = collection != "release_tail_stages"
    criterion_id = row.is_a?(Hash) ? row["id"] : "#{planned_id}-tail-ac-#{index + 1}"
    ["#{planned_id}:#{criterion_id}", required ? lane_result(collection, unit, row, admission["candidate"]) : "not_applicable", collection]
  end
  grouped = %w[pass fail non_proving absent not_applicable].to_h do |result|
    [result, lanes.select { |_id, lane_result_value, _collection| lane_result_value == result }.map(&:first).sort]
  end
  source_result_counts = %w[execution_issues retained_predecessors].to_h do |collection|
    counts = %w[pass fail non_proving absent].to_h do |result|
      [result, lanes.count { |_id, lane_result_value, lane_collection| lane_collection == collection && lane_result_value == result }]
    end
    [collection, counts]
  end
  {
    "schema" => "adl.v0921.quality_gate_denominator.v1",
    "candidate" => admission["candidate"],
    "source_identity" => admission["output_identity"],
    "counts" => admission["counts"],
    "inventoried_lane_count" => lanes.length,
    "required_lane_count" => lanes.count { |_id, result, _collection| result != "not_applicable" },
    "source_result_counts" => source_result_counts,
    "lane_results" => grouped
  }
end

def build_blockers(admission)
  findings = Array(admission["findings"])
  projected = findings.map do |finding|
    projection = finding.slice("id", "severity", "classification", "summary", "disposition", "owner").merge(
      "affected_row_count" => Array(finding["affected_rows"]).length
    )
    if finding["id"] == "execution-exact-head-review-gaps" &&
       (review_gaps = Array(admission["execution_issues"]).select do |unit|
         unit["acceptance_rows"].any? { |row| row.dig("proof", "classification") == "proven" } && !current_review_truth?(unit)
       end).empty?
      projection["disposition"] = "resolved_by_quality_gate_recomputation"
      projection["resolution"] = "Normal typed lifecycle and exact review-evidence tails are non-substantive; reviewed revisions remain ancestral to their implementation heads."
    elsif finding["id"] == "execution-exact-head-review-gaps"
      projection["summary"] = "#{review_gaps.length} executed issues retain substantive post-review changes without current exact-head review."
      projection["affected_row_count"] = review_gaps.sum do |unit|
        unit["acceptance_rows"].count { |row| row.dig("proof", "classification") == "proven" }
      end
    elsif finding["id"].match?(/\Aissue-(?:487|491)-semantic-implementation-gap\z/) &&
          Array(finding["affected_rows"]).all? { |criterion_id| remediation_proven?(admission["candidate"], criterion_id) }
      projection["disposition"] = "resolved_by_candidate_remediation"
      projection["resolution"] = "Candidate-ancestral reviewed remediation and live proof close every affected implementation criterion."
    elsif finding["id"] == "semantic-criterion-proof-gaps"
      proof_gaps = Array(admission["execution_issues"]).flat_map do |unit|
        unit["acceptance_rows"].select do |row|
          row.dig("proof", "classification") == "proof_gap" && !remediation_proven?(admission["candidate"], row["id"])
        end
      end
      if proof_gaps.empty?
        projection["disposition"] = "resolved_by_quality_gate_recomputation"
        projection["resolution"] = "No explicit semantic proof gaps remain after candidate remediation reconciliation."
      else
        projection["summary"] = "#{proof_gaps.length} current planned criteria lack candidate-bound proof or explicit amendment authority."
        projection["affected_row_count"] = proof_gaps.length
      end
    end
    projection
  end
  resolved_dispositions = %w[accepted resolved_by_quality_gate_recomputation resolved_by_candidate_remediation]
  unresolved = projected.reject { |finding| resolved_dispositions.include?(finding["disposition"]) }
  accepted = projected.select { |finding| finding["disposition"] == "accepted" }
  resolved = projected.select { |finding| resolved_dispositions.include?(finding["disposition"]) && finding["disposition"] != "accepted" }
  {
    "schema" => "adl.v0921.quality_gate_blockers.v1",
    "candidate" => admission["candidate"],
    "unresolved" => unresolved,
    "accepted" => accepted,
    "resolved" => resolved,
    "unowned" => projected.select { |finding| finding["owner"].to_s.strip.empty? }
  }
end

def build_gate(admission, predecessor, denominator, blockers)
  result_counts = %w[pass fail non_proving absent].to_h do |result|
    [result, Array(denominator.dig("lane_results", result)).length]
  end
  passing = admission["decision"] == "admitted" &&
            result_counts.fetch("pass", 0) == denominator["required_lane_count"] &&
            blockers["unresolved"].empty? && blockers["unowned"].empty?
  {
    "schema" => "adl.v0921.quality_gate.v1",
    "issue" => 517,
    "candidate" => admission["candidate"],
    "predecessor_issue" => 516,
    "predecessor_pr" => 737,
    "predecessor_merge" => predecessor["pull_request"]["merge_sha"],
    "predecessor_base" => predecessor["pull_request"]["base_sha"],
    "admission_source" => SOURCE.relative_path_from(ROOT).to_s,
    "admission_source_sha256" => sha256(SOURCE),
    "admission_source_digest" => admission["source_digest"],
    "denominator_sha256" => Digest::SHA256.hexdigest(JSON.pretty_generate(denominator) + "\n"),
    "blockers_sha256" => Digest::SHA256.hexdigest(JSON.pretty_generate(blockers) + "\n"),
    "predecessor_sha256" => sha256(PREDECESSOR),
    "candidate_remediations_sha256" => sha256(REMEDIATIONS),
    "inventoried_lane_count" => denominator["inventoried_lane_count"],
    "required_lane_count" => denominator["required_lane_count"],
    "downstream_stage_lane_count" => Array(denominator.dig("lane_results", "not_applicable")).length,
    "lane_result_counts" => result_counts,
    "lane_result_counts_by_source" => denominator["source_result_counts"],
    "unresolved_exception_count" => blockers["unresolved"].length,
    "unowned_exception_count" => blockers["unowned"].length,
    "decision" => passing ? "passed" : "blocked",
    "downstream_unlock" => passing
  }
end

def validate_bundle(admission, predecessor, denominator, blockers, gate, filesystem: true)
  errors = []
  errors << "source_schema_invalid" unless admission["schema"] == SOURCE_SCHEMA
  errors << "source_candidate_missing" unless admission["candidate"].to_s.match?(/\A[0-9a-f]{40}\z/)
  errors << "source_counts_missing" unless admission["counts"].is_a?(Hash)
  errors << "source_decision_invalid" unless %w[admitted blocked].include?(admission["decision"])

  rows = source_rows(admission)
  expected_count = admission.dig("counts", "acceptance_rows")
  errors << "source_acceptance_count_mismatch" unless expected_count == rows.length
  errors << "denominator_schema_invalid" unless denominator["schema"] == "adl.v0921.quality_gate_denominator.v1"
  errors << "denominator_candidate_mismatch" unless denominator["candidate"] == admission["candidate"]
  lane_results = denominator["lane_results"] || {}
  allowed_results = %w[pass fail non_proving absent not_applicable]
  lanes = lane_results.values.flat_map { |ids| Array(ids) }
  required_lanes = %w[pass fail non_proving absent].flat_map { |result| Array(lane_results[result]) }
  errors << "denominator_count_mismatch" unless denominator["inventoried_lane_count"] == rows.length &&
                                                lanes.length == rows.length &&
                                                denominator["required_lane_count"] == required_lanes.length
  errors << "denominator_duplicate_lane" unless lanes.uniq.length == lanes.length
  errors << "denominator_result_invalid" unless lane_results.keys.sort == allowed_results.sort &&
                                                lane_results.values.all? { |ids| ids.is_a?(Array) && ids.all? { |id| id.is_a?(String) && !id.empty? } }
  errors << "denominator_projection_mismatch" unless denominator == build_denominator(admission)

  errors << "blocker_schema_invalid" unless blockers["schema"] == "adl.v0921.quality_gate_blockers.v1"
  errors << "blocker_candidate_mismatch" unless blockers["candidate"] == admission["candidate"]
  unresolved = Array(blockers["unresolved"])
  accepted = Array(blockers["accepted"])
  errors << "exception_owner_missing" if (unresolved + accepted).any? { |finding| finding["owner"].to_s.strip.empty? }
  expected_unresolved = build_blockers(admission)["unresolved"]
  errors << "blocker_denominator_mismatch" unless unresolved.map { |f| f["id"] }.sort == expected_unresolved.map { |f| f["id"] }.sort
  errors << "blocker_projection_mismatch" unless blockers == build_blockers(admission)

  expected_gate = build_gate(admission, predecessor, denominator, blockers)
  expected_gate.each do |key, value|
    errors << "gate_#{key}_mismatch" unless gate[key] == value
  end
  errors << "predecessor_issue_not_closed" unless predecessor.dig("issue", "state") == "closed"
  errors << "predecessor_pr_not_merged" unless predecessor.dig("pull_request", "state") == "merged"
  errors << "predecessor_review_not_exact" unless predecessor.dig("review", "result") == "pass" &&
                                                  predecessor.dig("review", "reviewed_head") == predecessor.dig("pull_request", "head_sha")
  errors << "predecessor_candidate_mismatch" unless predecessor.dig("pull_request", "base_sha") == admission["candidate"]
  checks = predecessor.dig("pull_request", "required_checks") || {}
  errors << "predecessor_required_checks_missing" unless REQUIRED_CHECKS.all? { |name| checks[name] == "success" }

  if filesystem
    immutable = ROOT / "docs/milestones/v0.92.1/evidence/integration" / admission["output_identity"].to_s
    errors << "immutable_admission_missing" unless immutable.file?
    errors << "mutable_admission_alias_drift" if immutable.file? && immutable.read != SOURCE.read
    errors << "candidate_commit_missing" unless git_success?("cat-file", "-e", "#{admission['candidate']}^{commit}")
    merge_sha = predecessor.dig("pull_request", "merge_sha").to_s
    head_sha = predecessor.dig("pull_request", "head_sha").to_s
    base_sha = predecessor.dig("pull_request", "base_sha").to_s
    errors << "predecessor_merge_missing" unless git_success?("cat-file", "-e", "#{merge_sha}^{commit}")
    errors << "candidate_not_ancestral" unless git_success?("merge-base", "--is-ancestor", admission["candidate"].to_s, merge_sha)
    errors << "predecessor_merge_candidate_mismatch" unless git_output("rev-parse", "#{merge_sha}^1") == base_sha
    errors << "predecessor_merge_head_mismatch" unless git_output("rev-parse", "#{merge_sha}^2") == head_sha
    errors << "predecessor_merge_identity_mismatch" unless git_output("show", "-s", "--format=%s", merge_sha) == "Merge pull request #737 from agent-logic/codex/516-release-tail-admission"
    quality_text = QUALITY_DOC.read
    counts = gate["lane_result_counts"]
    errors << "quality_document_stale" unless quality_text.include?("Candidate: `#{admission['candidate']}`") &&
                                                   quality_text.include?("Decision: **#{gate['decision'].upcase}**") &&
                                                   quality_text.include?("#{counts['pass']} passing, #{counts['fail']} failing, #{counts['non_proving']} non-proving, and #{counts['absent']} absent") &&
                                                   quality_text.include?("#{gate['unresolved_exception_count']} unresolved exceptions remain")
  end
  errors
end

def write_json(path, value)
  path.dirname.mkpath
  path.write(JSON.pretty_generate(value) + "\n")
end

def generate
  admission = read_json(SOURCE)
  predecessor = read_json(PREDECESSOR)
  denominator = build_denominator(admission)
  blockers = build_blockers(admission)
  write_json(DENOMINATOR, denominator)
  write_json(BLOCKERS, blockers)
  write_json(GATE, build_gate(admission, predecessor, denominator, blockers))
  puts JSON.generate(schema: "adl.v0921.quality_gate_generation.v1", status: "generated", inventoried_lanes: denominator["inventoried_lane_count"], required_lanes: denominator["required_lane_count"])
end

def validate
  admission = read_json(SOURCE)
  predecessor = read_json(PREDECESSOR)
  denominator = read_json(DENOMINATOR)
  blockers = read_json(BLOCKERS)
  gate = read_json(GATE)
  errors = validate_bundle(admission, predecessor, denominator, blockers, gate)
  if errors.empty?
    puts JSON.generate(schema: "adl.v0921.quality_gate_validation.v1", status: "passed", gate_decision: gate["decision"], inventoried_lanes: denominator["inventoried_lane_count"], required_lanes: denominator["required_lane_count"])
  else
    warn JSON.generate(schema: "adl.v0921.quality_gate_validation.v1", status: "failed", errors: errors.uniq)
    exit 1
  end
end

def negative
  admission = read_json(SOURCE)
  predecessor = read_json(PREDECESSOR)
  denominator = build_denominator(admission)
  blockers = build_blockers(admission)
  gate = build_gate(admission, predecessor, denominator, blockers)
  cases = {
    "missing-lane" => ["denominator_count_mismatch", lambda { |_a, d, _b, _g| d["lane_results"]["non_proving"].pop }],
    "retained-gap-as-absent" => ["denominator_projection_mismatch", lambda do |_a, d, _b, _g|
      retained = d["lane_results"]["non_proving"].find { |id| id.include?("retained-") }
      d["lane_results"]["non_proving"].delete(retained)
      d["lane_results"]["absent"] << retained
    end],
    "substantive-review-tail" => ["denominator_projection_mismatch", lambda do |a, _d, _b, _g|
      unit = a["execution_issues"].find { |candidate| candidate["acceptance_rows"].any? { |row| row.dig("proof", "classification") == "proven" } }
      unit["review_truth"]["post_review_paths"] << "src/substantive-change.rs"
    end],
    "skipped-lane" => ["denominator_result_invalid", lambda { |_a, d, _b, _g| d["lane_results"]["skipped"] = [d["lane_results"]["non_proving"].shift] }],
    "zero-test-lane" => ["denominator_result_invalid", lambda { |_a, d, _b, _g| d["lane_results"]["zero_test"] = [d["lane_results"]["non_proving"].shift] }],
    "stale-candidate" => ["predecessor_candidate_mismatch", lambda { |a, _d, _b, _g| a["candidate"] = "0" * 40 }],
    "false-green" => ["gate_decision_mismatch", lambda { |_a, _d, _b, g| g["decision"] = "passed" }],
    "unowned-unresolved-exception" => ["exception_owner_missing", lambda { |_a, _d, b, _g| b["unresolved"][0]["owner"] = "" }],
    "unowned-accepted-exception" => ["exception_owner_missing", lambda { |_a, _d, b, _g| b["accepted"][0]["owner"] = "" }]
  }
  failures = []
  cases.each do |name, (expected, mutation)|
    a = Marshal.load(Marshal.dump(admission))
    d = Marshal.load(Marshal.dump(denominator))
    b = Marshal.load(Marshal.dump(blockers))
    g = Marshal.load(Marshal.dump(gate))
    mutation.call(a, d, b, g)
    errors = validate_bundle(a, predecessor, d, b, g, filesystem: false)
    failures << "#{name}:expected_#{expected}:got_#{errors.join(',')}" unless errors.include?(expected)
  end
  if failures.empty?
    puts JSON.generate(schema: "adl.v0921.quality_gate_negative.v1", status: "passed", cases: cases.length)
  else
    warn JSON.generate(schema: "adl.v0921.quality_gate_negative.v1", status: "failed", errors: failures)
    exit 1
  end
end

case ARGV.first
when "--generate" then generate
when "--negative" then negative
when nil then validate
else abort("usage: #{File.basename(__FILE__)} [--generate|--negative]")
end

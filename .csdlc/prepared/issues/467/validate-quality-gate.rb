#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "net/http"
require "openssl"
require "open3"
require "pathname"
require "time"
require "uri"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
ISSUE = 467
REPOSITORY = "agent-logic/agent-design-language"
FEATURE_INDEX = ROOT / "docs/milestones/v0.92/features/README.md"
COVERAGE = ROOT / "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
OUT_DIR = ROOT / "docs/reviews/v0.92/quality-gate-467"
MATRIX_PATH = OUT_DIR / "feature-completion-matrix.json"
GATE_PATH = OUT_DIR / "quality-gate-record.json"
REPORT_PATH = OUT_DIR / "blocker-report.md"
SUPERSESSION_PATH = OUT_DIR / "311-supersession.md"
EVIDENCE_DIR = ROOT / ".csdlc/evidence/467"
REQUIRED_CHECKS = %w[adl-ci adl-coverage].freeze
ALLOWED_DISPOSITIONS = %w[accepted blocked].freeze
BLOCKER_KINDS = %w[
  implementation_missing required_proof_missing evidence_stale_non_ancestral
  evidence_mapping_missing planned_deferred normalization_mapping_missing
].freeze
PROOF_CLASSES = %w[positive negative integration platform].freeze
PROHIBITED_AUTHORITY = %w[fixture receipt_only demo synthetic substituted_provider self_asserted_json].freeze
ACCEPTED_PROFILES = {
  "feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92" => {
    issue: 451, pull_request: 459,
    reviewed_head: "3c612a0c302d1a34562b9e0c160b12aca91222e3",
    pr_head: "414777b543bf5df295a41eacc9c4fd19735c413b",
    merge_sha: "e926e3bca0ab1981d77b4658d2feb4059bdf33a6",
    implementation_paths: ["adl/src/production_birthday.rs"],
    proofs: {
      positive: [".csdlc/evidence/451/production_birthday_kernel.log", 1],
      negative: [".csdlc/evidence/451/retained_evidence_contract.log", 3],
      integration: [".csdlc/evidence/451/production_birthday_resident_path.log", 2],
      platform: [".csdlc/evidence/451/runtime_feature_wiring_audit.log", 4]
    },
    claim_boundary: "Accepted only for the exact #451 production birthday composition row; it does not grant broader v0.92 release readiness."
  },
  "critical:AEE-008" => {
    issue: 451, pull_request: 459,
    reviewed_head: "3c612a0c302d1a34562b9e0c160b12aca91222e3",
    pr_head: "414777b543bf5df295a41eacc9c4fd19735c413b",
    merge_sha: "e926e3bca0ab1981d77b4658d2feb4059bdf33a6",
    implementation_paths: ["adl/src/production_birthday.rs"],
    proofs: {
      positive: [".csdlc/evidence/451/production_birthday_kernel.log", 1],
      negative: [".csdlc/evidence/451/retained_evidence_contract.log", 3],
      integration: [".csdlc/evidence/451/production_birthday_resident_path.log", 2],
      platform: [".csdlc/evidence/451/runtime_feature_wiring_audit.log", 4]
    },
    claim_boundary: "Accepted only for the AEE-008 birthday and identity critical path from #451; adjacent identity, witness, and release rows remain independently gated."
  },
  "feature:ADAPTIVE_LEARNING_DAG_v0.92" => {
    issue: 449, pull_request: 456,
    reviewed_head: "43b9cf33c58c2091223684e32efca9b15db135e6",
    pr_head: "5476288e0cc0e66de823df0c080aae4f2f852aa5",
    merge_sha: "d834c136a12e66d2334bcea5e36d860b290c7121",
    implementation_paths: [
      "adl-runtime-kernel/src/adaptive_learning.rs",
      "adl-runtime-kernel/src/resident_cycle.rs",
      "adl-runtime-kernel/src/live_continuity.rs"
    ],
    proofs: {
      positive: [".csdlc/evidence/449/adaptive-learning-regression-tests.log", 2],
      negative: [".csdlc/evidence/449/feature-evidence-truth-check.log", 7],
      integration: [".csdlc/evidence/449/runtime-resident-cycle-integration-proof.log", 1],
      platform: [".csdlc/evidence/449/diff-hygiene.log", 5]
    },
    claim_boundary: "Accepted only for the #449 governed Adaptive Learning DAG resident-cycle integration row; it does not grant broader cognitive profile, memory, or runtime release readiness."
  }
}.freeze

def env
  ENV.keys.grep(/\AGIT_/).to_h { |key| [key, nil] }.merge("PATH" => "/usr/bin:/bin", "GIT_CONFIG_NOSYSTEM" => "1", "GIT_CONFIG_GLOBAL" => "/dev/null")
end

def git(*argv)
  out, err, status = Open3.capture3(env, "/usr/bin/git", "-C", ROOT.to_s, *argv)
  raise "git #{argv.join(' ')} failed: #{err.strip}" unless status.success?
  out.strip
end

def git_bytes(*argv)
  out, err, status = Open3.capture3(env, "/usr/bin/git", "-C", ROOT.to_s, *argv)
  raise "git #{argv.join(' ')} failed: #{err.strip}" unless status.success?
  out
end

def ancestor?(a, b)
  _out, _err, status = Open3.capture3(env, "/usr/bin/git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", a, b)
  status.success?
end

def sha256_path(path)
  Digest::SHA256.file(path).hexdigest
end

def sha256_bytes(bytes)
  Digest::SHA256.hexdigest(bytes)
end

def safe_rel?(value)
  value.is_a?(String) && !value.start_with?("/") && !Pathname.new(value).each_filename.include?("..")
end

def denominator
  feature_text = FEATURE_INDEX.read
  feature_section = feature_text.split("## Feature Documents", 2).fetch(1).split("## WP Coverage Map", 2).first
  features = feature_section.scan(/\]\(([^)]+\.md)\)/).flatten.map do |relative|
    line = feature_text.lines.find { |candidate| candidate.start_with?("|") && candidate.include?("](#{relative})") }
    owner = line ? line.split("|").map(&:strip)[1] : "unmapped feature owner"
    {
      "id" => "feature:#{File.basename(relative, '.md')}", "kind" => "feature",
      "source" => "docs/milestones/v0.92/features/#{relative}", "owner" => owner,
      "source_status" => "feature_contract"
    }
  end
  critical = COVERAGE.read.lines.each_with_object([]) do |line, rows|
    next unless line.start_with?("|")
    cells = line.split("|").map(&:strip)
    next unless cells.length >= 6 && cells[5]&.match?(/^AEE-\d{3}$/)
    rows << {
      "id" => "critical:#{cells[5]}", "kind" => "critical_path",
      "source" => COVERAGE.relative_path_from(ROOT).to_s, "owner" => cells[2],
      "source_status" => cells[4], "outcome" => cells[1]
    }
  end.uniq { |row| row["id"] }
  raise "feature denominator must contain 13 rows" unless features.length == 13
  raise "critical denominator must contain 20 rows" unless critical.length == 20
  rows = features + critical
  raise "denominator contains duplicate ids" unless rows.map { |row| row["id"] }.uniq.length == rows.length
  rows
end

def duplicate_coverage_ids
  ids = COVERAGE.read.lines.each_with_object([]) do |line, values|
    next unless line.start_with?("|")
    cells = line.split("|").map(&:strip)
    values << cells[5] if cells.length >= 6 && cells[5]&.match?(/^AEE-\d{3}$/)
  end
  counts = ids.each_with_object(Hash.new(0)) { |id, memo| memo[id] += 1 }
  counts.select { |_id, count| count > 1 }.keys.map { |id| "critical:#{id}" }
end

def terminal_receipt(issue)
  common = Pathname.new(git("rev-parse", "--git-common-dir")).realpath
  bin = common.parent / ".adl/bin/csdlc-v2/csdlc-finish"
  out, err, status = Open3.capture3(env, bin.to_s, "--root", ROOT.to_s, "--validate-cached-issue", issue.to_s)
  raise "typed terminal unavailable for #{issue}: #{err}" unless status.success?
  JSON.parse(out)
end

def blob_ref(commit, path)
  bytes = git_bytes("show", "#{commit}:#{path}")
  { "path" => path, "sha256" => sha256_bytes(bytes) }
end

def proof_refs(profile)
  profile.fetch(:proofs).transform_values do |(path, index)|
    blob_ref(profile.fetch(:pr_head), path).merge("validation_index" => index)
  end
end

def row_contract(row, profile, proofs, validations)
  source_bytes = git_bytes("show", "#{profile.fetch(:reviewed_head)}:#{row.fetch('source')}")
  proof_binding = PROOF_CLASSES.map do |klass|
    ref = proofs.fetch(klass.to_sym)
    lane = validations.fetch(ref.fetch("validation_index"))
    { "class" => klass, "path" => ref["path"], "sha256" => ref["sha256"], "command" => lane["command"], "evidence_ref" => lane["evidence_ref"] }
  end
  {
    "schema" => "adl.v0.92.quality_gate_row_contract.v2",
    "row_id" => row.fetch("id"), "owner" => row.fetch("owner"), "source_path" => row.fetch("source"),
    "source_sha256" => sha256_bytes(source_bytes), "issue" => profile.fetch(:issue),
    "implementation_paths" => profile.fetch(:implementation_paths).sort,
    "proof_binding_sha256" => sha256_bytes(JSON.generate(proof_binding))
  }
end

def accepted_evidence(row, profile)
  terminal = terminal_receipt(profile.fetch(:issue))
  EVIDENCE_DIR.mkpath
  terminal_path = EVIDENCE_DIR / "terminal-#{profile.fetch(:issue)}.json"
  terminal_path.write(JSON.pretty_generate(terminal) + "\n")
  sor = JSON.parse(git_bytes("show", "#{profile.fetch(:pr_head)}:.csdlc/issues/#{profile.fetch(:issue)}/cards/sor.values.json"))
  validations = Array(sor.dig("content", "values", "actual_validation"))
  proofs = proof_refs(profile)
  {
    "authority_kind" => "canonical_observation",
    "repository" => REPOSITORY,
    "issue" => profile.fetch(:issue),
    "implementation_paths" => profile.fetch(:implementation_paths).sort,
    "reviewed_head" => profile.fetch(:reviewed_head),
    "pr_head" => profile.fetch(:pr_head),
    "pull_request" => profile.fetch(:pull_request),
    "merge_sha" => profile.fetch(:merge_sha),
    "positive" => proofs.fetch(:positive),
    "negative" => proofs.fetch(:negative),
    "integration" => proofs.fetch(:integration),
    "platform" => proofs.fetch(:platform),
    "typed_terminal" => {
      "generation" => terminal.dig("terminal", "canonical_generation"),
      "digest" => terminal.dig("terminal", "canonical_digest"),
      "cache" => { "path" => terminal_path.relative_path_from(ROOT).to_s, "sha256" => sha256_path(terminal_path) }
    },
    "review_artifact" => blob_ref(profile.fetch(:pr_head), ".csdlc/issues/#{profile.fetch(:issue)}/index.json"),
    "required_checks" => REQUIRED_CHECKS,
    "row_contract" => row_contract(row, profile, proofs, validations)
  }
end

def blocker_for(row)
  if row["source_status"] == "planned"
    ["planned_deferred", "planned_or_deferred_by_explicit_milestone_authority"]
  elsif row["kind"] == "feature"
    ["evidence_mapping_missing", "feature_has_no_current_canonical_accepted_evidence_mapping"]
  elsif row["source_status"] == "implemented_with_evidence"
    ["required_proof_missing", "implemented_with_evidence_row_requires_full_accepted_packet_binding"]
  elsif duplicate_coverage_ids.include?(row["id"])
    ["normalization_mapping_missing", "duplicate_source_coverage_row_requires_split_before_release_credit"]
  else
    ["required_proof_missing", "coverage_status_not_accepted:#{row['source_status']}"]
  end
end

def build_matrix
  rows = denominator.map do |entry|
    if (profile = ACCEPTED_PROFILES[entry["id"]])
      entry.merge(
        "disposition" => "accepted", "claim_boundary" => profile.fetch(:claim_boundary),
        "discovery" => { "status" => "investigated", "profile" => "canonical_row_profile" },
        "blocker_kind" => nil, "blockers" => [], "evidence" => accepted_evidence(entry, profile)
      )
    else
      kind, blocker = blocker_for(entry)
      entry.merge(
        "disposition" => "blocked",
        "claim_boundary" => "No release credit until a row-specific canonical accepted-evidence packet satisfies every required field.",
        "discovery" => { "status" => "investigated", "profile" => "no_accepted_profile" },
        "blocker_kind" => kind, "blockers" => [blocker], "evidence" => {}
      )
    end
  end
  {
    "schema" => "adl.v0.92.quality_gate_matrix.v2",
    "milestone" => "v0.92",
    "issue" => ISSUE,
    "supersedes" => { "issue" => 311, "pull_request" => 466, "path" => "docs/reviews/v0.92/quality-gate-311" },
    "evaluation_base_sha" => git("rev-parse", "HEAD"),
    "denominator" => { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 },
    "completion_guard" => "reject_uninvestigated_all_blocked",
    "rows" => rows
  }
end

def github_token
  path = Pathname.new(ENV.fetch("ADL_GITHUB_TOKEN_FILE", File.join(Dir.home, "keys/github.token")))
  raise "github token path invalid" unless path.absolute? && path.file? && !path.symlink?
  raise "github token permissions invalid" unless (path.stat.mode & 0o077).zero?
  token = path.read.strip
  raise "github token empty" if token.empty?
  token
end

def github(path)
  uri = URI("https://api.github.com#{path}")
  request = Net::HTTP::Get.new(uri)
  request["Authorization"] = "Bearer #{github_token}"
  request["Accept"] = "application/vnd.github+json"
  request["X-GitHub-Api-Version"] = "2022-11-28"
  request["User-Agent"] = "adl-467-quality-gate"
  http = Net::HTTP.new(uri.hostname, uri.port, nil)
  http.use_ssl = true
  http.verify_mode = OpenSSL::SSL::VERIFY_PEER
  response = http.start { |connection| connection.request(request) }
  raise "github #{path} #{response.code}" unless response.is_a?(Net::HTTPSuccess)
  JSON.parse(response.body)
end

def github_checks(commit)
  checks = []
  page = 1
  loop do
    payload = github("/repos/#{REPOSITORY}/commits/#{commit}/check-runs?filter=all&per_page=100&page=#{page}")
    batch = Array(payload["check_runs"])
    checks.concat(batch)
    break if batch.length < 100
    page += 1
  end
  checks
end

def validate_hex(value, length, label, errors)
  errors << "#{label}:invalid" unless value.is_a?(String) && value.match?(/\A[0-9a-f]{#{length}}\z/)
end

def validate_ref(ref, commit, label, errors)
  unless ref.is_a?(Hash) && safe_rel?(ref["path"]) && ref["sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
    errors << "#{label}:reference_invalid"
    return nil
  end
  bytes = git_bytes("show", "#{commit}:#{ref['path']}")
  errors << "#{label}:digest_mismatch" unless sha256_bytes(bytes) == ref["sha256"]
  bytes
rescue StandardError
  errors << "#{label}:unresolvable"
  nil
end

def validate_live_checks(evidence, row_id, errors)
  observed = github_checks(evidence.fetch("pr_head"))
  Array(evidence["required_checks"]).each do |name|
    runs = observed.select { |run| run["name"] == name && run.dig("app", "id").is_a?(Integer) }
    latest = runs.max_by { |run| Time.iso8601(run["completed_at"]) rescue Time.at(0) }
    errors << "#{row_id}:required_check_not_successful:#{name}" unless latest && latest["conclusion"] == "success"
  end
rescue StandardError => error
  errors << "#{row_id}:github_check_observation_failed:#{error.message}"
end

def validate_accepted(row, expected, errors, canonical:)
  row_id = row.fetch("id")
  evidence = row["evidence"]
  unless evidence.is_a?(Hash) && expected
    errors << "#{row_id}:accepted_without_canonical_profile"
    return
  end
  errors << "#{row_id}:prohibited_authority:#{evidence['authority_kind']}" if PROHIBITED_AUTHORITY.include?(evidence["authority_kind"])
  errors << "#{row_id}:authority_kind_invalid" unless evidence["authority_kind"] == "canonical_observation"
  errors << "#{row_id}:repository_mismatch" unless evidence["repository"] == REPOSITORY
  %i[issue pull_request reviewed_head pr_head merge_sha].each do |key|
    errors << "#{row_id}:#{key}_mismatch" unless evidence[key.to_s] == expected.fetch(key)
  end
  errors << "#{row_id}:implementation_paths_mismatch" unless Array(evidence["implementation_paths"]).sort == expected.fetch(:implementation_paths).sort
  validate_hex(evidence["reviewed_head"], 40, "#{row_id}:reviewed_head", errors)
  validate_hex(evidence["pr_head"], 40, "#{row_id}:pr_head", errors)
  validate_hex(evidence["merge_sha"], 40, "#{row_id}:merge_sha", errors)
  return unless evidence["reviewed_head"].to_s.match?(/\A[0-9a-f]{40}\z/) && evidence["pr_head"].to_s.match?(/\A[0-9a-f]{40}\z/) && evidence["merge_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)

  begin
    errors << "#{row_id}:reviewed_head_not_in_pr_head" unless ancestor?(evidence["reviewed_head"], evidence["pr_head"])
    tree_equal = git("rev-parse", "#{evidence['pr_head']}^{tree}") == git("rev-parse", "#{evidence['merge_sha']}^{tree}")
    errors << "#{row_id}:pr_head_not_merged" unless ancestor?(evidence["pr_head"], evidence["merge_sha"]) || tree_equal
    errors << "#{row_id}:merge_not_ancestral" unless ancestor?(evidence["merge_sha"], "HEAD")
    Array(evidence["implementation_paths"]).each { |path| git("cat-file", "-e", "#{evidence['reviewed_head']}:#{path}") }
  rescue StandardError
    errors << "#{row_id}:git_identity_unresolvable"
  end

  terminal_ref = evidence.dig("typed_terminal", "cache")
  terminal = nil
  if terminal_ref.is_a?(Hash) && safe_rel?(terminal_ref["path"]) && (ROOT / terminal_ref["path"]).file?
    errors << "#{row_id}:typed_terminal_cache:digest_mismatch" unless terminal_ref["sha256"] == sha256_path(ROOT / terminal_ref["path"])
    terminal = JSON.parse((ROOT / terminal_ref["path"]).read)
  else
    errors << "#{row_id}:typed_terminal_cache_missing"
  end
  if terminal
    expected_terminal = terminal_receipt(expected.fetch(:issue))
    errors << "#{row_id}:typed_terminal_not_canonical" unless terminal == expected_terminal && terminal["canonical_match"] == true
    errors << "#{row_id}:typed_terminal:generation_mismatch" unless evidence.dig("typed_terminal", "generation") == terminal.dig("terminal", "canonical_generation")
    errors << "#{row_id}:typed_terminal:digest_mismatch" unless evidence.dig("typed_terminal", "digest") == terminal.dig("terminal", "canonical_digest")
    errors << "#{row_id}:typed_terminal:merge_sha_mismatch" unless terminal.dig("terminal", "merge_sha") == evidence["merge_sha"]
  end

  review_bytes = validate_ref(evidence["review_artifact"], evidence["pr_head"], "#{row_id}:review_artifact", errors)
  review = review_bytes ? JSON.parse(review_bytes) : nil
  if review
    errors << "#{row_id}:review_issue_mismatch" unless review["issue"] == evidence["issue"]
    errors << "#{row_id}:review_not_complete" unless review.dig("review", "completed") == true && Array(review.dig("review", "findings")).empty?
    errors << "#{row_id}:review_revision_mismatch" unless review.dig("review", "reviewed_revision").to_s.include?(evidence["reviewed_head"])
  end
  validations = JSON.parse(git_bytes("show", "#{evidence['pr_head']}:.csdlc/issues/#{evidence['issue']}/cards/sor.values.json")).dig("content", "values", "actual_validation")
  proofs = PROOF_CLASSES.map do |klass|
    ref = evidence[klass]
    bytes = validate_ref(ref, evidence["pr_head"], "#{row_id}:#{klass}", errors)
    lane = validations&.[](ref["validation_index"]) if ref.is_a?(Hash) && ref["validation_index"].is_a?(Integer)
    errors << "#{row_id}:#{klass}:lane_not_passed" unless lane && lane["outcome"] == "passed"
    [klass, ref && ref["path"], bytes && sha256_bytes(bytes)]
  end
  errors << "#{row_id}:proof_paths_not_distinct" unless proofs.map { |_k, path, _sha| path }.compact.uniq.length == PROOF_CLASSES.length
  contract = row_contract(row, expected, PROOF_CLASSES.to_h { |klass| [klass.to_sym, evidence[klass]] }, validations)
  errors << "#{row_id}:row_contract_mismatch" unless evidence["row_contract"] == contract
  errors << "#{row_id}:required_checks_not_canonical" unless evidence["required_checks"] == REQUIRED_CHECKS
  validate_live_checks(evidence, row_id, errors) if canonical
rescue JSON::ParserError
  errors << "#{row_id}:json_artifact_invalid"
end

def validate_matrix(path, canonical: true)
  matrix = JSON.parse(path.read)
  errors = []
  errors << "schema_invalid" unless matrix["schema"] == "adl.v0.92.quality_gate_matrix.v2"
  errors << "issue_invalid" unless matrix["issue"] == ISSUE
  errors << "denominator_invalid" unless matrix["denominator"] == { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 }
  expected = denominator
  expected_ids = expected.map { |row| row["id"] }
  rows = Array(matrix["rows"])
  observed_ids = rows.map { |row| row["id"] }
  errors << "denominator_missing:#{(expected_ids - observed_ids).join(',')}" unless (expected_ids - observed_ids).empty?
  errors << "denominator_extra:#{(observed_ids - expected_ids).join(',')}" unless (observed_ids - expected_ids).empty?
  counts = observed_ids.each_with_object(Hash.new(0)) { |id, memo| memo[id] += 1 }
  dupes = counts.select { |_id, count| count > 1 }.keys
  errors << "denominator_duplicate:#{dupes.join(',')}" unless dupes.empty?
  expected_by_id = expected.to_h { |row| [row["id"], row] }
  rows.each do |row|
    id = row["id"]
    next unless expected_by_id.key?(id)
    expected_row = expected_by_id[id]
    %w[kind source owner source_status].each { |key| errors << "#{id}:#{key}_mismatch" unless row[key] == expected_row[key] }
    errors << "#{id}:disposition_invalid" unless ALLOWED_DISPOSITIONS.include?(row["disposition"])
    discovery = row["discovery"]
    errors << "#{id}:discovery_uninvestigated" unless discovery.is_a?(Hash) && discovery["status"] == "investigated"
    if ACCEPTED_PROFILES.key?(id) && row["disposition"] != "accepted"
      errors << "#{id}:discoverable_evidence_suppressed"
    end
    if row["disposition"] == "accepted"
      errors << "#{id}:accepted_has_blockers" unless Array(row["blockers"]).empty?
      validate_accepted(row, ACCEPTED_PROFILES[id], errors, canonical: canonical)
    else
      errors << "#{id}:blocked_without_reason" if Array(row["blockers"]).empty?
      errors << "#{id}:blocker_kind_invalid" unless BLOCKER_KINDS.include?(row["blocker_kind"])
      if Array(row["blockers"]).any? { |blocker| blocker == "accepted_evidence_packet_missing" || blocker == "evidence_normalization_missing" }
        errors << "#{id}:normalization_gap_not_concrete_blocker"
      end
    end
  end
  if rows.all? { |row| row["disposition"] == "blocked" } && rows.any? { |row| !row["discovery"].is_a?(Hash) || row["discovery"]["status"] != "investigated" }
    errors << "vacuous_all_blocked_publication"
  end
  if canonical && errors.empty?
    validate_packet_consistency(matrix, errors)
  end
  [matrix, errors]
end

def validate_packet_consistency(matrix, errors)
  gate = JSON.parse(GATE_PATH.read)
  receipt = JSON.parse((EVIDENCE_DIR / "validation.json").read)
  report = REPORT_PATH.read
  accepted = matrix["rows"].count { |row| row["disposition"] == "accepted" }
  blocked = matrix["rows"].count { |row| row["disposition"] == "blocked" }
  result = blocked.zero? ? "passed" : "blocked"
  expected_gate = {
    "schema" => "adl.v0.92.quality_gate_record.v2", "issue" => ISSUE,
    "supersedes_issue" => 311, "supersedes_pr" => 466,
    "matrix_sha256" => sha256_path(MATRIX_PATH), "validator_sha256" => sha256_path(Pathname.new(__FILE__)),
    "feature_rows" => 13, "critical_path_rows" => 20, "accepted_rows" => accepted,
    "blocked_rows" => blocked, "result" => result, "downstream_unlock" => blocked.zero?
  }
  expected_gate.each { |key, value| errors << "packet:gate:#{key}_mismatch" unless gate[key] == value }
  errors << "packet:receipt:matrix_sha256_mismatch" unless receipt["matrix_sha256"] == sha256_path(MATRIX_PATH)
  errors << "packet:receipt:gate_sha256_mismatch" unless receipt["gate_sha256"] == sha256_path(GATE_PATH)
  errors << "packet:receipt:result_mismatch" unless receipt["quality_gate_result"] == result
  matrix["rows"].select { |row| row["disposition"] == "blocked" }.each do |row|
    errors << "packet:report:row_missing:#{row['id']}" unless report.include?("- `#{row['id']}` — #{row['blocker_kind']}: #{row['blockers'].join(', ')}")
  end
end

def write_docs_notes(matrix)
  accepted = matrix["rows"].select { |row| row["disposition"] == "accepted" }.map { |row| row["id"] }
  blocked_count = matrix["rows"].count { |row| row["disposition"] == "blocked" }
  quality = ROOT / "docs/milestones/v0.92/QUALITY_GATE_v0.92.md"
  readiness = ROOT / "docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md"
  coverage = ROOT / "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
  marker = "\n## WP-22A Corrective Hydration\n\n"
  quality_text = quality.read.split(marker, 2).first + marker +
    "Issue #467 supersedes the #311 structural packet for release-credit semantics. The corrective packet lives at `docs/reviews/v0.92/quality-gate-467/`, accepts #{accepted.join(', ')}, and keeps #{blocked_count} rows blocked with concrete blocker taxonomy. #311/PR #466 remain historical provenance only.\n"
  readiness_text = readiness.read.split(marker, 2).first + marker +
    "WP-22A is executing under #467 because #311/PR #466 published a vacuous all-blocked packet. Downstream WP-23 through WP-30 remain blocked until the #467 packet has zero concrete blocker rows; administrative closeout is not a dependency edge.\n"
  coverage_text = coverage.read.split(marker, 2).first + marker +
    "The #467 corrective quality gate grants accepted release credit only to rows with complete canonical hydration. Current accepted rows are #{accepted.join(', ')}; all other feature and critical-path rows remain non-credit blockers or planned/deferred non-claims as recorded in `docs/reviews/v0.92/quality-gate-467/feature-completion-matrix.json`.\n"
  quality.write(quality_text)
  readiness.write(readiness_text)
  coverage.write(coverage_text)
end

def write_packet
  OUT_DIR.mkpath
  EVIDENCE_DIR.mkpath
  matrix = build_matrix
  MATRIX_PATH.write(JSON.pretty_generate(matrix) + "\n")
  accepted = matrix["rows"].count { |row| row["disposition"] == "accepted" }
  blocked = matrix["rows"].count { |row| row["disposition"] == "blocked" }
  result = blocked.zero? ? "passed" : "blocked"
  gate = {
    "schema" => "adl.v0.92.quality_gate_record.v2", "issue" => ISSUE,
    "supersedes_issue" => 311, "supersedes_pr" => 466,
    "matrix_sha256" => sha256_path(MATRIX_PATH), "validator_sha256" => sha256_path(Pathname.new(__FILE__)),
    "feature_rows" => 13, "critical_path_rows" => 20,
    "accepted_rows" => accepted, "blocked_rows" => blocked,
    "result" => result, "downstream_unlock" => blocked.zero?,
    "completion_guard" => "reject_uninvestigated_all_blocked",
    "non_claim" => "Corrective #467 hydration grants credit only to accepted rows; remaining concrete blockers keep downstream release work locked."
  }
  GATE_PATH.write(JSON.pretty_generate(gate) + "\n")
  report = ["# v0.92 WP-22A Corrective Blocker Report", "", "Result: **#{result.upcase}**", "", "Accepted rows: #{accepted}. Blocked rows: #{blocked}.", "", "## Accepted Rows", ""]
  matrix["rows"].select { |row| row["disposition"] == "accepted" }.each { |row| report << "- `#{row['id']}` — issue ##{row.dig('evidence', 'issue')} / PR ##{row.dig('evidence', 'pull_request')}: #{row['claim_boundary']}" }
  report.concat(["", "## Concrete Blockers", ""])
  matrix["rows"].select { |row| row["disposition"] == "blocked" }.each { |row| report << "- `#{row['id']}` — #{row['blocker_kind']}: #{row['blockers'].join(', ')}" }
  report.concat(["", "## Downstream", "", "WP-23, WP-25, and release-tail work remain blocked until every row is accepted or explicitly scoped out by milestone authority.", ""])
  REPORT_PATH.write(report.join("\n"))
  SUPERSESSION_PATH.write("# #311 Supersession Note\n\n#311 / PR #466 remain immutable historical provenance for the first structural WP-22 execution. #467 supersedes only the release-credit semantics by hydrating discoverable rows and replacing packet-missing defaults with concrete blocker classifications.\n")
  write_docs_notes(matrix)
  receipt = {
    "schema" => "adl.v0.92.quality_gate_validation_receipt.v2", "issue" => ISSUE,
    "matrix_sha256" => sha256_path(MATRIX_PATH), "gate_sha256" => sha256_path(GATE_PATH),
    "validator_sha256" => sha256_path(Pathname.new(__FILE__)), "blocker_report_sha256" => sha256_path(REPORT_PATH),
    "quality_gate_result" => result, "downstream_unlock" => blocked.zero?,
    "denominator" => { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33, "accepted_rows" => accepted, "blocked_rows" => blocked },
    "accepted_rows" => matrix["rows"].select { |row| row["disposition"] == "accepted" }.map { |row| row["id"] },
    "completion_guard" => "passed"
  }
  (EVIDENCE_DIR / "validation.json").write(JSON.pretty_generate(receipt) + "\n")
  matrix
end

if __FILE__ == $PROGRAM_NAME
  case ARGV.shift || "matrix"
  when "generate"
    matrix = write_packet
    puts JSON.generate(schema: "adl.v0.92.quality_gate_generation.v2", status: "generated", rows: matrix["rows"].length)
  when "matrix"
    matrix, errors = validate_matrix(MATRIX_PATH, canonical: true)
    if errors.empty?
      blocked = matrix["rows"].count { |row| row["disposition"] == "blocked" }
      puts JSON.generate(schema: "adl.v0.92.quality_gate_validation.v2", status: "passed", rows: matrix["rows"].length, blocked_rows: blocked, gate_result: blocked.zero? ? "passed" : "blocked")
    else
      warn JSON.generate(schema: "adl.v0.92.quality_gate_validation.v2", status: "failed", errors: errors)
      exit 1
    end
  else
    warn "usage: validate-quality-gate.rb generate|matrix"
    exit 2
  end
end

#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "digest"
require "time"
ROOT = File.expand_path("../../../..", __dir__)
NOTES = File.join(ROOT, "docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md")
RECEIPT = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-10/ceremony-receipt.json")
REVIEW = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-09/planning-review.json")
GATE = File.join(ROOT, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json")
PREDECESSORS = (516..525).to_a.freeze
BASE = ["bash", "adl/tools/release_ceremony.sh", "--version", "v0.92.1", "--target-branch", "main"].freeze
MUTATIONS = %w[--create-tag --push-tag --draft-release --publish-release].freeze
UNSAFE = %w[--allow-dirty --skip-sor-gate].freeze
AUTH_REL = "docs/milestones/v0.92.1/evidence/release/tail-10/authorization.json"

def sha?(value, length = 40)
  value.to_s.match?(/\A[0-9a-f]{#{length}}\z/)
end

def argv_errors(receipt)
  errors = []
  errors << "noncanonical preflight argv" unless receipt["preflight_argv"] == BASE
  actions = receipt.fetch("mutation_argv", [])
  errors << "mutation argv denominator mismatch" unless actions.length == MUTATIONS.length
  actions.each_with_index do |argv, index|
    errors << "noncanonical mutation argv #{index + 1}" unless argv == BASE + [MUTATIONS[index], "--authorization-file", AUTH_REL]
    errors << "unsafe ceremony flag" unless (Array(argv) & UNSAFE).empty?
  end
  errors << "unsafe preflight flag" unless (Array(receipt["preflight_argv"]) & UNSAFE).empty?
  errors
end

def gate_errors(gate)
  errors = []
  errors << "wrong gate schema" unless gate["schema"] == "adl.v0921.release_ceremony_gate.v1"
  errors << "wrong gate version" unless gate["version"] == "v0.92.1"
  errors << "invalid candidate" unless sha?(gate["candidate_sha"])
  errors << "invalid #525 reviewed revision" unless sha?(gate["planning_review_revision"])
  errors << "gate cannot authorize mutation" unless gate["release_mutation_authorized"] == false
  rows = gate.fetch("predecessors", [])
  errors << "predecessor denominator mismatch" unless rows.map { |row| row["issue"] } == PREDECESSORS
  errors << "predecessor proof incomplete" unless rows.all? { |row| sha?(row["merge_sha"]) && row["review_path"].to_s == ".csdlc/issues/#{row['issue']}/cards/srp.md" && sha?(row["review_sha256"], 64) && row["checks_path"].to_s.start_with?("docs/milestones/v0.92.1/evidence/release/") && sha?(row["checks_sha256"], 64) && row["required_checks"].is_a?(Array) && !row["required_checks"].empty? && row["required_checks"] == row["required_checks"].sort.uniq }
  errors
end

def green_evidence?(bytes, row)
  parsed = JSON.parse(bytes)
  results = parsed["checks"]
  return false unless parsed["schema"] == "adl.github.checks.v1"
  return false unless parsed["issue"] == row["issue"]
  return false unless parsed["head_sha"] == row["merge_sha"]
  return false unless results.is_a?(Array)
  return false unless results.map { |entry| entry["name"] }.sort == row["required_checks"]

  results.all? do |entry|
    entry.is_a?(Hash) && entry["name"].is_a?(String) && entry["conclusion"] == "success"
  end
rescue JSON::ParserError, TypeError
  false
end

def receipt_errors(receipt, notes_digest, review)
  errors = argv_errors(receipt)
  required = %w[candidate_sha tag release_id release_url published_at authorized_by authorized_at preflight_completed_at mutation_started_at tag_target_sha release_target_sha notes_sha256 readback_at planning_review_revision authorization_path authorization_sha256 gate_sha256 preflight_output_path preflight_output_sha256]
  errors << "receipt fields missing" unless required.all? { |key| !receipt[key].to_s.strip.empty? }
  %w[candidate_sha tag_target_sha release_target_sha planning_review_revision].each { |key| errors << "invalid #{key}" unless sha?(receipt[key]) }
  errors << "wrong tag" unless receipt["tag"] == "v0.92.1"
  errors << "tag does not target candidate" unless receipt["tag_target_sha"] == receipt["candidate_sha"]
  errors << "release does not target candidate" unless receipt["release_target_sha"] == receipt["candidate_sha"]
  errors << "notes digest mismatch" unless receipt["notes_sha256"] == notes_digest
  errors << "ceremony tests absent" unless receipt["ceremony_test_status"] == "passed"
  errors << "preflight absent" unless receipt["preflight_status"] == "passed"
  errors << "#525 review revision mismatch" unless receipt["planning_review_revision"] == review["reviewed_revision"]
  errors << "wrong authorization path" unless receipt["authorization_path"] == AUTH_REL
  errors << "invalid bound artifact digest" unless %w[authorization_sha256 gate_sha256 preflight_output_sha256].all? { |key| sha?(receipt[key], 64) }
  begin
    preflight = Time.iso8601(receipt["preflight_completed_at"])
    authorized = Time.iso8601(receipt["authorized_at"])
    mutation = Time.iso8601(receipt["mutation_started_at"])
    errors << "authorization/preflight ordering invalid" unless preflight <= authorized && authorized <= mutation
  rescue ArgumentError, TypeError
    errors << "invalid ceremony timestamps"
  end
  errors
end

if ARGV == ["--negative"]
  review = {"reviewed_revision" => "d" * 40}
  base = {"candidate_sha" => "a" * 40, "tag" => "v0.92.1", "release_id" => "1", "release_url" => "https://example.invalid/1",
          "published_at" => "2026-01-01T00:02:30Z", "authorized_by" => "operator", "authorized_at" => "2026-01-01T00:01:00Z", "preflight_completed_at" => "2026-01-01T00:00:00Z", "mutation_started_at" => "2026-01-01T00:02:00Z",
          "tag_target_sha" => "a" * 40, "release_target_sha" => "a" * 40, "notes_sha256" => "b" * 64, "readback_at" => "2026-01-01T00:03:00Z",
          "planning_review_revision" => "d" * 40, "authorization_path" => AUTH_REL, "authorization_sha256" => "e" * 64, "gate_sha256" => "f" * 64,
          "preflight_output_path" => "docs/milestones/v0.92.1/evidence/release/tail-10/preflight.log", "preflight_output_sha256" => "1" * 64,
          "preflight_argv" => BASE, "mutation_argv" => MUTATIONS.map { |flag| BASE + [flag, "--authorization-file", AUTH_REL] },
          "preflight_status" => "passed", "ceremony_test_status" => "passed"}
  gate = {"schema" => "adl.v0921.release_ceremony_gate.v1", "version" => "v0.92.1", "candidate_sha" => "a" * 40, "planning_review_revision" => "d" * 40,
          "release_mutation_authorized" => false, "predecessors" => PREDECESSORS.map { |issue| {"issue" => issue, "merge_sha" => "c" * 40, "review_path" => ".csdlc/issues/#{issue}/cards/srp.md", "review_sha256" => "e" * 64, "checks_path" => "docs/milestones/v0.92.1/evidence/release/checks-#{issue}.json", "checks_sha256" => "f" * 64, "required_checks" => ["adl-ci"]} }}
  mutations = [base.merge("preflight_argv" => BASE + ["--skip-sor-gate"]), base.merge("mutation_argv" => []),
               base.merge("authorized_at" => "2025-12-31T23:59:00Z"), base.merge("planning_review_revision" => "f" * 40),
               base.merge("tag_target_sha" => "f" * 40), base.merge("notes_sha256" => "f" * 64)]
  gate_mutations = [gate.merge("predecessors" => []), gate.merge("release_mutation_authorized" => true), gate.merge("planning_review_revision" => "main")]
  evidence_row = gate.fetch("predecessors").first
  valid_evidence = {"schema" => "adl.github.checks.v1", "issue" => evidence_row["issue"], "head_sha" => evidence_row["merge_sha"], "checks" => [{"name" => "adl-ci", "conclusion" => "success"}]}
  evidence_mutations = [
    valid_evidence.merge("status" => "success", "checks" => []),
    valid_evidence.merge("issue" => evidence_row["issue"] + 1),
    valid_evidence.merge("head_sha" => "f" * 40),
    valid_evidence.merge("checks" => [{"name" => "adl-ci", "conclusion" => "failure"}]),
    valid_evidence.merge("checks" => [{"name" => "different-check", "conclusion" => "success"}])
  ]
  abort "negative receipt mutation escaped" unless mutations.all? { |row| !receipt_errors(row, "b" * 64, review).empty? }
  abort "negative gate mutation escaped" unless gate_mutations.all? { |row| !gate_errors(row).empty? }
  abort "valid green evidence rejected" unless green_evidence?(JSON.generate(valid_evidence), evidence_row)
  abort "negative green evidence mutation escaped" unless evidence_mutations.none? { |row| green_evidence?(JSON.generate(row), evidence_row) }
  puts "issue 526 negative contract passed (#{mutations.length + gate_mutations.length + evidence_mutations.length} mutations)"
  exit 0
end

mode = ARGV.fetch(0, "receipt")
if mode == "gate"
  path = ARGV[1] || GATE
  abort "missing merge-based ceremony gate" unless File.file?(path)
  gate = JSON.parse(File.read(path))
  errors = gate_errors(gate)
  candidate = gate["candidate_sha"]
  head = IO.popen(["git", "-C", ROOT, "rev-parse", "HEAD"], &:read).strip
  errors << "gate candidate is not exact HEAD" unless candidate == head
  gate.fetch("predecessors", []).each do |row|
    errors << "merge ##{row['issue']} is not ancestral" unless system("git", "-C", ROOT, "merge-base", "--is-ancestor", row["merge_sha"], candidate, out: File::NULL, err: File::NULL)
    review = IO.popen(["git", "-C", ROOT, "show", "#{candidate}:#{row['review_path']}"], err: File::NULL, &:read)
    errors << "review proof mismatch for ##{row['issue']}" unless $CHILD_STATUS.success? && Digest::SHA256.hexdigest(review) == row["review_sha256"] && review.include?("Result: pass")
    checks = IO.popen(["git", "-C", ROOT, "show", "#{candidate}:#{row['checks_path']}"], err: File::NULL, &:read)
    checks_green = green_evidence?(checks, row)
    errors << "green-check proof mismatch for ##{row['issue']}" unless $CHILD_STATUS.success? && Digest::SHA256.hexdigest(checks) == row["checks_sha256"] && checks_green
  end
  errors << "#525 reviewed revision is not ancestral" unless system("git", "-C", ROOT, "merge-base", "--is-ancestor", gate["planning_review_revision"].to_s, candidate, out: File::NULL, err: File::NULL)
  abort errors.join("\n") unless errors.empty?
  puts JSON.generate(schema: "adl.v0921.release_gate_validation.v1", status: "pass", candidate_sha: candidate)
  exit 0
end
abort "invalid mode: #{mode}" unless mode == "receipt"

abort "missing release notes" unless File.file?(NOTES) && !File.zero?(NOTES)
abort "missing current #525 review" unless File.file?(REVIEW) && !File.zero?(REVIEW)
abort "missing ceremony receipt" unless File.file?(RECEIPT) && !File.zero?(RECEIPT)
review = JSON.parse(File.read(REVIEW))
abort "#525 has unresolved release blockers" unless review["outcome"] == "pass" && review["unresolved_release_blockers"] == 0
receipt = JSON.parse(File.read(RECEIPT))
errors = receipt_errors(receipt, Digest::SHA256.file(NOTES).hexdigest, review)
candidate = receipt["candidate_sha"]
authorization_path = File.join(ROOT, receipt["authorization_path"].to_s)
gate_bytes = File.file?(GATE) ? File.binread(GATE) : ""
preflight_path = File.join(ROOT, receipt["preflight_output_path"].to_s)
preflight_bytes = File.file?(preflight_path) ? File.binread(preflight_path) : ""
errors << "authorization artifact digest mismatch" unless File.file?(authorization_path) && Digest::SHA256.file(authorization_path).hexdigest == receipt["authorization_sha256"]
errors << "gate artifact digest mismatch" unless !gate_bytes.empty? && Digest::SHA256.hexdigest(gate_bytes) == receipt["gate_sha256"]
errors << "preflight output missing or digest mismatch" unless !preflight_bytes.empty? && Digest::SHA256.hexdigest(preflight_bytes) == receipt["preflight_output_sha256"]
if File.file?(authorization_path)
  begin
    authorization = JSON.parse(File.read(authorization_path))
    errors << "authorization artifact identity mismatch" unless authorization["schema"] == "adl.release_authorization.v1" && authorization["version"] == "v0.92.1" && authorization["tag"] == "v0.92.1" && authorization["candidate_sha"] == candidate && authorization["authorized_by"] == receipt["authorized_by"] && authorization["authorized_at"] == receipt["authorized_at"] && (MUTATIONS.map { |flag| flag.delete_prefix("--").tr("-", "_") } - Array(authorization["allowed_actions"])).empty?
  rescue JSON::ParserError
    errors << "authorization artifact is not JSON"
  end
end
errors << "#525 reviewed revision is not ancestral to candidate" unless system("git", "-C", ROOT, "merge-base", "--is-ancestor", review["reviewed_revision"].to_s, candidate.to_s, out: File::NULL, err: File::NULL)
remote = IO.popen(["git", "-C", ROOT, "ls-remote", "origin", "refs/tags/v0.92.1", "refs/tags/v0.92.1^{}"], err: File::NULL, &:read)
remote_target = remote.lines.find { |line| line.include?("^{}") }&.split&.first || remote.lines.first&.split&.first
errors << "live remote tag mismatch" unless remote_target == candidate
release_json = IO.popen(["gh", "release", "view", "v0.92.1", "--repo", "agent-logic/agent-design-language", "--json", "tagName,targetCommitish,url,isDraft,databaseId,publishedAt,body"], err: File::NULL, &:read)
begin
  live_release = JSON.parse(release_json)
  errors << "live release tag mismatch" unless live_release["tagName"] == "v0.92.1"
  errors << "live release target mismatch" unless [candidate, "main"].include?(live_release["targetCommitish"])
  errors << "live release remains draft" unless live_release["isDraft"] == false
  errors << "live release URL mismatch" unless live_release["url"] == receipt["release_url"]
  errors << "live release ID mismatch" unless live_release["databaseId"].to_s == receipt["release_id"].to_s
  errors << "live release publication time mismatch" unless live_release["publishedAt"] == receipt["published_at"]
  errors << "live release notes mismatch" unless Digest::SHA256.hexdigest(live_release["body"].to_s) == receipt["notes_sha256"]
rescue JSON::ParserError
  errors << "live release readback unavailable"
end
abort errors.join("\n") unless errors.empty?
puts "issue 526 live ceremony readback passed"

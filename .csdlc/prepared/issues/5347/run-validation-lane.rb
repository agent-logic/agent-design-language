#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"
require "pathname"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-external-bands")
EXTERNAL_MANIFEST = File.join(EVIDENCE, "external-band-deletion-manifest.json")
CORE_MANIFEST = File.join(EVIDENCE, "../wp13-core/final-core-deletion-manifest.json")
INDEX = File.join(ROOT, ".csdlc/issues/5347/index.json")
REPOSITORY = "danielbaustin/agent-design-language"
ALLOWED_DISPOSITIONS = %w[delete_external retain_owned retain_evidence handoff_to_5346 blocked].freeze
FORBIDDEN_COMPONENTS = %w[target build dist node_modules .git].freeze
PROOF_EXECUTABLES = %w[ruby bash cargo git].freeze
FIXED_CLAIM_PATHS = [
  ".csdlc/issues/5347",
  ".csdlc/locks/5347.lock",
  ".csdlc/prepared/issues/5347",
  ".csdlc/evidence/5347",
  "docs/milestones/v0.91.8/evidence/wp13-external-bands"
].freeze
RECEIPT_VERIFIER = File.join(__dir__, "verify-terminal-receipt.rb")
GATE_PATHS = [
  ".csdlc/prepared/issues/5347/check-dependencies.rb",
  ".csdlc/prepared/issues/5347/run-validation-lane.rb",
  ".csdlc/prepared/issues/5347/validate-blocked-state.rb",
  ".csdlc/prepared/issues/5347/verify-terminal-receipt.rb",
  ".csdlc/prepared/issues/5347/receipt-verifier/Cargo.toml",
  ".csdlc/prepared/issues/5347/receipt-verifier/src/main.rs",
  "docs/milestones/v0.91.8/evidence/wp13-external-bands/external-band-deletion-manifest.json"
].freeze

def fail!(message)
  warn("#5347 validation blocked: #{message}")
  exit(1)
end

def load_json(path)
  fail!("missing #{path.sub(ROOT + '/', '')}") unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  fail!("invalid JSON #{path}: #{error.message}")
end

def canonical_json(value)
  case value
  when Hash
    "{" + value.keys.sort.map { |key| JSON.generate(key) + ":" + canonical_json(value.fetch(key)) }.join(",") + "}"
  when Array
    "[" + value.map { |item| canonical_json(item) }.join(",") + "]"
  else
    JSON.generate(value)
  end
end

def verify_manifest_digest!(manifest)
  supplied = manifest.fetch("manifest_sha256")
  content = manifest.reject { |key, _value| key == "manifest_sha256" }
  actual = Digest::SHA256.hexdigest(canonical_json(content))
  fail!("manifest digest mismatch") unless supplied == actual
end

def git!(*argv)
  out, err, status = Open3.capture3("git", "-C", ROOT, *argv)
  fail!("git #{argv.join(' ')} failed: #{err.lines.first}") unless status.success?
  out
end

def canonical_paths(manifest, require_present:, require_head:, allow_deleted: false)
  fail!("manifest schema mismatch") unless manifest["schema"] == "adl.wp13.deletion_manifest.v1"
  fail!("manifest repository mismatch") unless manifest["repository"] == REPOSITORY
  fail!("manifest candidate revision malformed") unless manifest["candidate_revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
  if require_head
    fail!("manifest revision is not HEAD") unless manifest["candidate_revision"] == git!("rev-parse", "HEAD").strip
  end
  verify_manifest_digest!(manifest)
  rows = manifest.fetch("rows")
  paths = rows.map { |row| row.fetch("path") }
  fail!("manifest is not canonically sorted") unless paths == paths.sort
  paths.each do |path|
    fail!("non-relative path #{path}") if path.start_with?("/")
    fail!("escaping path #{path}") if path.split("/").include?("..")
    fail!("non-canonical path #{path}") unless Pathname.new(path).cleanpath.to_s == path
    fail!("forbidden generated/build component in #{path}") unless (path.split("/") & FORBIDDEN_COMPONENTS).empty?
    fail!("Runtime v2 path is outside #5347 authority: #{path}") if path.split("/").include?("runtime_v2")
    row = rows.find { |candidate| candidate.fetch("path") == path }
    fail!("#{path} must declare regular_file") unless row["file_kind"] == "regular_file"
    fail!("#{path} must explicitly be non-generated") unless row["generated"] == false
    if allow_deleted && row["disposition"] == "delete_external"
      fail!("deleted candidate still exists: #{path}") if File.exist?(File.join(ROOT, path))
      fail!("#{path} baseline object is not retained as a blob") unless git!("cat-file", "-t", row.fetch("baseline_object")).strip == "blob"
      next
    end
    next unless require_present

    tracked = git!("ls-files", "--stage", "--", path).lines.map(&:strip).reject(&:empty?)
    fail!("candidate is not one exact tracked file: #{path}") unless tracked.length == 1 && tracked.first.split("\t", 2).last == path
    mode = tracked.first.split.first
    fail!("submodule candidate #{path}") if mode == "160000"
    cursor = ROOT
    path.split("/").each do |component|
      cursor = File.join(cursor, component)
      fail!("symlink candidate or ancestor #{path}") if File.symlink?(cursor)
    end
    candidate = File.join(ROOT, path)
    fail!("candidate is not a regular file: #{path}") unless File.file?(candidate)
    real = File.realpath(candidate)
    fail!("candidate escapes repository root #{path}") unless real.start_with?(ROOT + File::SEPARATOR)
  end
  fail!("duplicate canonical path") unless paths.uniq.length == paths.length
  paths
end

def run_argv(argv)
  fail!("invalid proof command") unless argv.is_a?(Array) && argv.all? { |item| item.is_a?(String) && !item.empty? }
  fail!("proof executable not allowlisted") unless PROOF_EXECUTABLES.include?(argv.first)
  fail!("proof command contains network or credential token") if argv.any? { |item| item.match?(/https?:|aws|gh|curl|wget|token|credential|secret/i) }
  _out, err, status = Open3.capture3(*argv, chdir: ROOT)
  fail!("proof command failed: #{err.lines.first}") unless status.success?
end

def nonblank_lines(paths)
  paths.sum do |relative|
    path = File.join(ROOT, relative)
    fail!("accounted path missing: #{relative}") unless File.file?(path)
    File.readlines(path).count { |line| !line.strip.empty? }
  end
end

def verify_evidence_ref!(label, reference)
  fail!("#{label} evidence ref malformed") unless reference.is_a?(Hash) && reference.keys.sort == %w[path sha256]
  relative = reference.fetch("path")
  fail!("#{label} evidence path escapes") if relative.start_with?("/") || relative.split("/").include?("..")
  path = File.join(ROOT, relative)
  fail!("#{label} evidence missing") unless File.file?(path)
  fail!("#{label} evidence digest malformed") unless reference["sha256"].match?(/\A[0-9a-f]{64}\z/)
  fail!("#{label} evidence digest mismatch") unless Digest::SHA256.file(path).hexdigest == reference["sha256"]
end

lane = ARGV.fetch(0) { fail!("lane required") }

case lane
when "manifest-disjointness"
  external = load_json(EXTERNAL_MANIFEST)
  core = load_json(CORE_MANIFEST)
  post_deletion = ARGV[1] == "post"
  external_paths = canonical_paths(external, require_present: true, require_head: true, allow_deleted: post_deletion)
  core_paths = canonical_paths(core, require_present: false, require_head: false)
  overlap = external_paths & core_paths
  fail!("manifest overlap: #{overlap.join(', ')}") unless overlap.empty?
  claim_paths = load_json(INDEX).fetch("claim").fetch("protected_paths")
  deletion_paths = external.fetch("rows").select { |row| row["disposition"] == "delete_external" }.map { |row| row.fetch("path") }
  fail!("typed claim has extra or missing product authority") unless claim_paths.sort == (FIXED_CLAIM_PATHS + deletion_paths).uniq.sort
  external.fetch("rows").each do |row|
    disposition = row.fetch("disposition")
    fail!("unknown disposition #{disposition}") unless ALLOWED_DISPOSITIONS.include?(disposition)
    if disposition == "delete_external"
      %w[baseline_object capability replacement_owner replacement_issue replacement_revision terminal_receipt proof_refs reachability_evidence claim_id acceptance_status].each do |key|
        fail!("#{row['path']} missing #{key}") if row[key].nil? || row[key] == "" || row[key] == []
      end
      fail!("#{row['path']} replacement revision malformed") unless row["replacement_revision"].match?(/\A[0-9a-f]{40}\z/)
      fail!("#{row['path']} uses forbidden replacement owner") if row["replacement_owner"].match?(/incumbent|runtime.?v2/i)
      fail!("#{row['path']} replacement is not independently accepted") unless row["acceptance_status"] == "accepted"
      receipt = "csdlc-v2/closeout/#{Integer(row['replacement_issue'])}.json"
      fail!("#{row['path']} terminal receipt ref mismatch") unless row["terminal_receipt"] == receipt
      _out, err, status = Open3.capture3("ruby", RECEIPT_VERIFIER, row["replacement_issue"].to_s, row["replacement_revision"], chdir: ROOT)
      fail!("#{row['path']} replacement receipt invalid: #{err.lines.first}") unless status.success?
      _out, ancestry = Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", row["replacement_revision"], "HEAD")
      fail!("#{row['path']} replacement revision is not ancestral") unless ancestry.success?
      row.fetch("proof_refs").each { |reference| verify_evidence_ref!("#{row['path']} proof", reference) }
      verify_evidence_ref!("#{row['path']} reachability", row.fetch("reachability_evidence"))
      fail!("#{row['path']} lacks exact typed claim coverage") unless claim_paths.include?(row["path"])
      fail!("#{row['path']} claim id mismatch") unless row["claim_id"] == load_json(INDEX).fetch("claim").fetch("id")
    elsif disposition.start_with?("retain")
      %w[owner consumer rationale proof_role sunset_condition].each do |key|
        fail!("#{row['path']} retained row missing #{key}") if row[key].nil? || row[key] == "" || row[key] == []
      end
    end
  end
when "owner-and-consumer-proof"
  plan = load_json(File.join(EVIDENCE, "owner-and-consumer-proof-plan.json"))
  fail!("proof plan schema mismatch") unless plan["schema"] == "adl.wp13.owner_consumer_proof.v1"
  fail!("proof plan revision mismatch") unless plan["revision"] == git!("rev-parse", "HEAD").strip
  expected = %w[accepted-owner characterization-parity security-determinism selector-rollback authoritative-consumers]
  fail!("proof roles incomplete") unless plan.fetch("proofs").map { |proof| proof.fetch("id") }.sort == expected.sort
  fail!("proof plan permits network") unless plan.fetch("network") == "denied"
  fail!("proof plan permits credentials") unless plan.fetch("credentials") == []
  validation_request = plan.fetch("validation_request")
  fail!("proof validation request escapes") if validation_request.start_with?("/") || validation_request.split("/").include?("..")
  primary = File.dirname(File.expand_path(git!("rev-parse", "--git-common-dir").strip, ROOT))
  validator = File.join(primary, ".adl/bin/csdlc-v2/csdlc-validate")
  fail!("stable typed PVF validator missing") unless File.executable?(validator)
  _pvf_out, pvf_err, pvf_status = Open3.capture3(validator, "--root", ROOT, "--request", validation_request, chdir: ROOT)
  fail!("typed PVF owner proof failed: #{pvf_err.lines.first}") unless pvf_status.success?
  results = load_json(File.join(EVIDENCE, "owner-and-consumer-proof-results.json"))
  fail!("proof results schema mismatch") unless results["schema"] == "adl.wp13.owner_consumer_results.v1"
  fail!("proof results revision mismatch") unless results["revision"] == git!("rev-parse", "HEAD").strip
  by_id = results.fetch("results").to_h { |result| [result.fetch("id"), result] }
  plan.fetch("proofs").each do |proof|
    fail!("proof command must be structured argv") unless proof["argv"].is_a?(Array)
    fail!("proof executable not allowlisted") unless PROOF_EXECUTABLES.include?(proof["argv"].first)
    fail!("proof is not bound to a required PVF lane") if proof["pvf_lane_id"].to_s.empty?
    result = by_id.fetch(proof.fetch("id")) { fail!("missing result for #{proof['id']}") }
    fail!("#{proof['id']} did not pass") unless result["status"] == "pass" && result["exit_code"] == 0
    fail!("#{proof['id']} was deferred or skipped") if %w[deferred skipped not_run].include?(result["status"])
    fail!("#{proof['id']} exceeded timeout") unless result.fetch("elapsed_seconds") <= proof.fetch("timeout_seconds")
    fail!("#{proof['id']} network posture mismatch") unless result["network"] == "denied"
    fail!("#{proof['id']} credential posture mismatch") unless result["credentials"] == []
    fail!("#{proof['id']} command drift") unless result["argv"] == proof["argv"]
    fail!("#{proof['id']} evidence digest malformed") unless result["evidence_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
    evidence_path = result.fetch("evidence_path")
    fail!("#{proof['id']} evidence path escapes") if evidence_path.start_with?("/") || evidence_path.split("/").include?("..")
    evidence_file = File.join(ROOT, evidence_path)
    fail!("#{proof['id']} PVF evidence missing") unless File.file?(evidence_file)
    fail!("#{proof['id']} PVF evidence digest mismatch") unless Digest::SHA256.file(evidence_file).hexdigest == result["evidence_sha256"]
    evidence = load_json(evidence_file)
    fail!("#{proof['id']} was not executed by typed PVF") unless evidence["schema"] == "csdlc.pvf.lane_evidence.v1" && evidence["lane"] == proof["pvf_lane_id"]
    fail!("#{proof['id']} PVF result is not pass") unless evidence["status"] == "pass" && evidence["exit_code"] == 0
    fail!("#{proof['id']} PVF network isolation not enforced") unless evidence["network"] == "denied" && evidence["network_enforced"] == true
    fail!("#{proof['id']} PVF exposed credentials") unless evidence["credentials"] == []
    fail!("#{proof['id']} PVF command drift") unless evidence["argv"] == proof["argv"]
    fail!("#{proof['id']} PVF revision drift") unless evidence["revision"] == results["revision"]
  end
when "deletion-budgets-and-evidence"
  report = load_json(File.join(EVIDENCE, "deletion-accounting.json"))
  manifest = load_json(EXTERNAL_MANIFEST)
  fail!("accounting schema mismatch") unless report["schema"] == "adl.wp13.deletion_accounting.v1"
  fail!("accounting revision mismatch") unless report["revision"] == git!("rev-parse", "HEAD").strip
  deletion_rows = manifest.fetch("rows").select { |row| row["disposition"] == "delete_external" }
  deleted = deletion_rows.sum do |row|
    object = row.fetch("baseline_object")
    fail!("#{row['path']} baseline object is not a blob") unless git!("cat-file", "-t", object).strip == "blob"
    blob = git!("cat-file", "blob", object)
    actual = blob.lines.count
    fail!("#{row['path']} measured line count drift") unless row["measured_lines"] == actual
    actual
  end
  replacement = nonblank_lines(report.fetch("replacement_source_paths"))
  fail!("manifest/gate path inventory is incomplete") unless report.fetch("manifest_gate_paths").sort == GATE_PATHS.sort
  gate_lines = nonblank_lines(report.fetch("manifest_gate_paths"))
  test_lines = nonblank_lines(report.fetch("test_fixture_paths"))
  tests = report.fetch("test_inventory")
  fail!("duplicate test inventory") unless tests.map { |test| test.values_at("path", "id") }.uniq.length == tests.length
  fail!("test inventory references unaccounted paths") unless tests.all? { |test| report.fetch("test_fixture_paths").include?(test["path"]) }
  fail!("deleted source accounting mismatch") unless report["deleted_source_lines"] == deleted
  fail!("replacement source accounting mismatch") unless report["replacement_source_lines"] == replacement
  fail!("manifest/gate accounting mismatch") unless report["manifest_gate_nonblank_lines"] == gate_lines
  fail!("test/fixture accounting mismatch") unless report["test_fixture_nonblank_lines"] == test_lines
  fail!("test count accounting mismatch") unless report["test_count"] == tests.length
  added = replacement + gate_lines
  fail!("net source accounting mismatch") unless report.fetch("net_source_lines") == added - deleted
  fail!("source change is not net negative") unless added - deleted < 0
  fail!("manifest/gate budget exceeded") unless gate_lines <= 500
  fail!("test/fixture budget exceeded") unless test_lines <= 800
  fail!("test count exceeded") unless tests.length < 50
  evidence_lines = Dir.glob(File.join(EVIDENCE, "**/*")).select { |path| File.file?(path) }.sum do |path|
    File.readlines(path).count { |line| !line.strip.empty? }
  end
  fail!("evidence accounting mismatch") unless report.fetch("evidence_nonblank_lines") == evidence_lines
  fail!("evidence budget exceeded") unless evidence_lines <= 1200
when "validate-contracts"
  request = load_json(File.join(__dir__, "bootstrap-request.json"))
  lanes = request.fetch("initial").fetch("validation_lanes")
  expected = %w[preparation-contract dependency-terminal-gate manifest-disjointness owner-and-consumer-proof deletion-budgets-and-evidence post-deletion-exact]
  fail!("future lane set incomplete") unless lanes.map { |entry| entry.fetch("lane") }.sort == expected.sort
  fail!("acceptance coverage incomplete") unless lanes.flat_map { |entry| entry.fetch("acceptance_ids") }.uniq.sort == (1..8).map { |id| "AC-#{id}" }
  lanes.reject { |entry| entry["lane"] == "preparation-contract" }.each do |entry|
    fail!("#{entry['lane']} is not deterministic") unless entry["deterministic"] == true
    fail!("#{entry['lane']} has no executable timeout") unless entry["budget_seconds"].is_a?(Integer) && entry["budget_seconds"].positive?
    fail!("#{entry['lane']} has no structured command") unless entry["argv"].is_a?(Array) && PROOF_EXECUTABLES.include?(entry["argv"].first)
    fail!("#{entry['lane']} is not a mandatory gate") unless entry["defer_reason"].to_s.match?(/Mandatory|expected to fail|mandatory before/)
  end
when "post-deletion-exact"
  script = File.join(__dir__, "check-dependencies.rb")
  run_argv(["ruby", script])
  run_argv(["ruby", __FILE__, "manifest-disjointness", "post"])
  %w[owner-and-consumer-proof deletion-budgets-and-evidence].each { |child| run_argv(["ruby", __FILE__, child]) }
else
  fail!("unknown lane #{lane}")
end

puts(JSON.generate({schema: "adl.wp13.external_band_validation.v1", issue: 5347, lane: lane, status: "pass"}))

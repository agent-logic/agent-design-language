#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "tmpdir"

ISSUE = 217
EVIDENCE_ROOT = ".csdlc/evidence/217"
PLATFORM_ROOT = "#{EVIDENCE_ROOT}/native-platform"
DENOMINATOR = "#{EVIDENCE_ROOT}/retained-proof-denominator.json"
RETAINED_MANIFEST = "#{EVIDENCE_ROOT}/h2-retained-surface-manifest.json"
REVIEW_RECEIPT = "#{EVIDENCE_ROOT}/h2-retention-review-receipt.json"
SOURCE_DENOMINATOR = ".csdlc/prepared/issues/217/protected-source-denominator.json"
PROOF_DENOMINATOR = ".csdlc/prepared/issues/217/proof-contract-paths.json"
ALLOWLIST = ".csdlc/prepared/issues/217/h2-retention-allowlist.json"
WORKFLOW = ".github/workflows/wp14-retained-native-proof.yml"
TESTS = %w[canonical_ingress_applies_bounded_pressure production_binary_acip_wss_produces_observed_receipt].freeze
ASSERTIONS = %w[
  acip_write_token_authenticated
  binary_protobuf_dispatch_completed
  exact_production_binary_tls_ready
  observatory_read_token_rejected
  production_wss_pressure_rolls_back_and_recovers
  replay_rejected
  signed_graceful_shutdown
  terminal_sequence_rejected_without_cross_domain_poisoning
  text_frame_rejected
  typed_ingress_error_rolls_back_sequence
].freeze
TEN_PATHS = %w[
  .csdlc/evidence/217/native-platform/linux-nextest.log
  .csdlc/evidence/217/native-platform/linux-semantic.json
  .csdlc/evidence/217/native-platform/linux-source-manifest.json
  .csdlc/evidence/217/native-platform/linux.json
  .csdlc/evidence/217/native-platform/macos-nextest.log
  .csdlc/evidence/217/native-platform/macos-semantic.json
  .csdlc/evidence/217/native-platform/macos-source-manifest.json
  .csdlc/evidence/217/native-platform/macos.json
  .csdlc/evidence/217/native-receipts-validation.log
  .csdlc/evidence/217/native-validation-manifest.json
].freeze

def fail!(message)
  raise message
end

def canonical_json(value)
  case value
  when Hash then "{" + value.keys.sort.map { |key| "#{JSON.generate(key)}:#{canonical_json(value.fetch(key))}" }.join(",") + "}"
  when Array then "[" + value.map { |entry| canonical_json(entry) }.join(",") + "]"
  else JSON.generate(value)
  end
end

def sha(path)
  Digest::SHA256.file(path).hexdigest
end

def git(root, *argv, allow_failure: false)
  stdout, stderr, status = Open3.capture3("git", *argv, chdir: root.to_s)
  fail!("git #{argv.join(' ')} failed: #{stderr.strip}") unless status.success? || allow_failure
  [stdout, status.success?]
end

def confined(root, relative, prefix)
  fail!("absolute evidence path: #{relative}") if Pathname.new(relative).absolute?
  fail!("non-canonical evidence path: #{relative}") if relative.split(/[\\\/]/).include?("..")
  path = root.join(relative).cleanpath
  fail!("path escapes #{prefix}: #{relative}") unless path.to_s.start_with?("#{root.join(prefix)}/")
  path
end

def exact_denominator(root, relative, expected_count, prefix)
  path = confined(root, relative, EVIDENCE_ROOT)
  document = JSON.parse(path.read)
  files = document.fetch("files")
  paths = files.map { |entry| entry.fetch("path") }
  fail!("denominator count mismatch") unless document["expected_file_count"] == expected_count && files.length == expected_count
  fail!("denominator path duplication") unless paths.uniq.length == expected_count
  files.each do |entry|
    artifact = confined(root, entry.fetch("path"), prefix)
    fail!("artifact missing: #{entry.fetch('path')}") unless artifact.file?
    fail!("artifact digest mismatch: #{entry.fetch('path')}") unless sha(artifact) == entry.fetch("sha256")
  end
  [document, files]
end

def contract(root)
  source = JSON.parse(root.join(SOURCE_DENOMINATOR).read)
  proof = JSON.parse(root.join(PROOF_DENOMINATOR).read)
  allowlist = JSON.parse(root.join(ALLOWLIST).read)
  fail!("source denominator mismatch") unless source["expected_path_count"] == 17 && source["paths"].length == 17 && source["paths"].uniq.length == 17
  fail!("proof denominator mismatch") unless proof["expected_path_count"] == 8 && proof["paths"].length == 8 && proof["paths"].uniq.length == 8
  fail!("allowlist proof mismatch") unless allowlist["proof_paths"] == proof["paths"]
  fail!("allowlist lifecycle mismatch") unless allowlist["expected_lifecycle_path_count"] == 14 && allowlist["lifecycle_paths"].length == 14 && allowlist["lifecycle_paths"].uniq.length == 14
  fail!("retained surface count mismatch") unless allowlist["expected_retained_surface_entry_count"] == 19
  [source, proof, allowlist]
end

def unsafe_log?(text, root)
  text.include?(root.to_s) || [%r{/(?:users?|home|private)/}i, %r{[a-z]:[\\/]}, %r{\\\\[^\\/\s]+[\\/]}, %r{/volumes/(?:fastwork|home)/}i, %r{/var/folders/}i].any? { |pattern| text.match?(pattern) }
end

def validate_packet(root, receipt_paths, require_head: nil)
  source_contract, proof_contract, = contract(root)
  source_paths = source_contract.fetch("paths")
  proof_paths = proof_contract.fetch("paths")
  expected_names = TESTS.map { |name| "adl-runtime-kernel::production_acip_wss$#{name}" }.sort
  expected_argv = ["cargo", "nextest", "run", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--test", "production_acip_wss", "--no-tests=fail", "--status-level", "all", "--message-format", "libtest-json-plus"]
  expected_source = source_paths.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } }
  expected_proof = proof_paths.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } }
  payloads = receipt_paths.map do |relative|
    path = confined(root, relative, PLATFORM_ROOT)
    packet = JSON.parse(path.read)
    payload = packet.fetch("payload")
    fail!("receipt schema mismatch") unless packet["schema"] == "adl.native_ci_receipt.v1"
    fail!("payload digest mismatch") unless packet["payload_sha256"] == Digest::SHA256.hexdigest(canonical_json(payload))
    fail!("receipt source mismatch") unless require_head.nil? || payload["source_sha"] == require_head
    fail!("source denominator binding mismatch") unless payload["source_denominator_path"] == SOURCE_DENOMINATOR && payload["source_denominator_sha256"] == sha(root.join(SOURCE_DENOMINATOR))
    fail!("proof denominator binding mismatch") unless payload["proof_denominator_path"] == PROOF_DENOMINATOR && payload["proof_denominator_sha256"] == sha(root.join(PROOF_DENOMINATOR))
    fail!("proof manifest mismatch") unless payload["proof_manifest"] == expected_proof
    fail!("producer command drift") unless payload["test_argv"] == expected_argv
    fail!("test inventory mismatch") unless payload["passed_tests"] == expected_names && payload["tests_run"] == TESTS.length
    runner = payload.fetch("runner")
    fail!("runner provenance mismatch") unless runner["provider"] == "github_actions" && runner["repository"] == "agent-logic/agent-design-language" && runner["workflow_ref"].to_s.start_with?("agent-logic/agent-design-language/#{WORKFLOW}@")
    %w[command_output semantic_output source_manifest].each do |kind|
      artifact = confined(root, payload.fetch("#{kind}_path"), PLATFORM_ROOT)
      fail!("#{kind} digest mismatch") unless payload["#{kind}_sha256"] == sha(artifact)
    end
    log = confined(root, payload.fetch("command_output_path"), PLATFORM_ROOT).read
    fail!("machine-local log") if unsafe_log?(log, root)
    parsed_source = JSON.parse(confined(root, payload.fetch("source_manifest_path"), PLATFORM_ROOT).read)
    fail!("source manifest mismatch") unless parsed_source == expected_source
    semantic = JSON.parse(confined(root, payload.fetch("semantic_output_path"), PLATFORM_ROOT).read)
    fail!("semantic schema mismatch") unless semantic["schema"] == "adl.acip_native_platform_proof.v2"
    fail!("semantic platform mismatch") unless semantic["platform"] == payload["platform"]
    assertions = Array(semantic["assertions"])
    fail!("assertion inventory mismatch") unless assertions.map { |entry| entry["name"] }.sort == ASSERTIONS.sort
    fail!("semantic assertion failure") unless assertions.all? { |entry| entry["result"] == "passed" }
    projection = semantic.reject { |key, _value| key == "platform" }
    fail!("semantic projection mismatch") unless payload["semantic_projection_sha256"] == Digest::SHA256.hexdigest(canonical_json(projection))
    payload
  end
  fail!("platform denominator mismatch") unless payloads.map { |item| item["platform"] }.sort == %w[linux macos]
  fail!("source revision mismatch") unless payloads.map { |item| item["source_sha"] }.uniq.one?
  fail!("run mismatch") unless payloads.map { |item| item.dig("runner", "run_id") }.uniq.one?
  fail!("semantic mismatch") unless payloads.map { |item| item["semantic_projection_sha256"] }.uniq.one?
  payloads
end

def write_aggregate(root, receipt_paths)
  head = git(root, "rev-parse", "HEAD").first.strip
  payloads = validate_packet(root, receipt_paths, require_head: head)
  evidence = root.join(EVIDENCE_ROOT)
  FileUtils.mkdir_p(evidence)
  validation_log = evidence.join("native-receipts-validation.log")
  validation_manifest = evidence.join("native-validation-manifest.json")
  validation_log.write(JSON.generate(status: "passed", source_sha: head, platforms: %w[linux macos]) + "\n")
  manifest_payload = {
    "issue" => ISSUE,
    "source_sha" => head,
    "workflow" => WORKFLOW,
    "run_id" => payloads.first.dig("runner", "run_id"),
    "run_attempt" => payloads.first.dig("runner", "run_attempt"),
    "receipts" => receipt_paths.sort.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } },
    "proof_manifest" => payloads.first.fetch("proof_manifest"),
    "status" => "passed"
  }
  validation_manifest.write(JSON.pretty_generate({ "schema" => "adl.native_validation_manifest.v2", "payload" => manifest_payload, "payload_sha256" => Digest::SHA256.hexdigest(canonical_json(manifest_payload)) }) + "\n")
  denominator = {
    "schema" => "adl.retained_proof_denominator.v1", "issue" => ISSUE, "source_revision" => head,
    "expected_file_count" => 10, "files" => TEN_PATHS.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } }
  }
  root.join(DENOMINATOR).write(JSON.pretty_generate(denominator) + "\n")
  _, proof, allowlist = contract(root)
  retained_paths = [DENOMINATOR] + TEN_PATHS + proof.fetch("paths")
  fail!("retained path count mismatch") unless retained_paths.length == 19 && retained_paths.uniq.length == 19
  retained = {
    "schema" => allowlist.fetch("retained_surface_manifest_schema"), "issue" => ISSUE, "producer_head" => head,
    "expected_entry_count" => 19, "entries" => retained_paths.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } }
  }
  root.join(RETAINED_MANIFEST).write(JSON.pretty_generate(retained) + "\n")
  puts JSON.generate(status: "passed", source_sha: head, denominator: DENOMINATOR, retained_manifest: RETAINED_MANIFEST)
end

def receipt_anchor(root, receipt, expected_h2)
  path = REVIEW_RECEIPT
  commits = git(root, "log", "--format=%H", "--diff-filter=A", "--", path).first.lines.map(&:strip).reject(&:empty?)
  candidates = commits.select do |commit|
    parents = git(root, "show", "-s", "--format=%P", commit).first.split
    parents.all? do |parent|
      _commit_output, parent_available = git(root, "cat-file", "-e", "#{parent}^{commit}", allow_failure: true)
      next parent == expected_h2 unless parent_available
      _output, exists = git(root, "cat-file", "-e", "#{parent}:#{path}", allow_failure: true)
      !exists
    end
  end
  fail!("receipt anchor missing or ambiguous") unless candidates.length == 1
  anchor = candidates.first
  anchored, ok = git(root, "show", "#{anchor}:#{path}", allow_failure: true)
  fail!("receipt anchor blob missing") unless ok
  current = root.join(path).read
  fail!("current receipt differs from ancestral anchor") unless current == anchored
  blob = git(root, "rev-parse", "#{anchor}:#{path}").first.strip
  fail!("receipt blob identity mismatch") unless blob == git(root, "hash-object", path).first.strip
  { "commit" => anchor, "blob" => blob, "sha256" => Digest::SHA256.hexdigest(current) }
end

def validate_h2_diff(root, h, h2, evidence_paths, allowlist)
  output, available = git(root, "diff", "--name-status", "--find-renames", "--find-copies", h, h2, allow_failure: true)
  return nil unless available
  rows = output.lines.map(&:strip).reject(&:empty?).map { |line| line.split("\t") }
  fail!("H-to-H2 duplicate changed path") unless rows.flat_map { |row| row.drop(1) }.uniq.length == rows.flat_map { |row| row.drop(1) }.length
  fail!("H-to-H2 forbidden status") unless rows.all? { |row| %w[A M].include?(row.fetch(0)) && row.length == 2 }
  required = [DENOMINATOR, RETAINED_MANIFEST] + evidence_paths
  allowed = required + allowlist.fetch("lifecycle_paths")
  changed = rows.map { |row| row.fetch(1) }
  fail!("H-to-H2 required evidence missing") unless (required - changed).empty?
  fail!("H-to-H2 unexpected path: #{(changed - allowed).join(', ')}") unless (changed - allowed).empty?
  Digest::SHA256.hexdigest(output)
end

def validate_retained(root, denominator_path)
  denominator, files = exact_denominator(root, denominator_path, 10, EVIDENCE_ROOT)
  fail!("denominator exact path set mismatch") unless files.map { |entry| entry.fetch("path") } == TEN_PATHS
  receipt_paths = files.map { |entry| entry["path"] }.grep(%r{/native-platform/(?:linux|macos)\.json\z})
  payloads = validate_packet(root, receipt_paths)
  source_sha = payloads.first.fetch("source_sha")
  fail!("denominator source mismatch") unless denominator["source_revision"] == source_sha
  validation_packet = JSON.parse(root.join("#{EVIDENCE_ROOT}/native-validation-manifest.json").read)
  validation_payload = validation_packet.fetch("payload")
  fail!("validation manifest schema mismatch") unless validation_packet["schema"] == "adl.native_validation_manifest.v2"
  fail!("validation manifest payload mismatch") unless validation_packet["payload_sha256"] == Digest::SHA256.hexdigest(canonical_json(validation_payload))
  fail!("validation manifest source mismatch") unless validation_payload["source_sha"] == source_sha && validation_payload["status"] == "passed" && validation_payload["workflow"] == WORKFLOW
  expected_receipts = receipt_paths.sort.map { |relative| { "path" => relative, "sha256" => sha(root.join(relative)) } }
  fail!("validation manifest receipts mismatch") unless validation_payload["receipts"] == expected_receipts
  fail!("validation manifest proof mismatch") unless validation_payload["proof_manifest"] == payloads.first.fetch("proof_manifest")
  validation_log = root.join("#{EVIDENCE_ROOT}/native-receipts-validation.log").read
  fail!("validation log machine-local") if unsafe_log?(validation_log, root)
  retained = JSON.parse(root.join(RETAINED_MANIFEST).read)
  entries = retained.fetch("entries")
  fail!("retained manifest count mismatch") unless retained["expected_entry_count"] == 19 && entries.length == 19 && entries.map { |entry| entry["path"] }.uniq.length == 19
  _, proof_contract, = contract(root)
  expected_retained_paths = [DENOMINATOR] + TEN_PATHS + proof_contract.fetch("paths")
  fail!("retained manifest exact path set mismatch") unless entries.map { |entry| entry.fetch("path") } == expected_retained_paths
  entries.each do |entry|
    path = root.join(entry.fetch("path")).cleanpath
    fail!("retained path escapes repository") unless path.to_s.start_with?("#{root}/")
    fail!("retained path missing: #{entry.fetch('path')}") unless path.file?
    fail!("retained path digest mismatch: #{entry.fetch('path')}") unless sha(path) == entry.fetch("sha256")
  end
  _, _, allowlist = contract(root)
  h = retained.fetch("producer_head")
  evidence_paths = files.map { |entry| entry.fetch("path") }
  receipt_path = root.join(REVIEW_RECEIPT)
  if receipt_path.file?
    receipt = JSON.parse(receipt_path.read)
    payload = receipt.fetch("payload")
    fail!("review receipt schema mismatch") unless receipt["schema"] == "adl.h2_retention_review_receipt.v1"
    fail!("review receipt payload mismatch") unless receipt["payload_sha256"] == Digest::SHA256.hexdigest(canonical_json(payload))
    fail!("review receipt manifest mismatch") unless payload["retained_surface_manifest_sha256"] == sha(root.join(RETAINED_MANIFEST))
    fail!("review result mismatch") unless payload["review_result"] == "passed" && payload["findings"] == []
    fail!("review receipt producer mismatch") unless payload["h"] == h
    fail!("review scope missing") unless payload["reviewer"].to_s.start_with?("/") && !payload["review_scope"].to_s.empty?
    diff_digest = validate_h2_diff(root, h, payload.fetch("h2"), evidence_paths, allowlist)
    fail!("review receipt diff mismatch") if diff_digest && payload["h_to_h2_name_status_sha256"] != diff_digest
    h2_tree, h2_available = git(root, "rev-parse", "#{payload.fetch('h2')}^{tree}", allow_failure: true)
    fail!("review receipt H2 tree mismatch") if h2_available && payload["h2_tree"] != h2_tree.strip
    anchor = receipt_anchor(root, receipt, payload.fetch("h2"))
    puts JSON.generate(status: "passed", source_sha: source_sha, receipt_anchor: anchor, relation: "retained_surface")
  elsif ENV["ADL_ALLOW_UNREVIEWED_H2"] == "1"
    head = git(root, "rev-parse", "HEAD").first.strip
    fail!("unreviewed H2 diff unavailable") unless validate_h2_diff(root, h, head, evidence_paths, allowlist)
    puts JSON.generate(status: "passed", source_sha: source_sha, relation: "unreviewed_h2_preflight")
  else
    fail!("review receipt missing")
  end
end

def expect_failure(fragment)
  yield
  fail!("expected failure containing: #{fragment}")
rescue RuntimeError => error
  fail!("wrong failure: #{error.message}") unless error.message.include?(fragment)
end

def self_test(repo)
  source, proof, allowlist = contract(repo)
  workflow_paths = repo.join(WORKFLOW).read.split("permissions:", 2).first
  (source.fetch("paths") + proof.fetch("paths")).each { |path| fail!("workflow trigger omits #{path}") unless workflow_paths.include?(%Q["#{path}"]) }
  fail!("workflow recursively triggers on evidence") if workflow_paths.include?(%Q["#{EVIDENCE_ROOT}])
  fail!("workflow recursively triggers on lifecycle") if workflow_paths.include?(%Q[".csdlc/issues/217/])

  Dir.mktmpdir("adl-217-anchor-") do |directory|
    root = Pathname.new(directory)
    git(root, "init", "-q")
    git(root, "config", "user.email", "proof@example.invalid")
    git(root, "config", "user.name", "Proof Fixture")
    FileUtils.mkdir_p(root.join(EVIDENCE_ROOT))
    root.join(RETAINED_MANIFEST).write("manifest-v1\n")
    git(root, "add", ".")
    git(root, "commit", "-qm", "h2")
    h2 = git(root, "rev-parse", "HEAD").first.strip
    payload = { "h2" => h2, "retained_surface_manifest_sha256" => sha(root.join(RETAINED_MANIFEST)), "review_result" => "passed", "findings" => [] }
    packet = { "schema" => "adl.h2_retention_review_receipt.v1", "payload" => payload, "payload_sha256" => Digest::SHA256.hexdigest(canonical_json(payload)) }
    root.join(REVIEW_RECEIPT).write(JSON.pretty_generate(packet) + "\n")
    git(root, "add", ".")
    git(root, "commit", "-qm", "h3")
    parsed = JSON.parse(root.join(REVIEW_RECEIPT).read)
    anchor = receipt_anchor(root, parsed, h2)
    fail!("anchor commit mismatch") unless anchor["commit"] == git(root, "rev-parse", "HEAD").first.strip

    shallow = root.join("shallow")
    git(root, "clone", "-q", "--depth", "1", "file://#{root}", shallow.to_s)
    shallow_receipt = JSON.parse(shallow.join(REVIEW_RECEIPT).read)
    shallow_anchor = receipt_anchor(shallow, shallow_receipt, h2)
    fail!("shallow anchor mismatch") unless shallow_anchor["sha256"] == anchor["sha256"]

    # A coherent later rewrite still differs from the retained introduction blob.
    root.join(RETAINED_MANIFEST).write("manifest-v2\n")
    rewritten = JSON.parse(root.join(REVIEW_RECEIPT).read)
    rewritten["payload"]["retained_surface_manifest_sha256"] = sha(root.join(RETAINED_MANIFEST))
    rewritten["payload_sha256"] = Digest::SHA256.hexdigest(canonical_json(rewritten["payload"]))
    root.join(REVIEW_RECEIPT).write(JSON.pretty_generate(rewritten) + "\n")
    expect_failure("differs from ancestral anchor") { receipt_anchor(root, rewritten, h2) }

    git(root, "checkout", "--", RETAINED_MANIFEST, REVIEW_RECEIPT)
    original_receipt = root.join(REVIEW_RECEIPT).read
    FileUtils.rm_f(root.join(REVIEW_RECEIPT))
    git(root, "add", "-A")
    git(root, "commit", "-qm", "delete receipt")
    root.join(REVIEW_RECEIPT).write(original_receipt)
    git(root, "add", ".")
    git(root, "commit", "-qm", "readd receipt")
    expect_failure("missing or ambiguous") { receipt_anchor(root, JSON.parse(original_receipt), h2) }
  end

  Dir.mktmpdir("adl-217-diff-") do |directory|
    root = Pathname.new(directory)
    git(root, "init", "-q")
    git(root, "config", "user.email", "proof@example.invalid")
    git(root, "config", "user.name", "Proof Fixture")
    root.join("base.txt").write("base\n")
    git(root, "add", ".")
    git(root, "commit", "-qm", "H")
    h = git(root, "rev-parse", "HEAD").first.strip
    ([DENOMINATOR, RETAINED_MANIFEST] + TEN_PATHS).each do |relative|
      path = root.join(relative)
      FileUtils.mkdir_p(path.dirname)
      path.write("#{relative}\n")
    end
    git(root, "add", ".")
    git(root, "commit", "-qm", "H2")
    h2 = git(root, "rev-parse", "HEAD").first.strip
    fail!("valid H2 diff rejected") unless validate_h2_diff(root, h, h2, TEN_PATHS, { "lifecycle_paths" => [] })
    root.join("README.md").write("unexpected\n")
    git(root, "add", ".")
    git(root, "commit", "-qm", "unexpected source")
    bad = git(root, "rev-parse", "HEAD").first.strip
    expect_failure("unexpected path") { validate_h2_diff(root, h, bad, TEN_PATHS, { "lifecycle_paths" => [] }) }
  end
  puts JSON.generate(status: "passed", check: "retained-native-proof-contract")
end

begin
  root = Pathname.new(git(Pathname.pwd, "rev-parse", "--show-toplevel").first.strip).realpath
  if ARGV == ["--self-test"]
    self_test(root)
  elsif ARGV.first == "--aggregate"
    fail!("aggregate requires macOS and Linux receipts") unless ARGV.length == 3
    write_aggregate(root, ARGV.drop(1))
  else
    fail!("expected retained denominator") unless ARGV.length == 1
    validate_retained(root, ARGV.first)
  end
rescue StandardError => error
  warn(error.message)
  exit 1
end

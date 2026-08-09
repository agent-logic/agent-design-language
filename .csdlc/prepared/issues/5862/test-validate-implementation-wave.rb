#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "tmpdir"

VALIDATOR = File.expand_path("validate-implementation-wave.rb", __dir__)
ISSUE = 5863
PRODUCTS = ["adl-runtime/src/distributed/fixture.rs", "adl-runtime/tests/distributed_fixture.rs"].freeze
EVIDENCE_PATH = ".csdlc/evidence/#{ISSUE}/proof-v3".freeze
PROOF_PATH = "#{EVIDENCE_PATH}/execution-proof.json".freeze

def run!(*argv, chdir:)
  stdout, stderr, status = Open3.capture3(*argv, chdir: chdir)
  raise "#{argv.join(' ')} failed: #{stderr} #{stdout}" unless status.success?
  stdout.strip
end

def commit!(root, message)
  run!("git", "add", ".", chdir: root)
  run!("git", "commit", "-m", message, chdir: root)
  run!("git", "rev-parse", "HEAD", chdir: root)
end

def write(path, content)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, content)
end

def fixture(product_drift: false, evidence_drift: false, fake_source: false, transient_product: false, transient_evidence: false, unsafe_evidence: false, merge_product_drift: false, candidate_evidence_drift: false, split_evidence: false, issue: ISSUE, integrated: false, invalid_native_digest: false, file_evidence_mapping: false, late_integrated_artifact: false, sibling_late_artifact: false, product_directory: false, missing_runner: false, missing_v3_strategy: false, wrong_wp: false, invalid_negative_result: false, duplicate_native_run: false, fake_test_command: false, malformed_timestamp: false, reversed_timestamp: false, schema: "adl.wp04.execution_proof.v3")
  root = Dir.mktmpdir("adl-wave-topology")
  run!("git", "init", "-q", chdir: root)
  run!("git", "config", "user.email", "fixture@example.invalid", chdir: root)
  run!("git", "config", "user.name", "fixture", chdir: root)
  fixture_products = issue == 5878 ? [PRODUCTS.first, "adl-runtime/tests/distributed_guardian.rs"] : PRODUCTS
  fixture_products.each { |path| write(File.join(root, path), "#{path}\n") }
  evidence_path = ".csdlc/evidence/#{issue}/proof-v3"
  proof_path = "#{evidence_path}/execution-proof.json"
  if schema == "adl.wp04.execution_proof.v2"
    commit!(root, "legacy baseline")
    write(File.join(root, proof_path), "{\"legacy\":\"preexisting\"}\n")
    commit!(root, "legacy evidence predating source")
    write(File.join(root, fixture_products.first), "#{fixture_products.first}\nsource revision\n")
  end
  source = commit!(root, "source")
  declared_products = product_directory ? ["adl-runtime"] : fixture_products
  source_artifacts = declared_products.map do |path|
    digest = product_directory ? "0" * 64 : Digest::SHA256.file(File.join(root, path)).hexdigest
    {"path" => path, "sha256" => digest}
  end
  if split_evidence
    write(File.join(root, evidence_path, "premature.txt"), "premature\n")
    commit!(root, "premature evidence")
  end
  stdout_path = "#{evidence_path}/focused.stdout.log"
  stderr_path = "#{evidence_path}/focused.stderr.log"
  negative_path = "#{evidence_path}/negative.json"
  artifact_path = "#{evidence_path}/artifact.json"
  write(File.join(root, stdout_path), "3 tests passed\n")
  write(File.join(root, stderr_path), "")
  write(File.join(root, negative_path), "{}\n")
  write(File.join(root, artifact_path), "{}\n")
  commands = [{
    "argv" => ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_fixture", "--no-tests=fail"], "exit_code" => 0, "selected_tests" => 3,
    "started_at" => "2026-08-09T00:00:00Z", "finished_at" => "2026-08-09T00:00:01Z",
    "runner" => {"provider" => "fixture", "run_id" => "run-focused", "os" => "macos", "arch" => "aarch64", "identity_sha256" => Digest::SHA256.hexdigest("runner-focused")},
    "stdout_path" => stdout_path, "stdout_sha256" => Digest::SHA256.file(File.join(root, stdout_path)).hexdigest,
    "stderr_path" => stderr_path, "stderr_sha256" => Digest::SHA256.file(File.join(root, stderr_path)).hexdigest
  }]
  negative_cases = [{"case" => "post_source_drift", "result" => "rejected", "evidence_path" => negative_path, "evidence_sha256" => Digest::SHA256.file(File.join(root, negative_path)).hexdigest}]
  artifacts = [{"path" => artifact_path, "sha256" => Digest::SHA256.file(File.join(root, artifact_path)).hexdigest}]
  native_receipts = []
  if integrated
    commands = [
      ["cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_guardian", "--no-tests=fail"],
      ["bash", "adl/tools/validate_v092_distributed_guardian.sh"],
      ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"]
    ].each_with_index.map do |argv, index|
      stdout_path = "#{evidence_path}/command-#{index}.stdout.log"
      stderr_path = "#{evidence_path}/command-#{index}.stderr.log"
      write(File.join(root, stdout_path), "pass #{index}\n")
      write(File.join(root, stderr_path), "")
      {"argv" => argv, "exit_code" => 0, "selected_tests" => index + 1,
       "started_at" => "2026-08-09T00:00:0#{index}Z", "finished_at" => "2026-08-09T00:00:1#{index}Z",
       "runner" => {"provider" => "fixture", "run_id" => "run-command-#{index}", "os" => "macos", "arch" => "aarch64", "identity_sha256" => Digest::SHA256.hexdigest("runner-command-#{index}")},
       "stdout_path" => stdout_path, "stdout_sha256" => Digest::SHA256.file(File.join(root, stdout_path)).hexdigest, "stderr_path" => stderr_path, "stderr_sha256" => Digest::SHA256.file(File.join(root, stderr_path)).hexdigest}
    end
    negative_path = "#{evidence_path}/negative.json"
    write(File.join(root, negative_path), "{}\n")
    negative_cases = [{"case" => "native_tamper", "result" => "rejected", "evidence_path" => negative_path, "evidence_sha256" => Digest::SHA256.file(File.join(root, negative_path)).hexdigest}]
    artifact_path = sibling_late_artifact ? ".csdlc/evidence/#{issue}/sibling/integrated.json" : "#{evidence_path}/integrated.json"
    artifact_bytes = "{}\n"
    write(File.join(root, artifact_path), artifact_bytes) unless late_integrated_artifact || sibling_late_artifact
    artifacts = [{"path" => artifact_path, "sha256" => Digest::SHA256.hexdigest(artifact_bytes)}]
    native_receipts = %w[macos linux windows].each_with_index.map do |platform, index|
      stdout_path = "#{evidence_path}/#{platform}.stdout.log"
      stderr_path = "#{evidence_path}/#{platform}.stderr.log"
      artifact_path = "#{evidence_path}/#{platform}.json"
      write(File.join(root, stdout_path), "native #{platform} pass\n")
      write(File.join(root, stderr_path), "")
      write(File.join(root, artifact_path), "{\"platform\":\"#{platform}\"}\n")
      artifact_digest = invalid_native_digest && platform == "windows" ? "f" * 64 : Digest::SHA256.file(File.join(root, artifact_path)).hexdigest
      {"platform" => platform, "source_revision" => source,
       "command" => {"argv" => ["native", platform], "exit_code" => 0, "selected_tests" => 1,
                     "started_at" => "2026-08-09T00:01:0#{index}Z", "finished_at" => "2026-08-09T00:01:1#{index}Z",
                     "runner" => {"provider" => "fixture", "run_id" => "run-#{platform}", "os" => platform, "arch" => "fixture-arch", "identity_sha256" => Digest::SHA256.hexdigest("runner-#{index}")},
                     "stdout_path" => stdout_path, "stdout_sha256" => Digest::SHA256.file(File.join(root, stdout_path)).hexdigest,
                     "stderr_path" => stderr_path, "stderr_sha256" => Digest::SHA256.file(File.join(root, stderr_path)).hexdigest},
       "artifacts" => [{"path" => artifact_path, "sha256" => artifact_digest}]}
    end
    if duplicate_native_run
      native_receipts[1]["command"]["runner"]["run_id"] = native_receipts[0]["command"]["runner"]["run_id"]
      native_receipts[1]["command"]["runner"]["identity_sha256"] = native_receipts[0]["command"]["runner"]["identity_sha256"]
    end
  end
  commands[0].delete("runner") if missing_runner
  commands[0]["argv"][0] = "fake" if fake_test_command
  commands[0]["started_at"] = "not-a-time" if malformed_timestamp
  if reversed_timestamp
    commands[0]["started_at"] = "2026-08-09T00:00:02Z"
    commands[0]["finished_at"] = "2026-08-09T00:00:01Z"
  end
  negative_cases[0]["result"] = "passed" if invalid_negative_result
  proof = {
    "schema" => schema,
    "issue" => issue,
    "wp" => wrong_wp ? "WP-04.99" : (issue == 5878 ? "WP-04.16" : "WP-04.01"),
    "source_revision" => fake_source ? "0" * 40 : source,
    "evidence_revision_strategy" => schema == "adl.wp04.execution_proof.v3" ? "derive_from_receipt_introduction" : nil,
    "protected_paths" => declared_products,
    "source_artifacts" => source_artifacts,
    "commands" => commands,
    "negative_cases" => negative_cases,
    "artifacts" => artifacts,
    "native_receipts" => native_receipts
  }
  proof.delete("evidence_revision_strategy") if schema == "adl.wp04.execution_proof.v2"
  proof.delete("evidence_revision_strategy") if missing_v3_strategy
  write(File.join(root, proof_path), JSON.pretty_generate(proof) + "\n")
  evidence = commit!(root, "evidence")
  write(File.join(root, artifacts.fetch(0).fetch("path")), "{}\n") if late_integrated_artifact || sibling_late_artifact
  if transient_product
    original = File.read(File.join(root, fixture_products.first))
    write(File.join(root, fixture_products.first), "transient\n")
    commit!(root, "transient product drift")
    write(File.join(root, fixture_products.first), original)
    commit!(root, "revert product drift")
  end
  if transient_evidence
    write(File.join(root, evidence_path, "transient.txt"), "transient\n")
    commit!(root, "transient evidence drift")
    File.delete(File.join(root, evidence_path, "transient.txt"))
    commit!(root, "revert evidence drift")
  end
  if unsafe_evidence
    File.symlink("execution-proof.json", File.join(root, evidence_path, "unsafe-link"))
  end
  write(File.join(root, "lifecycle.json"), "{}\n")
  write(File.join(root, fixture_products.first), "drift\n") if product_drift
  write(File.join(root, evidence_path, "late.txt"), "drift\n") if evidence_drift
  head = commit!(root, "head")
  write(File.join(root, "merge.txt"), "merged\n")
  write(File.join(root, fixture_products.last), "merge drift\n") if merge_product_drift
  merge = commit!(root, "merge")
  write(File.join(root, "candidate.txt"), "umbrella\n")
  write(File.join(root, evidence_path, "candidate-drift.txt"), "drift\n") if candidate_evidence_drift
  candidate = commit!(root, "candidate")
  mapping = {
    "issue" => issue,
    "head_sha" => head,
    "evidence_sha" => evidence,
    "merge_sha" => merge,
    "execution_proof_path" => proof_path,
    "execution_proof_sha256" => Digest::SHA256.file(File.join(root, proof_path)).hexdigest,
    "evidence_path" => file_evidence_mapping ? proof_path : evidence_path,
    "product_paths" => declared_products
  }
  [root, mapping, candidate]
end

def validate(root, mappings, candidate, require_integrated_proof: false)
  request = File.join(root, "request.json")
  File.write(request, JSON.pretty_generate({"repository_root" => root, "candidate_sha" => candidate, "mappings" => mappings, "require_integrated_proof" => require_integrated_proof}) + "\n")
  Open3.capture3("ruby", VALIDATOR, "--validate-topology", request, chdir: root)
end

def expect_pass(name)
  root, mapping, candidate = fixture
  stdout, stderr, status = validate(root, [mapping], candidate)
  raise "#{name}: #{stderr} #{stdout}" unless status.success? && stdout.include?("PASS:")
ensure
  FileUtils.remove_entry(root) if root && File.exist?(root)
end

def expect_reject(name, expected, **fixture_options)
  root, mapping, candidate = fixture(**fixture_options)
  yield(mapping) if block_given?
  stdout, stderr, status = validate(root, [mapping], candidate)
  output = "#{stderr} #{stdout}"
  raise "#{name}: unexpectedly passed" if status.success?
  raise "#{name}: expected #{expected.inspect}, got #{output.inspect}" unless output.include?(expected)
ensure
  FileUtils.remove_entry(root) if root && File.exist?(root)
end

expect_pass("valid S-E-H")
root, mapping, candidate = fixture(schema: "adl.wp04.execution_proof.v2")
stdout, stderr, status = validate(root, [mapping], candidate)
raise "non-self-referential legacy v2 failed: #{stderr} #{stdout}" unless status.success?
FileUtils.remove_entry(root)
expect_reject("product drift", "product object or mode drift", product_drift: true)
expect_reject("evidence drift", "evidence object or mode drift", evidence_drift: true)
expect_reject("wrong head", "missing proof at child head") { |mapping| mapping["head_sha"] = "1" * 40 }
expect_reject("wrong merge", "head is not ancestral") { |mapping| mapping["merge_sha"] = mapping["evidence_sha"] }
expect_reject("missing mapping", "key not found") { |mapping| mapping.delete("evidence_sha") }
expect_reject("self-referential fake", "source is not ancestral", fake_source: true)
expect_reject("transient product drift", "transient product drift", transient_product: true)
expect_reject("transient evidence drift", "transient evidence drift", transient_evidence: true)
expect_reject("unsafe evidence", "unsafe symlink or gitlink", unsafe_evidence: true)
expect_reject("merge product drift", "product object or mode drift", merge_product_drift: true)
expect_reject("candidate evidence drift", "evidence object or mode drift", candidate_evidence_drift: true)
expect_reject("collapsed legacy v2", "self-referential or collapsed") do |mapping|
  mapping["evidence_sha"] = mapping["head_sha"]
end
expect_reject("proof digest mismatch", "proof digest drift") { |mapping| mapping["execution_proof_sha256"] = "f" * 64 }
expect_reject("split evidence introduction", "whole evidence mapping was not introduced once", split_evidence: true)
expect_reject("broken S-E ancestry", "source is not ancestral") { |mapping| mapping["evidence_sha"] = "2" * 40 }
expect_reject("file evidence mapping with late referenced artifact", "strict descendant", issue: 5878, integrated: true, file_evidence_mapping: true, late_integrated_artifact: true)
expect_reject("directory product mapping", "exact ordinary blob", product_directory: true)
expect_reject("missing command runner", "runner missing", missing_runner: true)
expect_reject("missing v3 strategy", "wrong v3 evidence revision strategy", missing_v3_strategy: true)
expect_reject("wrong WP mapping", "proof WP mapping drift", wrong_wp: true)
expect_reject("invalid negative result", "no proving result", invalid_negative_result: true)
expect_reject("fake exact test command", "missing or duplicate exact nonzero test command", fake_test_command: true)
expect_reject("malformed timestamp", "timestamps are not RFC3339", malformed_timestamp: true)
expect_reject("reversed timestamp", "finish time precedes start time", reversed_timestamp: true)
expect_reject("sibling late referenced artifact", "outside frozen mapping", issue: 5878, integrated: true, sibling_late_artifact: true)

root, mapping, candidate = fixture(issue: 5878, integrated: true)
stdout, stderr, status = validate(root, [mapping], candidate, require_integrated_proof: true)
raise "full WP-04.16 proof failed: #{stderr} #{stdout}" unless status.success?
FileUtils.remove_entry(root)

root, mapping, candidate = fixture(issue: 5878, integrated: true, duplicate_native_run: true)
stdout, stderr, status = validate(root, [mapping], candidate, require_integrated_proof: true)
raise "duplicate native runner unexpectedly passed" if status.success?
raise "duplicate native runner diagnostic missing: #{stderr} #{stdout}" unless "#{stderr} #{stdout}".include?("native run IDs are missing or duplicated")
FileUtils.remove_entry(root)

root, mapping, candidate = fixture(issue: 5878, integrated: true, invalid_native_digest: true)
stdout, stderr, status = validate(root, [mapping], candidate, require_integrated_proof: true)
raise "invalid native receipt unexpectedly passed" if status.success?
raise "native receipt diagnostic missing: #{stderr} #{stdout}" unless "#{stderr} #{stdout}".include?("native artifact digest drift")
FileUtils.remove_entry(root)

root, mapping, candidate = fixture
stdout, stderr, status = validate(root, [mapping, mapping.dup], candidate)
raise "ambiguous mapping unexpectedly passed" if status.success?
raise "ambiguous mapping diagnostic missing: #{stderr} #{stdout}" unless "#{stderr} #{stdout}".include?("missing or ambiguous")
FileUtils.remove_entry(root)

source = File.read(VALIDATOR)
raise "sixteen-child denominator missing" unless source.include?("EXPECTED = (1..16)")
raise "exact dependency DAG missing" unless source.include?("EXPECTED_DEPENDENCIES") && source.include?("dependency DAG drift")
raise "WP-04.13 dependency drift" unless source.include?("5875 => [5870, 5873, 5874]")
raise "WP-04.15 dependency drift" unless source.include?("5877 => [5867, 5870, 5875, 5876]")
raise "WP-04.16 integrated test binding missing" unless source.include?("validate_v092_distributed_guardian.sh")
raise "WP-04.16 native receipt binding missing" unless source.include?("validate_v092_distributed_native_receipts.rb")

real_legacy_path = File.expand_path("../../../evidence/5863/execution-proof.json", __dir__)
real_legacy = JSON.parse(File.read(real_legacy_path))
raise "real #5863 legacy schema drift" unless real_legacy["schema"] == "adl.wp04.execution_proof.v2"
raise "real #5863 legacy command shape drift" unless real_legacy.dig("commands", 0, "runner", "identity_sha256")
raise "real #5863 legacy artifact/negative denominator missing" if Array(real_legacy["artifacts"]).empty? || Array(real_legacy["negative_cases"]).empty?
real_legacy_source = real_legacy.fetch("source_revision")
run!("git", "cat-file", "-e", "#{real_legacy_source}:.csdlc/evidence/5863/execution-proof.json", chdir: File.expand_path("../../../..", __dir__))

puts "PASS: 31 generated v3/legacy topology and integrated-native cases, real #5863 legacy shape, plus sixteen-child and exact-DAG guards"

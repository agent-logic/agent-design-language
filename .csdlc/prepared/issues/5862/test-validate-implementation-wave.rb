#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "tmpdir"

VALIDATOR = File.expand_path("validate-implementation-wave.rb", __dir__)
ISSUE = 5863
PRODUCTS = ["product/source.rs", "product/test.rs"].freeze
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

def fixture(product_drift: false, evidence_drift: false, fake_source: false, transient_product: false, transient_evidence: false, unsafe_evidence: false, merge_product_drift: false, candidate_evidence_drift: false, split_evidence: false, issue: ISSUE, integrated: false, invalid_native_digest: false, schema: "adl.wp04.execution_proof.v3")
  root = Dir.mktmpdir("adl-wave-topology")
  run!("git", "init", "-q", chdir: root)
  run!("git", "config", "user.email", "fixture@example.invalid", chdir: root)
  run!("git", "config", "user.name", "fixture", chdir: root)
  PRODUCTS.each { |path| write(File.join(root, path), "#{path}\n") }
  source = commit!(root, "source")
  source_artifacts = PRODUCTS.map do |path|
    {"path" => path, "sha256" => Digest::SHA256.file(File.join(root, path)).hexdigest}
  end
  evidence_path = ".csdlc/evidence/#{issue}/proof-v3"
  proof_path = "#{evidence_path}/execution-proof.json"
  if split_evidence
    write(File.join(root, evidence_path, "premature.txt"), "premature\n")
    commit!(root, "premature evidence")
  end
  commands = [{"argv" => ["ruby", "focused.rb"], "exit_code" => 0, "selected_tests" => 3}]
  negative_cases = [{"case" => "post_source_drift", "result" => "rejected"}]
  artifacts = []
  native_receipts = []
  if integrated
    commands = [
      ["bash", "adl/tools/validate_v092_distributed_guardian.sh"],
      ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"]
    ].each_with_index.map do |argv, index|
      stdout_path = "#{evidence_path}/command-#{index}.stdout.log"
      stderr_path = "#{evidence_path}/command-#{index}.stderr.log"
      write(File.join(root, stdout_path), "pass #{index}\n")
      write(File.join(root, stderr_path), "")
      {"argv" => argv, "exit_code" => 0, "selected_tests" => index + 1, "stdout_path" => stdout_path, "stdout_sha256" => Digest::SHA256.file(File.join(root, stdout_path)).hexdigest, "stderr_path" => stderr_path, "stderr_sha256" => Digest::SHA256.file(File.join(root, stderr_path)).hexdigest}
    end
    negative_path = "#{evidence_path}/negative.json"
    write(File.join(root, negative_path), "{}\n")
    negative_cases = [{"case" => "native_tamper", "result" => "rejected", "evidence_path" => negative_path, "evidence_sha256" => Digest::SHA256.file(File.join(root, negative_path)).hexdigest}]
    artifact_path = "#{evidence_path}/integrated.json"
    write(File.join(root, artifact_path), "{}\n")
    artifacts = [{"path" => artifact_path, "sha256" => Digest::SHA256.file(File.join(root, artifact_path)).hexdigest}]
    native_receipts = %w[macos linux windows].each_with_index.map do |platform, index|
      path = "#{evidence_path}/#{platform}.json"
      write(File.join(root, path), "{\"platform\":\"#{platform}\"}\n")
      digest = invalid_native_digest && platform == "windows" ? "f" * 64 : Digest::SHA256.file(File.join(root, path)).hexdigest
      {"platform" => platform, "source_revision" => source, "run_id" => "run-#{platform}", "runner_identity_sha256" => Digest::SHA256.hexdigest("runner-#{index}"), "path" => path, "sha256" => digest}
    end
  end
  proof = {
    "schema" => schema,
    "issue" => issue,
    "wp" => issue == 5878 ? "WP-04.16" : "WP-04.01",
    "source_revision" => fake_source ? "0" * 40 : source,
    "protected_paths" => PRODUCTS,
    "source_artifacts" => source_artifacts,
    "commands" => commands,
    "negative_cases" => negative_cases,
    "artifacts" => artifacts,
    "native_receipts" => native_receipts
  }
  write(File.join(root, proof_path), JSON.pretty_generate(proof) + "\n")
  evidence = commit!(root, "evidence")
  if transient_product
    original = File.read(File.join(root, PRODUCTS.first))
    write(File.join(root, PRODUCTS.first), "transient\n")
    commit!(root, "transient product drift")
    write(File.join(root, PRODUCTS.first), original)
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
  write(File.join(root, PRODUCTS.first), "drift\n") if product_drift
  write(File.join(root, evidence_path, "late.txt"), "drift\n") if evidence_drift
  head = commit!(root, "head")
  write(File.join(root, "merge.txt"), "merged\n")
  write(File.join(root, PRODUCTS.last), "merge drift\n") if merge_product_drift
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
    "evidence_path" => evidence_path,
    "product_paths" => PRODUCTS
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

root, mapping, candidate = fixture(issue: 5878, integrated: true)
stdout, stderr, status = validate(root, [mapping], candidate, require_integrated_proof: true)
raise "full WP-04.16 proof failed: #{stderr} #{stdout}" unless status.success?
FileUtils.remove_entry(root)

root, mapping, candidate = fixture(issue: 5878, integrated: true, invalid_native_digest: true)
stdout, stderr, status = validate(root, [mapping], candidate, require_integrated_proof: true)
raise "invalid native receipt unexpectedly passed" if status.success?
raise "native receipt diagnostic missing: #{stderr} #{stdout}" unless "#{stderr} #{stdout}".include?("native receipt digest drift")
FileUtils.remove_entry(root)

root, mapping, candidate = fixture
stdout, stderr, status = validate(root, [mapping, mapping.dup], candidate)
raise "ambiguous mapping unexpectedly passed" if status.success?
raise "ambiguous mapping diagnostic missing: #{stderr} #{stdout}" unless "#{stderr} #{stdout}".include?("missing or ambiguous")
FileUtils.remove_entry(root)

source = File.read(VALIDATOR)
raise "sixteen-child denominator missing" unless source.include?("EXPECTED = (1..16)")
raise "exact dependency DAG missing" unless source.include?("EXPECTED_DEPENDENCIES") && source.include?("dependency DAG drift")
raise "WP-04.16 integrated test binding missing" unless source.include?("validate_v092_distributed_guardian.sh")
raise "WP-04.16 native receipt binding missing" unless source.include?("validate_v092_distributed_native_receipts.rb")

puts "PASS: 20 generated v3/legacy topology and integrated-native cases plus sixteen-child and exact-DAG guards"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "stringio"
require "tmpdir"

require_relative "../5862/proof-receipt-contract"

ISSUE = 53
WP = "WP-04.RECEIPT"
PRODUCT = "src/product.txt"
TEST_SOURCE = "tests/receipt_contract_test.rb"
PROTECTED_PATHS = [PRODUCT, TEST_SOURCE].freeze
TEST_NAME = "receipt_contract"
EVIDENCE = ".csdlc/evidence/#{ISSUE}"
PROOF = "#{EVIDENCE}/execution-proof.json"
FIXTURE_PARENT = File.expand_path(".csdlc/prepared/issues/53", Dir.pwd)

def run_git(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  raise "git #{args.join(' ')} failed: #{stderr}" unless status.success?
  stdout.strip
end

def write(path, content)
  FileUtils.mkdir_p(File.dirname(path))
  File.binwrite(path, content)
end

def commit(message)
  run_git("add", "--all")
  run_git("commit", "--quiet", "-m", message)
  run_git("rev-parse", "HEAD")
end

def digest(path)
  Digest::SHA256.file(path).hexdigest
end

def base_proof(source, schema: "adl.wp04.execution_proof.v3")
  stdout_path = "#{EVIDENCE}/test.stdout.log"
  stderr_path = "#{EVIDENCE}/test.stderr.log"
  negative_path = "#{EVIDENCE}/negative.json"
  {
    "schema" => schema,
    "issue" => ISSUE,
    "wp" => WP,
    "source_revision" => source,
    "protected_paths" => PROTECTED_PATHS,
    "commands" => [{
      "argv" => ["ruby", "test", TEST_NAME, "--no-tests=fail"],
      "exit_code" => 0,
      "selected_tests" => 1,
      "started_at" => "2026-08-08T00:00:00Z",
      "finished_at" => "2026-08-08T00:00:01Z",
      "runner" => {
        "provider" => "local",
        "run_id" => "receipt-contract-fixture",
        "os" => "test",
        "arch" => "test",
        "identity_sha256" => "a" * 64
      },
      "stdout_path" => stdout_path,
      "stdout_sha256" => digest(stdout_path),
      "stderr_path" => stderr_path,
      "stderr_sha256" => digest(stderr_path)
    }],
    "negative_cases" => [{
      "case" => "fixture-denial",
      "result" => "denied",
      "evidence_path" => negative_path,
      "evidence_sha256" => digest(negative_path)
    }],
    "artifacts" => [
      {"path" => stdout_path, "sha256" => digest(stdout_path)},
      {"path" => negative_path, "sha256" => digest(negative_path)}
    ],
    "native_receipts" => []
  }.tap do |proof|
    if schema.end_with?("v3")
      proof["source_artifacts"] = [
        {"path" => PRODUCT, "sha256" => Digest::SHA256.hexdigest("product A\n")},
        {"path" => TEST_SOURCE, "sha256" => Digest::SHA256.hexdigest("test A\n")}
      ]
      proof["evidence_revision_strategy"] = "derive_from_receipt_introduction"
    end
  end
end

def fixture(schema: "adl.wp04.execution_proof.v3", mutate_before_evidence: nil, mutate_after_evidence: nil)
  Dir.mktmpdir("receipt-fixture-", FIXTURE_PARENT) do |dir|
    Dir.chdir(dir) do
      run_git("init", "--quiet", "--initial-branch=main")
      run_git("config", "user.email", "csdlc-test@example.invalid")
      run_git("config", "user.name", "C-SDLC Test")
      write(PRODUCT, "product A\n")
      write(TEST_SOURCE, "test A\n")
      source = commit("substantive A")

      write("#{EVIDENCE}/test.stdout.log", "one selected test passed\n")
      write("#{EVIDENCE}/test.stderr.log", "")
      write("#{EVIDENCE}/negative.json", "{\"result\":\"denied\"}\n")
      proof = base_proof(source, schema: schema)
      mutate_before_evidence&.call(proof, source)
      write(PROOF, JSON.pretty_generate(proof) + "\n")

      evidence = schema.end_with?("v3") ? commit("evidence-only B") : source
      if schema.end_with?("v3")
        write(".csdlc/issues/#{ISSUE}/metadata.json", "{\"phase\":\"reviewed\"}\n")
        commit("later metadata C")
      end
      mutate_after_evidence&.call(proof, source, evidence)
      yield(source, evidence)
    end
  end
end

def validate
  ARGV.replace([PROOF])
  Wp04ProofReceiptContract.validate(
    issue: ISSUE,
    wp: WP,
    paths: PROTECTED_PATHS,
    test: TEST_NAME,
    platforms: []
  )
end

def expect_failure(label, pattern)
  stderr = StringIO.new
  original = $stderr
  $stderr = stderr
  failed = false
  begin
    yield
  rescue SystemExit
    failed = true
  ensure
    $stderr = original
  end
  raise "#{label}: expected failure" unless failed
  raise "#{label}: wrong failure: #{stderr.string}" unless stderr.string.match?(pattern)
end

fixture { validate }

fixture(mutate_before_evidence: ->(_proof, _source) { write(PRODUCT, "product drift in B\n") }) do
  expect_failure("product drift", /escapes issue evidence/) { validate }
end

fixture(mutate_before_evidence: ->(_proof, _source) { write(TEST_SOURCE, "test tamper in B\n") }) do
  expect_failure("test tamper", /escapes issue evidence/) { validate }
end

fixture(mutate_before_evidence: ->(proof, _source) { proof["source_revision"] = "bad" }) do
  expect_failure("malformed source", /source revision malformed/) { validate }
end

fixture(mutate_before_evidence: ->(proof, _source) { proof["source_artifacts"][0]["sha256"] = "0" * 64 }) do
  expect_failure("source digest", /source artifact digest mismatch/) { validate }
end

fixture(mutate_before_evidence: ->(proof, _source) { proof["evidence_revision"] = "0" * 40 }) do
  expect_failure("self-referential field", /stored evidence revision is self-referential/) { validate }
end

fixture(mutate_after_evidence: lambda do |proof, _source, _evidence|
  proof["tampered_marker"] = true
  write(PROOF, JSON.pretty_generate(proof) + "\n")
  commit("tamper receipt")
end) do
  expect_failure("receipt tamper", /receipt content differs from evidence revision/) { validate }
end

fixture(mutate_after_evidence: lambda do |_proof, _source, _evidence|
  original = File.binread("#{EVIDENCE}/test.stdout.log")
  write("#{EVIDENCE}/test.stdout.log", "transient tamper\n")
  commit("tamper evidence")
  write("#{EVIDENCE}/test.stdout.log", original)
  commit("revert evidence tamper")
end) do
  expect_failure("tamper then revert", /evidence changed after its introduction/) { validate }
end

fixture(mutate_after_evidence: ->(_proof, _source, _evidence) { write("#{EVIDENCE}/test.stdout.log", "tampered\n") }) do
  expect_failure("log tamper", /stdout digest mismatch/) { validate }
end

fixture(mutate_before_evidence: lambda do |proof, source|
  run_git("checkout", "--quiet", "--orphan", "unrelated")
  run_git("rm", "--quiet", "-r", "-f", ".")
  write(PRODUCT, "product A\n")
  write(TEST_SOURCE, "test A\n")
  unrelated = commit("unrelated source")
  run_git("checkout", "--quiet", "main")
  raise "fixture source changed" unless run_git("rev-parse", "HEAD") == source
  proof["source_revision"] = unrelated
end) do
  expect_failure("unrelated ancestry", /not an ancestor/) { validate }
end

fixture(schema: "adl.wp04.execution_proof.v2") do
  validate
  write("metadata.txt", "later\n")
  commit("later head")
  expect_failure("v2 exact HEAD", /stale source revision/) { validate }
end

puts "PASS: issue 53 non-self-referential A/B/C receipt contract and fail-closed cases"

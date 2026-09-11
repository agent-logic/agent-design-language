#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "digest"
require "json"
require "open3"
require "tmpdir"

SOURCE_ROOT = File.expand_path("../../../..", __dir__)
VALIDATOR = ".csdlc/prepared/issues/833/validate-release-evidence.rb"
VERSION_VALIDATOR = ".csdlc/prepared/issues/833/validate-release-version.rb"
MANIFESTS = %w[
  adl/Cargo.toml adl-runtime/Cargo.toml adl-runtime-kernel/Cargo.toml
  adl-resilience/Cargo.toml adl-characterization/Cargo.toml adl-v2/Cargo.toml
  csdlc-v2/Cargo.toml tools/remote_validation/Cargo.toml
].freeze
LOCKS = %w[
  adl/Cargo.lock adl-runtime/Cargo.lock adl-runtime-kernel/Cargo.lock
  adl-resilience/Cargo.lock adl-characterization/Cargo.lock adl-v2/Cargo.lock
  adl-v2/crates/adl-runtime-v3-adapter/Cargo.lock csdlc-v2/Cargo.lock
  tools/remote_validation/Cargo.lock
].freeze
EXPECTED_LOCK_PACKAGES = {
  "adl/Cargo.lock" => %w[adl adl-resilience adl-runtime adl-runtime-kernel],
  "adl-runtime/Cargo.lock" => %w[adl-resilience adl-runtime adl-runtime-kernel],
  "adl-runtime-kernel/Cargo.lock" => %w[adl-runtime-kernel],
  "adl-resilience/Cargo.lock" => %w[adl-resilience],
  "adl-characterization/Cargo.lock" => %w[adl-characterization],
  "adl-v2/Cargo.lock" => %w[adl-adapters adl-cli adl-compiler adl-engine adl-language adl-records adl-runtime-kernel adl-runtime-v3-adapter adl-workcell-conductor adl-workcell-convergence adl-workcell-task-adapter],
  "adl-v2/crates/adl-runtime-v3-adapter/Cargo.lock" => %w[adl-runtime-kernel],
  "csdlc-v2/Cargo.lock" => %w[adl-resilience csdlc-v2],
  "tools/remote_validation/Cargo.lock" => %w[adl-remote-validation]
}.freeze

def run(*argv, env: {})
  Open3.capture3(env, *argv)
end

def write(path, content)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, content)
end

def commit(root, message)
  run("git", "-C", root, "add", ".")
  _out, err, status = run("git", "-C", root, "commit", "-q", "-m", message)
  abort err unless status.success?
  run("git", "-C", root, "rev-parse", "HEAD").first.strip
end

def validate(root)
  Open3.capture3({ "ADL_RELEASE_VALIDATION_ROOT" => root }, "ruby", File.join(root, VALIDATOR), "gate",
                 File.join(root, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json"))
end

Dir.mktmpdir("v0921-native-gate-") do |root|
  run("git", "-C", root, "init", "-q", "--initial-branch=main")
  run("git", "-C", root, "config", "user.name", "Gate Test")
  run("git", "-C", root, "config", "user.email", "gate@example.invalid")
  FileUtils.cp(File.join(SOURCE_ROOT, VALIDATOR), File.join(root, VALIDATOR).tap { |p| FileUtils.mkdir_p(File.dirname(p)) })
  FileUtils.cp(File.join(SOURCE_ROOT, VERSION_VALIDATOR), File.join(root, VERSION_VALIDATOR))
  MANIFESTS.each { |path| write(File.join(root, path), "[package]\nname = \"fixture\"\nversion = \"0.92.1\"\n") }
  LOCKS.each do |path|
    packages = EXPECTED_LOCK_PACKAGES.fetch(path).map { |name| "[[package]]\nname = \"#{name}\"\nversion = \"0.92.1\"\n" }.join("\n")
    write(File.join(root, path), "version = 4\n\n#{packages}")
  end
  write(File.join(root, "product.txt"), "candidate\n")
  candidate = commit(root, "candidate")

  status_path = "docs/milestones/v0.92.1/evidence/release/current-status/status.json"
  status = { "candidate" => candidate, "release_decision" => "ready", "release_authorized" => true, "blockers" => [] }
  write(File.join(root, status_path), JSON.pretty_generate(status))
  gate = {
    "schema" => "adl.v0921.native_v3_release_ceremony_gate.v1",
    "version" => "v0.92.1", "authority" => "csdlc-v3",
    "candidate_policy" => "reviewed_authority_inputs_at_clean_head",
    "validator" => VALIDATOR, "version_validator" => VERSION_VALIDATOR,
    "release_status" => status_path, "evidence_candidate" => candidate,
    "release_mutation_authorized" => false,
    "authority_input_sha256" => {
      "release_status" => Digest::SHA256.file(File.join(root, status_path)).hexdigest,
      "version_validator" => Digest::SHA256.file(File.join(root, VERSION_VALIDATOR)).hexdigest,
      "gate_validator" => Digest::SHA256.file(File.join(root, VALIDATOR)).hexdigest
    },
    "non_claims" => ["This gate never invokes C-SDLC v2."]
  }
  write(File.join(root, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json"), JSON.pretty_generate(gate))
  commit(root, "reviewed authority inputs")

  _out, err, result = validate(root)
  abort "valid gate rejected: #{err}" unless result.success?
  baseline = run("git", "-C", root, "rev-parse", "HEAD").first.strip

  cases = {
    "wrong authority" => ["non-v3 authority", -> { gate["authority"] = "csdlc-v2" }],
    "unauthorized" => ["release is not authorized", -> { status["release_authorized"] = false }],
    "blocker" => ["release blockers remain", -> { status["blockers"] = ["open"] }],
    "candidate drift" => ["release evidence candidate drift", -> { gate["evidence_candidate"] = "f" * 40 }],
    "local package mismatch" => ["release version validation failed", -> { write(File.join(root, LOCKS.first), "version = 4\n\n[[package]]\nname = \"adl\"\nversion = \"0.91.9\"\n") }]
  }

  cases.each do |name, (expected, mutation)|
    run("git", "-C", root, "reset", "--hard", baseline)
    gate = JSON.parse(File.read(File.join(root, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json")))
    status = JSON.parse(File.read(File.join(root, status_path)))
    mutation.call
    write(File.join(root, status_path), JSON.pretty_generate(status))
    gate["authority_input_sha256"]["release_status"] = Digest::SHA256.file(File.join(root, status_path)).hexdigest
    write(File.join(root, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json"), JSON.pretty_generate(gate))
    commit(root, "negative #{name}")
    _stdout, stderr, rejected = validate(root)
    abort "#{name}: malformed gate was accepted" if rejected.success?
    abort "#{name}: expected #{expected.inspect}, got #{stderr.inspect}" unless stderr.include?(expected)
  end

  run("git", "-C", root, "reset", "--hard", baseline)
  status = JSON.parse(File.read(File.join(root, status_path)))
  status["release_authorized"] = false
  write(File.join(root, status_path), JSON.pretty_generate(status))
  commit(root, "tamper status without rebinding")
  _out, stderr, tampered = validate(root)
  abort "tampered status was accepted" if tampered.success?
  abort "tampered status missed digest guard" unless stderr.include?("release status digest mismatch")

  run("git", "-C", root, "reset", "--hard", baseline)
  FileUtils.rm(File.join(root, status_path))
  commit(root, "remove status")
  _out, stderr, missing = validate(root)
  abort "missing status was accepted" if missing.success?
  abort "missing status missed guard" unless stderr.include?("release status missing")

  # A registry dependency at 0.92.0 is allowed; only local/workspace packages
  # are release-version-bearing.
  run("git", "-C", root, "reset", "--hard", baseline)
  packages = EXPECTED_LOCK_PACKAGES.fetch(LOCKS.first).map { |name| "[[package]]\nname = \"#{name}\"\nversion = \"0.92.1\"\n" }.join("\n")
  write(File.join(root, LOCKS.first), "version = 4\n\n#{packages}\n[[package]]\nname = \"external\"\nversion = \"0.92.0\"\nsource = \"registry+https://example.invalid/index\"\n")
  _out, err, external = Open3.capture3(
    { "ADL_RELEASE_VALIDATION_ROOT" => root }, "ruby", File.join(root, VERSION_VALIDATOR)
  )
  abort "external dependency false-positive: #{err}" unless external.success?
end

production_gate = File.join(SOURCE_ROOT, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json")
production_gate_sha = Digest::SHA256.file(production_gate).hexdigest
ceremony = File.read(File.join(SOURCE_ROOT, "adl/tools/release_ceremony.sh"))
abort "release ceremony does not pin the production gate digest" unless ceremony.include?(production_gate_sha)

puts "PASS: native v3 release gate binds authority inputs and rejects seven exact malformed cases"

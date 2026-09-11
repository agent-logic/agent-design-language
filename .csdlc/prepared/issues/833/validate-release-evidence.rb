#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"

ROOT = ENV.fetch("ADL_RELEASE_VALIDATION_ROOT", File.expand_path("../../../..", __dir__))
EXPECTED_VERSION = "v0.92.1"

def fail_with(errors)
  warn JSON.generate(schema: "adl.v0921.native_v3_release_ceremony_validation.v1", status: "fail", errors: errors)
  exit 1
end

manifest_path = ARGV[1] || File.join(ROOT, "docs/milestones/v0.92.1/RELEASE_CEREMONY_GATE_v0.92.1.json")
fail_with(["native v3 gate missing"]) unless File.file?(manifest_path)

errors = []
gate = JSON.parse(File.read(manifest_path))
errors << "wrong schema" unless gate["schema"] == "adl.v0921.native_v3_release_ceremony_gate.v1"
errors << "wrong version" unless gate["version"] == EXPECTED_VERSION
errors << "non-v3 authority" unless gate["authority"] == "csdlc-v3"
errors << "candidate policy drift" unless gate["candidate_policy"] == "reviewed_authority_inputs_at_clean_head"
errors << "validator route drift" unless gate["validator"] == ".csdlc/prepared/issues/833/validate-release-evidence.rb"
errors << "gate may not authorize mutation" unless gate["release_mutation_authorized"] == false
errors << "v2 fallback claim missing" unless gate.fetch("non_claims", []).include?("This gate never invokes C-SDLC v2.")

head, head_status = Open3.capture2("git", "-C", ROOT, "rev-parse", "HEAD")
errors << "cannot resolve current HEAD" unless head_status.success?
head = head.strip
dirty, dirty_status = Open3.capture2("git", "-C", ROOT, "status", "--porcelain")
errors << "cannot inspect checkout" unless dirty_status.success?
errors << "release checkout is dirty" unless dirty.empty?

status_path = File.join(ROOT, gate.fetch("release_status", ""))
if File.file?(status_path)
  expected_status_sha = gate.dig("authority_input_sha256", "release_status")
  errors << "release status digest mismatch" unless Digest::SHA256.file(status_path).hexdigest == expected_status_sha
  status = JSON.parse(File.read(status_path))
  candidate = status["candidate"].to_s
  candidate_exists = system("git", "-C", ROOT, "cat-file", "-e", "#{candidate}^{commit}", out: File::NULL, err: File::NULL)
  errors << "release candidate is missing" unless candidate_exists
  if candidate_exists
    ancestry_ok = system("git", "-C", ROOT, "merge-base", "--is-ancestor", candidate, head, out: File::NULL, err: File::NULL)
    errors << "release projection candidate is not an ancestor" unless ancestry_ok
  end
  errors << "release evidence candidate drift" unless candidate == gate["evidence_candidate"]
  errors << "release decision is not ready" unless status["release_decision"] == "ready"
  errors << "release is not authorized" unless status["release_authorized"] == true
  errors << "release blockers remain" unless status.fetch("blockers", []).empty?
else
  errors << "release status missing"
end

version_validator = gate.fetch("version_validator", "")
if version_validator.empty? || !File.file?(File.join(ROOT, version_validator))
  errors << "release version validator missing"
else
  expected_version_sha = gate.dig("authority_input_sha256", "version_validator")
  errors << "release version validator digest mismatch" unless Digest::SHA256.file(File.join(ROOT, version_validator)).hexdigest == expected_version_sha
  _stdout, stderr, version_status = Open3.capture3(
    { "ADL_RELEASE_VALIDATION_ROOT" => ROOT },
    "ruby", File.join(ROOT, version_validator)
  )
  errors << "release version validation failed: #{stderr.strip}" unless version_status.success?
end

fail_with(errors) unless errors.empty?
puts JSON.generate(schema: "adl.v0921.native_v3_release_ceremony_validation.v1", status: "pass", candidate: head)

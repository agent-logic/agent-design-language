#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = ROOT / ".csdlc/prepared/issues/311/validate-quality-gate.rb"
MATRIX = ROOT / "docs/reviews/v0.92/quality-gate-311/feature-completion-matrix.json"
WORK = ROOT / ".csdlc/evidence/311/negative-fixtures"

def invoke(path)
  Open3.capture3("ruby", VALIDATOR.to_s, "matrix", "--matrix", path.to_s, chdir: ROOT.to_s)
end

def expect_failure(name, matrix)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  _stdout, _stderr, status = invoke(path)
  raise "#{name} unexpectedly passed" if status.success?
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = JSON.parse(MATRIX.read)

stdout, stderr, status = invoke(MATRIX)
raise "positive matrix failed: #{stdout} #{stderr}" unless status.success?

tampered = Marshal.load(Marshal.dump(base))
tampered["rows"].shift
expect_failure("missing-row", tampered)

tampered = Marshal.load(Marshal.dump(base))
tampered["rows"] << Marshal.load(Marshal.dump(tampered["rows"].first))
expect_failure("duplicate-row", tampered)

tampered = Marshal.load(Marshal.dump(base))
tampered["rows"] << { "id" => "feature:INVENTED", "kind" => "feature", "source" => "invented", "owner" => "none", "disposition" => "blocked", "claim_boundary" => "none", "blockers" => ["invented"] }
expect_failure("extra-row", tampered)

tampered = Marshal.load(Marshal.dump(base))
tampered["evaluation_base_sha"] = "0" * 40
expect_failure("stale-head", tampered)

tampered = Marshal.load(Marshal.dump(base))
tampered["rows"].first["disposition"] = "planned"
expect_failure("planned-disposition", tampered)

tampered = Marshal.load(Marshal.dump(base))
tampered["rows"].first["blockers"] = []
expect_failure("blockerless-row", tampered)

tampered = Marshal.load(Marshal.dump(base))
row = tampered["rows"].first
row["disposition"] = "accepted"
row["blockers"] = []
row["evidence"] = { "authority_kind" => "self_asserted_json" }
expect_failure("self-attested-accepted", tampered)

tampered = Marshal.load(Marshal.dump(base))
row = tampered["rows"].first
row["disposition"] = "accepted"
row["blockers"] = []
row["evidence"] = {
  "authority_kind" => "canonical_observation", "implementation_paths" => ["adl/src/lib.rs"],
  "reviewed_head" => "1" * 40, "pull_request" => 1, "merge_sha" => "2" * 40,
  "positive" => ["x"], "negative" => ["x"], "integration" => ["x"], "platform" => ["x"],
  "typed_terminal" => { "generation" => 1, "digest" => "3" * 64 }, "review_artifact" => "x",
  "required_checks" => ["fabricated"]
}
expect_failure("unresolvable-git-authority", tampered)

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_negative_suite.v1", status: "passed", cases: 8)

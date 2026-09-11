#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path(__dir__)
SOURCE = File.join(ROOT, "issue-818-candidate-current-supersession.json")
VALIDATOR = File.join(ROOT, "validate-issue-818-candidate-current-supersession.rb")
TMP = File.join(ROOT, ".negative-tmp")

def reject_case(source, name, expected)
  copy = Marshal.load(Marshal.dump(source))
  yield copy
  path = File.join(TMP, "#{name}.json")
  File.write(path, JSON.pretty_generate(copy) + "\n")
  output, status = Open3.capture2e({"ISSUE818_SUPERSESSION" => path}, "ruby", VALIDATOR)
  raise "negative case passed: #{name}" if status.success?
  raise "wrong rejection for #{name}: #{output}" unless output.include?(expected)
end

FileUtils.rm_rf(TMP)
FileUtils.mkdir_p(TMP)
source = JSON.parse(File.read(SOURCE))

reject_case(source, "missing-row", "supersession rows differ") { |copy| copy.fetch("rows").pop }
reject_case(source, "duplicate-row", "duplicate supersession row") { |copy| copy.fetch("rows")[-1] = copy.fetch("rows").first }
reject_case(source, "proposal-digest", "supersession rows differ") { |copy| copy.fetch("rows").first["proposal_digest"] = "0" * 64 }
reject_case(source, "aggregate-digest", "proposal digest set mismatch") { |copy| copy["proposal_digest_set_sha256"] = "0" * 64 }
reject_case(source, "widened-disposition", "approval disposition mismatch") { |copy| copy["disposition"] = "release_approved" }
reject_case(source, "widened-supersession", "supersession scope widened") { |copy| copy["supersedes"] = "All milestone blockers" }
reject_case(source, "downstream-close-authority", "downstream authority widened") { |copy| copy.fetch("non_claims")[-1] = "This closes #522 and #833." }
reject_case(source, "wrong-merge", "ancestry failure") { |copy| copy["merge_commit"] = copy.fetch("source_candidate") }

FileUtils.rm_rf(TMP)
puts "PASS: 8/8 malformed, forged, or scope-widened #818 supersession receipts rejected"

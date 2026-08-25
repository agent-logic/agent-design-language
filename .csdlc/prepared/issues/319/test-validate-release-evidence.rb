#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "fileutils"

root = File.expand_path("../../../..", __dir__)
source = File.join(root, "docs/milestones/v0.92/RELEASE_CEREMONY_GATE_v0.92.json")
validator = File.join(__dir__, "validate-release-evidence.rb")
work = File.join(__dir__, ".negative-work")
FileUtils.rm_rf(work)
FileUtils.mkdir_p(work)

cases = {
  "wrong-disposition" => ->(j) { j["predecessors"][0]["disposition"] = "anything" },
  "wrong-pr" => ->(j) { j["predecessors"][1]["pr"] = 999 },
  "missing-required-merge" => ->(j) { j["predecessors"][2]["merge_commit"] = nil },
  "unrelated-ancestral-merge" => ->(j) { j["predecessors"][3]["merge_commit"] = `git -C #{root} rev-list --max-parents=0 HEAD`.strip },
  "evidence-substitution" => ->(j) { j["predecessors"][4]["evidence"] = "README.md" },
  "evidence-digest-tamper" => ->(j) { j["predecessors"][5]["evidence_sha256"] = "0" * 64 },
  "external-review-as-approval" => ->(j) { j["predecessors"][6]["disposition"] = "reviewed_green_merge" }
}

begin
  cases.each do |name, mutate|
    packet = JSON.parse(File.read(source))
    mutate.call(packet)
    path = File.join(work, "#{name}.json")
    File.write(path, JSON.pretty_generate(packet))
    abort "negative passed: #{name}" if system("ruby", validator, "gate", path, out: File::NULL, err: File::NULL)
  end
  puts JSON.generate(schema: "adl.v092.release_ceremony_negative.v1", status: "pass", cases: cases.length)
ensure
  FileUtils.rm_rf(work)
end

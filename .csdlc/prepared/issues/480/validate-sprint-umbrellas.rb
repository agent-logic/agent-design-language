#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01")
RECEIPT = File.join(EVIDENCE, "sprint-umbrella-membership-v3-receipt.json")
RUNNER = File.join(__dir__, "create-sprint-umbrellas.rb")

packet = JSON.parse(File.read(RECEIPT))
abort "wrong umbrella receipt schema" unless packet.fetch("schema") == "adl.v0921.wp01.sprint-umbrella-membership-update.v1"
abort "wrong membership version" unless packet.fetch("membership_version") == 3
reason = packet.fetch("change_reason")
abort "missing change reason" if reason.strip.empty?
rows = packet.fetch("umbrellas")
abort "umbrella denominator mismatch" unless rows.length == 11 && rows.map { |row| row.fetch("sprint") } == (1..11).to_a
abort "duplicate umbrella issue" unless rows.map { |row| row.fetch("issue") }.uniq.length == 11

runner = File.read(RUNNER)
abort "machine-local path in runner" if runner.match?(%r{/(Users|Volumes)/|github\.token})
abort "typed owner is not environment-resolved" unless runner.include?('ENV.fetch("CSDLC_GITHUB_ISSUE_BIN")')

rows.each do |row|
  sprint = row.fetch("sprint")
  abort "row version mismatch" unless row.fetch("membership_version") == 3 && row.fetch("change_reason") == reason
  result_path = File.join(EVIDENCE, "umbrella-update-v3-operations", format("sprint-%02d.json", sprint))
  abort "result digest mismatch" unless Digest::SHA256.file(result_path).hexdigest == row.fetch("result_sha256")
  stdout, stderr, status = Open3.capture3("gh", "api", "repos/agent-logic/agent-design-language/issues/#{row.fetch("issue")}")
  abort "live read failed: #{stderr}" unless status.success?
  live = JSON.parse(stdout)
  labels = live.fetch("labels").map { |label| label.fetch("name") }
  body = live.fetch("body")
  abort "live identity mismatch" unless live.fetch("state") == "open" && live.fetch("title") == row.fetch("title") &&
                                         live.dig("milestone", "number") == 1 && labels.include?("version:v0.92.1")
  abort "live version/reason mismatch" unless body.include?("Membership version: `3`") && body.include?(reason)
  row.fetch("members").each { |issue| abort "missing member ##{issue}" unless body.include?("- ##{issue}\n") }
end

puts JSON.generate(schema: "adl.v0921.wp01.sprint-umbrella-validation.v1", result: "passed",
                   membership_version: 3, umbrellas: 11)

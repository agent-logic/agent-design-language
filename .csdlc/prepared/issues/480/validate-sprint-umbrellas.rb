#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01")
RUNNER = File.join(__dir__, "create-sprint-umbrellas.rb")

receipt_paths = Dir.glob(File.join(EVIDENCE, "sprint-umbrella-membership-v*-receipt.json"))
abort "missing versioned umbrella receipt" if receipt_paths.empty?
versions = receipt_paths.to_h do |path|
  version = File.basename(path)[/membership-v(\d+)-receipt/, 1]
  abort "invalid versioned receipt name" unless version
  [Integer(version, 10), path]
end
membership_version = versions.keys.max
RECEIPT = versions.fetch(membership_version)

packet = JSON.parse(File.read(RECEIPT))
abort "wrong umbrella receipt schema" unless packet.fetch("schema") == "adl.v0921.wp01.sprint-umbrella-membership-update.v1"
abort "wrong membership version" unless packet.fetch("membership_version") == membership_version
reason = packet.fetch("change_reason")
abort "missing change reason" if reason.strip.empty?
rows = packet.fetch("umbrellas")
abort "umbrella denominator mismatch" unless rows.length == 11 && rows.map { |row| row.fetch("sprint") } == (1..11).to_a
abort "duplicate umbrella issue" unless rows.map { |row| row.fetch("issue") }.uniq.length == 11
all_members = rows.flat_map { |row| row.fetch("members") }
duplicate_members = all_members.group_by(&:itself).select { |_issue, owners| owners.length > 1 }.keys.sort
abort "duplicate issue ownership across Sprint umbrellas: #{duplicate_members.join(",")}" unless duplicate_members.empty?

expected_existing_homes = { 84 => 8, 122 => 2, 251 => 2, 345 => 7 }
expected_existing_homes.each do |issue, sprint|
  owners = rows.select { |row| row.fetch("members").include?(issue) }.map { |row| row.fetch("sprint") }
  abort "wrong Sprint home for ##{issue}: #{owners.inspect}" unless owners == [sprint]
end

runner = File.read(RUNNER)
abort "machine-local path in runner" if runner.match?(%r{/(Users|Volumes)/|github\.token})
abort "typed owner is not environment-resolved" unless runner.include?('ENV.fetch("CSDLC_GITHUB_ISSUE_BIN")')

rows.each do |row|
  sprint = row.fetch("sprint")
  abort "row version mismatch" unless row.fetch("membership_version") == membership_version && row.fetch("change_reason") == reason
  result_path = File.join(EVIDENCE, "umbrella-update-v#{membership_version}-operations", format("sprint-%02d.json", sprint))
  abort "result digest mismatch" unless Digest::SHA256.file(result_path).hexdigest == row.fetch("result_sha256")
  stdout, stderr, status = Open3.capture3("gh", "api", "repos/agent-logic/agent-design-language/issues/#{row.fetch("issue")}")
  abort "live read failed: #{stderr}" unless status.success?
  live = JSON.parse(stdout)
  labels = live.fetch("labels").map { |label| label.fetch("name") }
  body = live.fetch("body")
  abort "live identity mismatch" unless live.fetch("state") == "open" && live.fetch("title") == row.fetch("title") &&
                                         live.dig("milestone", "number") == 1 && labels.include?("version:v0.92.1")
  abort "live version/reason mismatch" unless body.include?("Membership version: `#{membership_version}`") && body.include?(reason)
  member_block = body[/## Initial child membership baseline\n\n(.*?)\n\n- Membership version:/m, 1]
  abort "missing live membership block" unless member_block
  live_members = member_block.scan(/^- #(\d+)$/).flatten.map(&:to_i)
  abort "live membership mismatch" unless live_members == row.fetch("members")
end

puts JSON.generate(schema: "adl.v0921.wp01.sprint-umbrella-validation.v1", result: "passed",
                   membership_version: membership_version, umbrellas: 11, unique_members: all_members.length)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "fileutils"

ROOT = File.expand_path("../../../..", __dir__)
TARGETS = {
  536 => ["ruby", ".csdlc/prepared/issues/536/validate-sprint-readiness.rb"],
  51 => ["ruby", ".csdlc/prepared/issues/51/validate-podcast-coordination.rb"],
  261 => ["ruby", ".csdlc/prepared/issues/261/validate-show-identity.rb"],
  262 => ["ruby", ".csdlc/prepared/issues/262/validate-podcast-hosting.rb"],
  263 => ["ruby", ".csdlc/prepared/issues/263/validate-directory-runbooks.rb"],
  264 => ["ruby", ".csdlc/prepared/issues/264/validate-directory-submissions.rb"],
  342 => ["bash", "adl/tools/test_podcast_launch_packet.sh"],
  511 => ["ruby", ".csdlc/prepared/issues/511/validate-observatory-experience.rb"],
  84 => ["bash", "adl/tools/validate_v092_unity_observatory_live.sh"],
  512 => ["bash", "adl/tools/validate_layer8_authority_observatory_ui.sh"]
}.freeze

mode = ARGV.fetch(0)
abort "mode must be affected or lanes" unless %w[affected lanes].include?(mode)

TARGETS.each do |issue, argv|
  index_path = File.join(ROOT, ".csdlc/issues/#{issue}/index.json")
  index = JSON.parse(File.read(index_path))
  dir = File.join(ROOT, ".csdlc/prepared/issues/#{issue}")
  values = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues/#{issue}/cards/spp.values.json")))
  affected = values.fetch("content").fetch("values").fetch("affected_areas")
  target = argv.fetch(1)

  request = {
    issue: issue,
    card: mode == "affected" ? "spp" : "vpp",
    expected_generation: index.fetch("generation"),
    expected_digest: index.fetch("digest"),
    actor: "codex:sprint8-readiness",
    reason: mode == "affected" ? "Add the issue-owned validator to the exact readiness denominator." : "Use an explicit governed interpreter argv for the issue-owned proof target."
  }

  if mode == "affected"
    request[:operation] = {
      operation: "replace_planning_collection",
      field: "affected_areas",
      values: (affected + [target]).uniq
    }
  else
    next if issue == 536
    vpp = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues/#{issue}/cards/vpp.values.json")))
    lanes = vpp.fetch("content").fetch("values").fetch("lanes")
    lanes.fetch(0)["argv"] = argv
    request[:operation] = { operation: "replace_validation_lanes", lanes: lanes }
  end

  File.write(File.join(dir, "readiness-#{mode}-edit.json"), JSON.pretty_generate(request) + "\n")
end

puts JSON.generate({ status: "generated", mode: mode, issues: TARGETS.keys.sort })

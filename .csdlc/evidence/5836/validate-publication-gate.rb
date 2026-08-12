#!/usr/bin/env ruby
require "json"
require "pathname"

abort "usage: validate-publication-gate.rb --check-only" unless ARGV == ["--check-only"]
root = Pathname.new(__dir__).join("../../..").cleanpath
required = [
  "demos/v0.92/first-birthday/positive.json",
  "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
  "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
  "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
  "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
]
missing = required.reject { |path| root.join(path).file? }
abort "publication gate missing: #{missing.join(', ')}" unless missing.empty?
packet = JSON.parse(root.join(required.first).read)
abort "publication gate requires an accepted complete packet" unless packet["status"] == "complete" && packet.dig("decision", "accepted")
abort "publication remains operator-gated"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[live-manifest live-two-shard baseline-comparison post-merge-exact].freeze
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

if lane == "live-manifest"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  manifest = root.join(".csdlc/evidence/5501/live-run-manifest.json")
  validator = Pathname.new(__dir__).join("validate-live-run-manifest.rb")
  exec("ruby", validator.to_s, manifest.to_s)
end

warn("#{lane} is execution-gated and has no live proof in the preparation-only packet")
puts JSON.pretty_generate(
  status: "blocked",
  lane: lane,
  reason: "#5349, #5499, #5498, #5500, and #5502 must be merged and typed closed_out"
)
exit 2

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = File.join(ROOT, ".csdlc/issues/5347")
PREP = File.join(ROOT, ".csdlc/prepared/issues/5347")

def fail!(message)
  warn("#5347 preparation validation failed: #{message}")
  exit(1)
end

required = %w[
  design.md
  diagram.mmd
  bootstrap-request.json
  bind-request.json
  check-dependencies.rb
  run-validation-lane.rb
  validate-blocked-state.rb
  verify-terminal-receipt.rb
  receipt-verifier/Cargo.toml
  receipt-verifier/Cargo.lock
  receipt-verifier/src/main.rs
  validation-request.json
]
required.each { |name| fail!("missing #{name}") unless File.file?(File.join(PREP, name)) }

cards = %w[sip stp spp vpp srp sor]
cards.each do |card|
  fail!("missing #{card}.md") unless File.file?(File.join(ISSUE, "cards/#{card}.md"))
  fail!("missing #{card}.values.json") unless File.file?(File.join(ISSUE, "cards/#{card}.values.json"))
end

request = JSON.parse(File.read(File.join(PREP, "bootstrap-request.json")))
fail!("wrong issue") unless request["issue"] == 5347
initial = request.fetch("initial")
design = File.read(File.join(PREP, "design.md"))
diagram = File.read(File.join(PREP, "diagram.mmd"))
text = JSON.generate(initial) + design

%w[#5346 #5344 #5343 #5358 #5361].each do |dependency|
  fail!("missing dependency #{dependency}") unless text.include?(dependency)
end

[
  "closed_out", "retained merged receipt", "ancestral", "claim",
  "dependency cycle", "zero canonical path overlap", "authority-rooted",
  "delete_external", "retain_owned", "retain_evidence", "handoff_to_5346",
  "Runtime v2", "net source change is negative", "no deferred acceptance"
].each do |term|
  fail!("missing contract term #{term.inspect}") unless text.include?(term)
end

claim_paths = request.fetch("claim").fetch("protected_paths")
allowed = [
  ".csdlc/issues/5347",
  ".csdlc/locks/5347.lock",
  ".csdlc/prepared/issues/5347",
  ".csdlc/evidence/5347",
  "docs/milestones/v0.91.8/evidence/wp13-external-bands"
]
fail!("preparation claim contains product paths") unless claim_paths.sort == allowed.sort

lanes = initial.fetch("validation_lanes")
expected_lanes = %w[
  preparation-contract
  dependency-terminal-gate
  manifest-disjointness
  owner-and-consumer-proof
  deletion-budgets-and-evidence
  post-deletion-exact
]
fail!("validation lane set mismatch") unless lanes.map { |lane| lane["lane"] }.sort == expected_lanes.sort
future_lanes = lanes.reject { |lane| lane["lane"] == "preparation-contract" }
future_lanes.each do |lane|
  reason = lane["defer_reason"].to_s
  fail!("#{lane['lane']} lacks a mandatory admission condition") unless reason.include?("Mandatory") || reason.include?("expected to fail") || reason.include?("mandatory before")
  fail!("#{lane['lane']} permits optional/skipped/deferred acceptance") if reason.match?(/optional|may skip|deferred acceptance/i)
end

fail!("design omits dependency cycle") unless design.include?("dependency cycle")
fail!("design omits typed claim amendment") unless design.include?("typed protected-path claim amendment")
fail!("diagram omits fail-closed route") unless diagram.include?("Fail closed")
fail!("diagram omits #5346 boundary") unless diagram.include?("#5346")

vpp = JSON.parse(File.read(File.join(ISSUE, "cards/vpp.values.json"))).dig("content", "values", "lanes")
fail!("VPP lane contract differs from bootstrap") unless vpp == lanes

status_output, status = Open3.capture2("git", "-C", ROOT, "status", "--porcelain")
fail!("git status failed") unless status.success?
status_output.lines.each do |line|
  path = line.sub(/\A.. /, "").strip
  next if path.start_with?(".csdlc/issues/5347/", ".csdlc/prepared/issues/5347/", ".csdlc/evidence/5347/")
  next if path == ".csdlc/locks/5347.lock"

  fail!("out-of-scope changed path #{path}")
end


puts(JSON.generate({schema: "adl.wp13.external_band_preparation.v1", issue: 5347, status: "pass", cards: cards.length, product_changes: 0}))

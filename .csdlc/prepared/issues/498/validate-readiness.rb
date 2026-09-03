#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../../").expand_path
ISSUE = "498"
PREP = ROOT.join(".csdlc/prepared/issues/#{ISSUE}")
CORP_D = ROOT.join("docs/milestones/v0.92.1/evidence/corporate/corp-d")
OPS = ROOT.join("docs/operations/corporate/diligence")

failures = []

%w[design.md diagram.mmd bootstrap-request.json bind-request.json validate-readiness.rb check-prerequisites.rb validate-diligence-index.rb validate-counsel-boundary.rb validate-acceptance-readback.rb].each do |name|
  failures << "missing prepared file #{name}" unless PREP.join(name).file?
end

bootstrap = JSON.parse(PREP.join("bootstrap-request.json").read)
initial = bootstrap.fetch("initial")

failures << "wrong issue" unless bootstrap.fetch("issue") == 498
failures << "wrong repository" unless bootstrap.fetch("repository") == "agent-logic/agent-design-language"
failures << "design is not approved for execution readiness" unless bootstrap["design_approved"] == true

declared_scope = initial.fetch("declared_scope")
[
  "docs/operations/corporate/diligence/**",
  "docs/milestones/v0.92.1/evidence/corporate/corp-d/**",
  ".csdlc/prepared/issues/498/**",
  ".csdlc/evidence/498/**"
].each do |scope|
  failures << "missing declared scope #{scope}" unless declared_scope.include?(scope)
end

acceptance = initial.fetch("acceptance_criteria").join("\n")
failures << "missing live merged ancestry acceptance" unless acceptance.include?("live merged into main and ancestral")
failures << "missing no-private-material acceptance" unless acceptance.include?("No private advice")

dependencies = initial.fetch("dependencies").join("\n")
failures << "missing CORP-C fail-closed dependency" unless dependencies.include?("CORP-C #497") && dependencies.include?("fail closed")

lanes = initial.fetch("validation_lanes")
lane_names = lanes.map { |lane| lane.fetch("lane") }
%w[readiness-preparation prerequisite-census diligence-index counsel-boundary acceptance-readback diff-hygiene].each do |lane|
  failures << "missing validation lane #{lane}" unless lane_names.include?(lane)
end
prereq_lane = lanes.find { |lane| lane.fetch("lane") == "prerequisite-census" }
failures << "prerequisite lane should defer until execution" unless prereq_lane && prereq_lane["defer_reason"].to_s.include?("#497")

text = [
  PREP.join("design.md").read,
  PREP.join("diagram.mmd").read,
  PREP.join("bootstrap-request.json").read
].join("\n")
[
  /password\s*[:=]/i,
  /aws_secret_access_key\s*[:=]/i,
  /access_key\s*[:=]/i,
  /private_key\s*[:=]/i,
  /BEGIN .*PRIVATE KEY/
].each do |pattern|
  failures << "forbidden sensitive token shape appears in prepared packet: #{pattern.inspect}" if text.match?(pattern)
end

failures << "missing corp-d evidence directory" unless CORP_D.directory?
failures << "missing diligence operations directory" unless OPS.directory?

if failures.any?
  warn(JSON.pretty_generate({schema: "adl.issue498.readiness.v1", status: "fail", failures: failures}))
  exit 1
end

puts JSON.pretty_generate({
  schema: "adl.issue498.readiness.v1",
  status: "pass",
  issue: 498,
  ready_for_execution_binding: true,
  execution_gate: "CORP-C #497 must be live merged into main and ancestral before acceptance."
})

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../../").expand_path
ACCEPTANCE = ROOT.join("docs/milestones/v0.92.1/evidence/corporate/corp-d/corporate-diligence-acceptance.v1.json")
INDEX = ROOT.join("docs/operations/corporate/diligence/diligence-index.v1.json")

failures = []
failures << "missing acceptance record" unless ACCEPTANCE.file?
failures << "missing diligence index" unless INDEX.file?

if ACCEPTANCE.file?
  record = JSON.parse(ACCEPTANCE.read)
  failures << "record must name issue 498" unless record["issue"] == 498
  failures << "record must bind CORP-D" unless record["planned_id"] == "CORP-D"
  failures << "record must list prerequisite census result" unless record["prerequisite_census_status"]
  failures << "accepted record cannot have unresolved blockers" if record["status"] == "accepted" && !record.fetch("unresolved_blockers", []).empty?
  failures << "blocked record must name unresolved blockers" if record["status"] == "blocked" && record.fetch("unresolved_blockers", []).empty?
end

if failures.any?
  warn(JSON.pretty_generate({schema: "adl.issue498.acceptance_readback.v1", status: "fail", failures: failures}))
  exit 1
end

puts JSON.pretty_generate({schema: "adl.issue498.acceptance_readback.v1", status: "pass"})

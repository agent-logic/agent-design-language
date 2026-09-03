#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "digest"

ROOT = Pathname.new(__dir__).join("../../../../").expand_path
INDEX = ROOT.join("docs/operations/corporate/diligence/diligence-index.v1.json")
ACCEPTANCE = ROOT.join("docs/milestones/v0.92.1/evidence/corporate/corp-d/corporate-diligence-acceptance.v1.json")

failures = []
failures << "missing diligence index #{INDEX.relative_path_from(ROOT)}" unless INDEX.file?
failures << "missing acceptance record #{ACCEPTANCE.relative_path_from(ROOT)}" unless ACCEPTANCE.file?

if failures.empty?
  index = JSON.parse(INDEX.read)
  acceptance = JSON.parse(ACCEPTANCE.read)
  expected = %w[CORP-A CORP-B CORP-C]
  rows = index.fetch("entries")
  ids = rows.map { |row| row.fetch("planned_id") }
  failures << "diligence index planned IDs mismatch" unless ids.sort == expected.sort
  actual_digest = Digest::SHA256.file(INDEX.to_s).hexdigest
  failures << "acceptance record does not bind exact diligence index digest" unless acceptance["diligence_index_sha256"] == actual_digest
  failures << "acceptance status must be accepted or blocked" unless %w[accepted blocked].include?(acceptance["status"])
end

if failures.any?
  warn(JSON.pretty_generate({schema: "adl.issue498.diligence_index.v1", status: "fail", failures: failures}))
  exit 1
end

puts JSON.pretty_generate({schema: "adl.issue498.diligence_index.v1", status: "pass"})

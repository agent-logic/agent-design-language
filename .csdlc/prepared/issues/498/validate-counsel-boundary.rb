#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../../").expand_path
RECEIPTS = ROOT.join("docs/milestones/v0.92.1/evidence/corporate/corp-d/counsel-boundary-receipts.v1.json")

failures = []
failures << "missing counsel-boundary receipt register" unless RECEIPTS.file?

if RECEIPTS.file?
  receipts = JSON.parse(RECEIPTS.read)
  receipts.fetch("receipts").each_with_index do |receipt, index|
    visibility = receipt.fetch("visibility")
    failures << "receipt #{index} visibility must be public or redacted" unless %w[public redacted].include?(visibility)
    failures << "receipt #{index} must not include advice_text" if receipt.key?("advice_text")
    failures << "receipt #{index} missing receipt_ref" if receipt.fetch("receipt_ref", "").empty?
  end
end

scan_paths = [
  ROOT.join("docs/operations/corporate/diligence"),
  ROOT.join("docs/milestones/v0.92.1/evidence/corporate/corp-d")
]
forbidden = [/private advice/i, /legal advice content/i, /BEGIN .*PRIVATE KEY/, /aws_secret_access_key/i, /password\s*[:=]/i]
scan_paths.each do |path|
  next unless path.directory?
  Dir.glob(path.join("**/*")).each do |file|
    next unless File.file?(file)
    content = File.read(file)
    forbidden.each do |pattern|
      failures << "forbidden private/sensitive pattern #{pattern.inspect} in #{Pathname.new(file).relative_path_from(ROOT)}" if content.match?(pattern)
    end
  end
end

if failures.any?
  warn(JSON.pretty_generate({schema: "adl.issue498.counsel_boundary.v1", status: "fail", failures: failures}))
  exit 1
end

puts JSON.pretty_generate({schema: "adl.issue498.counsel_boundary.v1", status: "pass"})

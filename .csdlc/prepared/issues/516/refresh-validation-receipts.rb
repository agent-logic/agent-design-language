#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").realpath
validator = ".csdlc/prepared/issues/516/validate-release-tail-admission.rb"
destinations = {
  "denominator" => ".csdlc/evidence/516/release-tail-denominator.log",
  "gaps" => ".csdlc/evidence/516/implementation-gap-analysis.log",
  "decision" => ".csdlc/evidence/516/admission-consistency.log"
}.freeze

destinations.each do |mode, relative_path|
  stdout, stderr, status = Open3.capture3(
    { "ADL_RECORD_VALIDATION_RECEIPT" => "1" },
    "ruby", validator, mode,
    chdir: root.to_s
  )
  abort "#{mode} validation failed: #{stderr}" unless status.success?
  receipt = JSON.parse(stdout)
  abort "#{mode} receipt mode mismatch" unless receipt.dig("stdout", "mode") == mode
  root.join(relative_path).write(JSON.generate(receipt) + "\n")
end

puts JSON.generate(
  schema: "adl.v0921.release_tail_receipt_refresh.v1",
  status: "pass",
  receipts: destinations.values
)

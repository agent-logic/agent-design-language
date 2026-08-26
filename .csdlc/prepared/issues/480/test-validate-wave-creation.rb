#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
VALIDATOR = File.join(__dir__, "validate-wave-creation.rb")

stdout, stderr, status = Open3.capture3("ruby", VALIDATOR, "plan", chdir: ROOT)
abort "plan validator failed: #{stderr}" unless status.success?
packet = JSON.parse(stdout)
abort "wrong creation denominator" unless packet["creation_slots"] == 45
abort "duplicate ordered IDs" unless packet.fetch("ordered_ids").uniq.length == 45
abort "excluded #269 missing" unless packet.fetch("excluded_issues") == [269]

_live_stdout, live_stderr, live_status = Open3.capture3("ruby", VALIDATOR, "live", chdir: ROOT)
if !File.file?(File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"))
  abort "live validation did not fail closed without a final receipt" if live_status.success?
  abort "missing fail-closed diagnostic" unless live_stderr.include?("final creation receipt is absent")
end

puts JSON.generate(schema: "adl.v0921.wp01.validator-smoke.v1", result: "passed", cases: 4)

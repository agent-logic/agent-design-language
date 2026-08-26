#!/usr/bin/env ruby
# frozen_string_literal: true

require "English"

ROOT = File.expand_path("../../../..", __dir__)
BASELINE_RESILIENCE_RS_LOC = 5_278
facade = File.join(ROOT, "adl/src/resilience.rs")
modules = Dir[File.join(ROOT, "adl/src/resilience/*.rs")].sort
facade_loc = File.readlines(facade).size

abort "RUST-01 validation-impact failed: no extracted resilience modules found" if modules.empty?
if facade_loc >= BASELINE_RESILIENCE_RS_LOC
  abort "RUST-01 validation-impact failed: resilience.rs remained #{facade_loc} lines"
end

report_cmd = ["bash", "adl/tools/report_large_rust_modules.sh", "--format", "tsv"]
report = IO.popen(report_cmd, chdir: ROOT, err: [:child, :out], &:read)
unless $CHILD_STATUS.success?
  warn report
  abort "RUST-01 validation-impact failed: large module report failed"
end

resilience_rationale = report.lines.any? do |line|
  path, _loc, status = line.split("\t")
  path == "adl/src/resilience.rs" && status&.include?("RATIONALE")
end
abort "RUST-01 validation-impact failed: facade remains on large-module rationale watchlist" if resilience_rationale

puts report
puts "RUST-01 validation-impact passed: adl/src/resilience.rs #{facade_loc} lines; #{modules.size} extracted modules"

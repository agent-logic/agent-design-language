#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
VALIDATOR = File.join(__dir__, "validate-wave-creation.rb")
EXECUTOR = File.join(__dir__, "execute-wave-creation.rb")

stdout, stderr, status = Open3.capture3("ruby", VALIDATOR, "plan", chdir: ROOT)
abort "plan validator failed: #{stderr}" unless status.success?
packet = JSON.parse(stdout)
abort "wrong creation denominator" unless packet["creation_slots"] == 45
abort "duplicate ordered IDs" unless packet.fetch("ordered_ids").uniq.length == 45
abort "excluded #269 missing" unless packet.fetch("excluded_issues") == [269]

plan_stdout, plan_stderr, plan_status = Open3.capture3("ruby", EXECUTOR, "plan", chdir: ROOT)
abort "executor plan failed: #{plan_stderr}" unless plan_status.success?
creation_plan = JSON.parse(plan_stdout)
children = creation_plan.fetch("children")
abort "executor denominator mismatch" unless children.length == 45
abort "executor operation keys are not unique" unless children.map { |row| row.fetch("operation_key") }.uniq.length == 45
abort "executor operation key is not portable" unless children.all? { |row| row.fetch("operation_key").match?(/\Av0921-wp01-[a-f0-9]{64}-[a-z0-9-]+-create\z/) }
abort "executor title contract mismatch" unless children.all? { |row| row.fetch("title").start_with?("[v0.92.1][#{row.fetch('planned_id')}] ") }
abort "executor routing contract mismatch" unless children.all? do |row|
  labels = row.fetch("labels")
  labels.include?("version:v0.92.1") && labels.include?("track:roadmap") && labels.include?("type:task") && labels.one? { |label| label.start_with?("area:") }
end

_order_stdout, order_stderr, order_status = Open3.capture3("ruby", EXECUTOR, "create", "CORP-B", chdir: ROOT)
abort "out-of-order create did not fail closed" if order_status.success?
abort "out-of-order diagnostic missing" unless order_stderr.include?("out-of-order create")

_unknown_stdout, unknown_stderr, unknown_status = Open3.capture3("ruby", EXECUTOR, "create", "UNKNOWN", chdir: ROOT)
abort "unknown create did not fail closed" if unknown_status.success?
abort "unknown-ID diagnostic missing" unless unknown_stderr.include?("unknown planned ID")

_final_stdout, final_stderr, final_status = Open3.capture3("ruby", EXECUTOR, "finalize", chdir: ROOT)
abort "incomplete finalization did not fail closed" if final_status.success?
abort "incomplete-finalization diagnostic missing" unless final_stderr.include?("children absent")

_live_stdout, live_stderr, live_status = Open3.capture3("ruby", VALIDATOR, "live", chdir: ROOT)
if !File.file?(File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"))
  abort "live validation did not fail closed without a final receipt" if live_status.success?
  abort "missing fail-closed diagnostic" unless live_stderr.include?("final creation receipt is absent")
end

puts JSON.generate(schema: "adl.v0921.wp01.validator-smoke.v1", result: "passed", cases: 12)

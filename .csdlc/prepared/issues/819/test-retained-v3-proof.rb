#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"

SOURCE = ".csdlc/evidence/819/retained-v3/reconciliation.json"
TMP = ".csdlc/evidence/819/retained-v3-negative-tmp"
VALIDATOR = ".csdlc/prepared/issues/819/validate-retained-v3-proof.rb"

def mutate(document, name)
  copy = Marshal.load(Marshal.dump(document))
  yield copy
  path = File.join(TMP, "#{name}.json")
  File.write(path, JSON.pretty_generate(copy) + "\n")
  _output, status = Open3.capture2e({"ISSUE819_RECEIPT" => path}, "ruby", VALIDATOR)
  raise "negative fixture passed: #{name}" if status.success?
end

FileUtils.rm_rf(TMP)
FileUtils.mkdir_p(TMP)
document = JSON.parse(File.read(SOURCE))

mutate(document, "missing-row") { |copy| copy.fetch("rows").pop }
mutate(document, "duplicate-row") { |copy| copy.fetch("rows")[-1] = copy.fetch("rows").first }
mutate(document, "unclassified") { |copy| copy["unclassified"] = 1 }
mutate(document, "failed-command") { |copy| copy.fetch("commands").first["status"] = "failed" }
mutate(document, "command-argv") { |copy| copy.fetch("commands").first["argv"] << "--ignored" }
mutate(document, "stale-evidence") { |copy| copy.fetch("rows").first.fetch("candidate_evidence").first["sha256"] = "0" * 64 }
mutate(document, "empty-evidence") { |copy| copy.fetch("rows").first["candidate_evidence"] = [] }
mutate(document, "wrong-candidate") { |copy| copy["candidate"] = "0" * 40 }
mutate(document, "surface-digest") { |copy| copy["candidate_surface_tree_sha256"] = "0" * 64 }
mutate(document, "resolution-drift") { |copy| copy.fetch("rows").first.fetch("resolution")["criterion_specific_basis"] = "unrelated claim" }
amendment_index = document.fetch("rows").index { |row| row.dig("resolution", "type") == "governed_amendment_proposal" }
mutate(document, "synthetic-pass-amendment") { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["behavioral_pass_claim"] = true }
mutate(document, "fabricated-amendment-approval") { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["status"] = "approved" }
mutate(document, "premature-release") { |copy| copy["release_ready"] = true }

FileUtils.rm_rf(TMP)
puts "PASS issue #819 negative matrix: 13/13 invalid receipts rejected"

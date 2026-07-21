#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ISSUE = 5589
KINDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_ACCEPTANCE = (1..8).map { |number| "AC-#{number}" }.freeze
EXPECTED_CLAIM_PATHS = [
  ".csdlc/evidence/5589",
  ".csdlc/issues/5589",
  ".csdlc/locks/5589.lock",
  ".csdlc/prepared/issues/5589"
].freeze

def run_git(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  abort "git #{args.join(' ')} failed: #{stderr}" unless status.success?
  stdout
end

index = JSON.parse(File.read(".csdlc/issues/#{ISSUE}/index.json"))
abort "issue is not preparation-bound" unless index.fetch("phase") == "bound"
abort "issue generation omits preparation semantic edits" unless index.fetch("generation") >= 3
abort "claim generation mismatch" unless index.dig("claim", "generation") == index.fetch("generation")
abort "design review missing" unless index.fetch("design_review").key?("approved")
abort "prepublication review must remain absent" if index["review"] || index["review_assignment"]

claim_paths = index.dig("claim", "protected_paths")
abort "preparation claim paths differ" unless claim_paths == EXPECTED_CLAIM_PATHS
abort "product path entered preparation claim" if claim_paths.any? { |path| path.start_with?("adl-runtime") }

cards = KINDS.to_h do |kind|
  path = ".csdlc/issues/#{ISSUE}/cards/#{kind}.values.json"
  card = JSON.parse(File.read(path))
  abort "#{kind} identity mismatch" unless card.dig("identity", "issue") == ISSUE
  [kind, card.fetch("content").fetch("values")]
end
abort "expected all six cards" unless cards.length == 6

acceptance = cards.fetch("stp").fetch("acceptance_criteria").map { |value| value[/AC-\d+/] }.compact.uniq.sort
abort "acceptance set incomplete" unless acceptance == EXPECTED_ACCEPTANCE

step_coverage = cards.fetch("spp").fetch("steps").flat_map { |step| step.fetch("acceptance_ids") }.uniq.sort
lane_coverage = cards.fetch("vpp").fetch("lanes").flat_map { |lane| lane.fetch("acceptance_ids") }.uniq.sort
abort "SPP coverage incomplete" unless step_coverage == EXPECTED_ACCEPTANCE
abort "VPP coverage incomplete" unless lane_coverage == EXPECTED_ACCEPTANCE
abort "deferred validation lane" if cards.fetch("vpp").fetch("lanes").any? { |lane| lane["defer_reason"] }
abort "validation time exceeds large profile" unless cards.fetch("vpp").fetch("planned_validation_seconds") <= 7_200
abort "validation tokens exceed large profile" unless cards.fetch("vpp").fetch("planned_validation_tokens") <= 50_000

design = File.read(".csdlc/prepared/issues/5589/design.md")
diagram = File.read(".csdlc/prepared/issues/5589/diagram.mmd")
matrix = File.read(".csdlc/prepared/issues/5589/adapter-authority-matrix.md")
%w[Freedom\ Gate AEE delegation provider scheduler identity private checkpoint lifelog].each do |term|
  normalized = term.tr("\\", "")
  abort "design missing #{normalized}" unless design.downcase.include?(normalized.downcase)
end
abort "design omits zero-credit rule" unless design.include?("zero parity credit")
abort "diagram omits gate-before-actuation flow" unless %w[identity delegation gate scheduler checkpoint lifelog].all? { |term| diagram.downcase.include?(term) }
abort "adapter matrix is incomplete" unless matrix.scan(/^\| [^|]+ \|/).length >= 10
abort "matrix omits degraded zero-credit rule" unless matrix.include?("zero parity credit")

status_paths = run_git("status", "--porcelain=v1").lines.map { |line| line[3..].strip }
allowed = status_paths.all? do |path|
  path.start_with?(".csdlc/issues/5589", ".csdlc/locks/5589.lock", ".csdlc/prepared/issues/5589", ".csdlc/evidence/5589")
end
abort "preparation changed a non-#5589 path: #{status_paths.join(', ')}" unless allowed

parity_a_ref = "codex/5591-runtime-v3-parity-a-preparation"
parity_a_revision = run_git("rev-parse", parity_a_ref).strip
parity_a = JSON.parse(run_git("show", "#{parity_a_ref}:.csdlc/issues/5591/index.json"))
clean_review = %w[reviewed published merge_ready merged closed_out].include?(parity_a.fetch("phase")) &&
  parity_a["review"] && parity_a.dig("review", "result") == "pass"
abort "#5591 unexpectedly satisfies the implementation gate; replan against its reviewed contract" if clean_review
abort "#5591 blocker is not concrete" unless parity_a.fetch("phase") == "bound" && parity_a["review"].nil?

puts "cards=6 acceptance=8 spp=complete vpp=complete deferrals=0 claim=preparation-only product_changes=0"
puts "parity_a_revision=#{parity_a_revision} parity_a_phase=#{parity_a.fetch('phase')} parity_a_review=absent implementation=blocked"

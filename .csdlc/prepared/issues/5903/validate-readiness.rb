#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "open3"
require "pathname"
require "set"

ROOT = File.expand_path("../../../..", __dir__)
SPRINT_ISSUES = [5857, 5825, 5826, 5827, 5828, 5829, 5830, 5831, 5833, 5834].freeze
CHILD_ISSUES = SPRINT_ISSUES.drop(1).freeze
PREREQUISITES = [5817, 5818, 5819, 5801].freeze
MERGED_READINESS_SHA = "51f4e00a32176a7d2fb9388997da8448d8e3d4f2"

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  raise "command failed: #{argv.join(' ')}\n#{stderr}" unless status.success?
  stdout
end

def read_json(path)
  JSON.parse(File.read(File.join(ROOT, path)))
end

capture!("git", "merge-base", "--is-ancestor", MERGED_READINESS_SHA, "HEAD")
source_head = capture!("git", "rev-parse", "HEAD").strip

doctor = File.join(ROOT, "csdlc-v2/target/debug/csdlc-doctor")
capture!("cargo", "build", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--bin", "csdlc-doctor")
raise "source-built doctor is unavailable" unless File.executable?(doctor)
doctor_sha256 = Digest::SHA256.file(doctor).hexdigest

manifest = read_json(".csdlc/prepared/issues/5903/serialization-gates.json")
raise "serialization manifest schema mismatch" unless manifest.fetch("schema") == "csdlc.sprint_serialization_gate_manifest.v1"
raise "serialization manifest denominator mismatch" unless manifest.fetch("issues").map { |entry| entry.fetch("issue") } == CHILD_ISSUES

manifest.fetch("issues").each do |entry|
  issue = entry.fetch("issue")
  baseline = JSON.parse(capture!("git", "show", "#{MERGED_READINESS_SHA}:.csdlc/issues/#{issue}/cards/spp.values.json"))
    .fetch("content").fetch("values").fetch("affected_areas")
    .select { |value| value.start_with?("SERIALIZATION_GATE ") }
  spp = read_json(".csdlc/issues/#{issue}/cards/spp.values.json").fetch("content").fetch("values")
  affected = spp.fetch("affected_areas")
  raise "#{issue} has no owned paths" if affected.empty?
  affected.each do |path|
    relative = Pathname.new(path)
    components = relative.each_filename.to_a
    raise "#{issue} has invalid owned path: #{path}" if relative.absolute? || components.empty? || components.any? { |part| part == "." || part == ".." } || path.start_with?("SERIALIZATION_GATE ")
  end
  actual_gates = spp.fetch("replan_triggers").select { |value| value.start_with?("SERIALIZATION_GATE ") }
  raise "#{issue} baseline-to-manifest serialization-gate mismatch" unless baseline.sort == entry.fetch("gates").sort
  raise "#{issue} serialization-gate parity mismatch" unless actual_gates.sort == entry.fetch("gates").sort
end

doctor_reports = SPRINT_ISSUES.map do |issue|
  report = JSON.parse(capture!(doctor, "--repo", ROOT, "--issue", issue.to_s))
  raise "doctor blocked #{issue}: #{report.fetch('findings')}" unless report.fetch("status") == "pass" && report.fetch("ready") == true
  report
end

operator_paths = [
  ".adl/docs/TBD/V092_SPRINT_5857_BIRTHDAY_CORE_SESSION_PROMPT.md",
  ".csdlc/prepared/issues/5857/sprint-execution-packet.md",
  ".csdlc/prepared/issues/5857/sprint-execution-packet.yaml"
]
banned = ["--reacquire-request", "reacquires the exact issue-local claim", "publication claim"]
operator_paths.each do |path|
  text = File.read(File.join(ROOT, path))
  banned.each { |term| raise "retired operator instruction remains in #{path}: #{term}" if text.include?(term) }
end

dependency_observations = PREREQUISITES.map do |issue|
  value = JSON.parse(capture!("gh", "issue", "view", issue.to_s, "--repo", "danielbaustin/agent-design-language", "--json", "number,state,closedAt,url"))
  raise "prerequisite #{issue} is not closed" unless value.fetch("state") == "CLOSED" && value.fetch("closedAt")
  value
end

changed = Set.new
capture!("git", "diff", "--name-only", "origin/main...HEAD").lines.each { |line| changed << line.strip unless line.strip.empty? }
capture!("git", "diff", "--name-only").lines.each { |line| changed << line.strip unless line.strip.empty? }
capture!("git", "ls-files", "--others", "--exclude-standard").lines.each do |line|
  path = line.strip
  changed << path unless path.empty? || path.start_with?(".csdlc/locks/")
end
allowed_exact = Set[
  ".adl/docs/TBD/V092_SPRINT_5857_BIRTHDAY_CORE_SESSION_PROMPT.md",
  ".csdlc/prepared/issues/5857/sprint-execution-packet.md",
  ".csdlc/prepared/issues/5857/sprint-execution-packet.yaml"
]
allowed_prefixes = (SPRINT_ISSUES + [5903]).map { |issue| ".csdlc/issues/#{issue}/" } + [
  ".csdlc/prepared/issues/5857/",
  ".csdlc/prepared/issues/5903/",
  ".csdlc/evidence/5903/"
]
unexpected = changed.reject { |path| allowed_exact.include?(path) || allowed_prefixes.any? { |prefix| path.start_with?(prefix) } }
raise "unexpected changed paths: #{unexpected.to_a.sort.join(', ')}" unless unexpected.empty?

puts JSON.generate(
  schema: "csdlc.sprint4_readiness_validation.v1",
  source_head: source_head,
  readiness_base: MERGED_READINESS_SHA,
  doctor_binary_sha256: doctor_sha256,
  sprint_issue: 5857,
  issues: SPRINT_ISSUES,
  doctor_generations: doctor_reports.to_h { |report| [report.fetch("issue"), report.fetch("generation")] },
  serialization_gate_occurrences: manifest.fetch("issues").sum { |entry| entry.fetch("gates").length },
  serialization_gate_ids: manifest.fetch("issues").flat_map { |entry| entry.fetch("gates") }.map { |gate| JSON.parse(gate.delete_prefix("SERIALIZATION_GATE ")).fetch("id") }.uniq.sort,
  dependencies: dependency_observations,
  changed_paths: changed.to_a.sort,
  result: "pass"
)

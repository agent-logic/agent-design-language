#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").expand_path
FEATURE = ROOT.join("docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md")
DESIGN = ROOT.join("docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md")
HANDOFF = ROOT.join("docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md")
DEPENDENCIES = ROOT.join(".csdlc/evidence/5835/dependency-authority.json")
REVIEW_INVENTORY = ROOT.join("docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json")
SPRINT_REVIEW = ROOT.join(".csdlc/evidence/5857/sprint-review.json")

EXPECTED_ROWS = [
  "Stable name", "Identity root", "Continuity head", "Memory-grounding references",
  "Capability envelope", "Cognitive profile", "Adaptive-learning history",
  "ACIP transport-readiness proof", "Witness set", "Citizen-facing receipt",
  "WP-16 review inventory"
].freeze

EXPECTED_SCHEMAS = [
  "adl.birthday.identity_record.v2", "adl.birthday.continuity_record.v1",
  "adl.memory_palace.context_packet.v1", "adl.capability_envelope.v1",
  "adl.cognitive_profile.v1", "adl.cognitive_profile.public.v1",
  "adl.adaptive_learning.history.v1", "adl.acip_native_platform_proof.v2",
  "adl.birth_witness.set.v1", "adl.birth_witness.citizen_receipt.v1",
  "adl.v092.first-birthday-review-evidence.v1"
].freeze

REQUIRED_PATHS = %w[
  adl-runtime-kernel/src/birthday_identity.rs
  adl-runtime-kernel/src/birthday_continuity.rs
  adl-runtime-kernel/src/memory_palace.rs
  adl-runtime-kernel/src/capability_envelope.rs
  adl-runtime-kernel/src/cognitive_profile.rs
  adl-runtime-kernel/src/adaptive_learning.rs
  adl-runtime-kernel/src/acip.rs
  adl-runtime-kernel/src/birth_witness.rs
  docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json
].freeze

FORBIDDEN_CLAIMS = [
  "production migration is implemented", "cross-polis federation is complete",
  "grants citizenship", "proves legal personhood", "raw private state may move",
  "public schema grants content access"
].freeze

def assert(condition, code, detail)
  raise "#{code}: #{detail}" unless condition
end

def validate_docs(feature, design, handoff)
  errors = []
  check = ->(condition, code, detail) { errors << "#{code}: #{detail}" unless condition }
  EXPECTED_ROWS.each do |row|
    check.call(feature.scan(/^\| #{Regexp.escape(row)} \|/).length == 1, "matrix_row", row)
  end
  check.call(feature.scan(/^\| (?!---)/).length == EXPECTED_ROWS.length + 1,
             "matrix_size", "expected header plus #{EXPECTED_ROWS.length} rows")
  EXPECTED_SCHEMAS.each do |schema|
    check.call(feature.include?("`#{schema}`"), "schema_missing", schema)
  end
  ["Portable reference", "Local-only state", "Requires v0.93 governance",
   "Requires transport security", "Redaction posture", "Fail-closed disposition"].each do |heading|
    check.call(feature.include?(heading), "matrix_column", heading)
  end
  check.call(feature.include?("Byte-identical state in a second location is still copied state."),
             "copy_semantics", "copy must not become continuity")
  check.call(feature.include?("remain quarantined") && design.include?("quarantined"),
             "ambiguity_semantics", "competing heads must remain quarantined")
  check.call(feature.include?("Raw state") && design.include?("cannot contain raw memory"),
             "private_state", "raw/private state must be excluded")
  check.call(design.include?("WP-17 does not define or implement:") &&
             design.include?("snapshot/chunk transport or storage replication"),
             "wp04_boundary", "runtime mechanics must remain WP-04")
  check.call(design.include?("Missing governance authority produces\n`defer`, never implicit acceptance."),
             "governance_boundary", "missing v0.93 authority must defer")
  check.call(handoff.include?("`candidate` is not an admission"),
             "handoff_boundary", "candidate must not imply governance admission")
  check.call(feature.include?("No production migration or federation is implemented."),
             "nonclaim", "production nonclaim is required")
  combined = [feature, design, handoff].join("\n").downcase
  FORBIDDEN_CLAIMS.each { |claim| check.call(!combined.include?(claim), "forbidden_claim", claim) }
  errors
end

def ancestor?(revision)
  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", revision, "HEAD", chdir: ROOT.to_s)
  status.success?
end

def validate_dependencies
  packet = JSON.parse(DEPENDENCIES.read)
  review = JSON.parse(REVIEW_INVENTORY.read)
  sprint = JSON.parse(SPRINT_REVIEW.read)
  assert(packet["schema"] == "adl.v092.wp17_dependency_authority.v1", "dependency_schema", packet["schema"])
  assert(packet["dependencies"].map { |item| item["issue"] } == [5826, 5827, 5834],
         "dependency_roster", "expected 5826, 5827, 5834")
  packet["dependencies"].each do |dependency|
    assert(dependency["issue_state"] == "closed", "dependency_issue_state", dependency["issue"])
    assert(dependency["pr_state"] == "merged", "dependency_pr_state", dependency["issue"])
    assert(ancestor?(dependency["merge_sha"]), "dependency_ancestry", dependency["merge_sha"])
    authority = if dependency["issue"] == 5834
                  sprint.fetch("children").find { |item| item["issue"] == 5834 }
                else
                  review.fetch("entries").find { |item| item["issue"] == dependency["issue"] }
                end
    assert(authority, "dependency_authority", dependency["issue"])
    pr_key = dependency["issue"] == 5834 ? "pr" : "pull_request"
    merge_key = dependency["issue"] == 5834 ? "merge_sha" : "merge_commit"
    assert(authority[pr_key] == dependency["pull_request"], "dependency_pr", dependency["issue"])
    assert(authority[merge_key] == dependency["merge_sha"], "dependency_merge", dependency["issue"])
  end
end

def negative_suite(feature, design, handoff)
  mutations = {
    "copied_state" => [feature.sub("Byte-identical state in a second location is still copied state.", "Copied state is approved."), design, handoff],
    "ambiguous_head" => [feature.gsub("remain quarantined", "select the newest head"), design.gsub("quarantined", "selected by timestamp"), handoff],
    "raw_private" => [feature, design.sub("cannot contain raw memory", "may contain raw memory"), handoff],
    "wp04_capture" => [feature, design.sub("WP-17 does not define or implement:", "WP-17 now implements:"), handoff],
    "governance_default" => [feature, design.sub("Missing governance authority produces\n`defer`, never implicit acceptance.", "Missing governance authority produces implicit acceptance."), handoff],
    "production_overclaim" => [feature.sub("No production migration or federation is implemented.", "Production migration is implemented."), design, handoff]
  }
  outcomes = mutations.transform_values do |texts|
    errors = validate_docs(*texts)
    assert(!errors.empty?, "negative_false_green", "mutation unexpectedly passed")
    errors.first
  end
  puts JSON.pretty_generate("schema" => "adl.v092.wp17_negative_validation.v1", "result" => "passed", "mutations" => outcomes)
end

[FEATURE, DESIGN, HANDOFF, DEPENDENCIES, REVIEW_INVENTORY, SPRINT_REVIEW].each do |path|
  assert(path.file?, "missing_path", path.relative_path_from(ROOT).to_s)
end
REQUIRED_PATHS.each { |path| assert(ROOT.join(path).file?, "missing_source", path) }
feature = FEATURE.read
design = DESIGN.read
handoff = HANDOFF.read
if ARGV == ["--negative"]
  negative_suite(feature, design, handoff)
  exit 0
end
assert(ARGV.empty?, "arguments", "use no argument or --negative")
errors = validate_docs(feature, design, handoff)
raise "document_contract: #{errors.join('; ')}" unless errors.empty?
validate_dependencies
puts JSON.pretty_generate(
  "schema" => "adl.v092.wp17_continuity_transfer_validation.v1",
  "result" => "passed", "matrix_rows" => EXPECTED_ROWS.length,
  "landed_schemas" => EXPECTED_SCHEMAS.length, "dependencies" => [5826, 5827, 5834]
)

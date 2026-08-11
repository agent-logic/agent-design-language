#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "digest"

ROOT = Pathname.new(__dir__).join("../../..").expand_path
FEATURE = ROOT.join("docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md")
DESIGN = ROOT.join("docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md")
HANDOFF = ROOT.join("docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md")
DEPENDENCIES = ROOT.join(".csdlc/evidence/5835/dependency-authority.json")
REVIEW_INVENTORY = ROOT.join("docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json")
SPRINT_REVIEW = ROOT.join(".csdlc/evidence/5857/sprint-review.json")
REJECTED_MATRIX = ROOT.join(".csdlc/evidence/5835/rejected-transfer-matrix.json")
ROLLBACK_PROOF = ROOT.join(".csdlc/evidence/5835/rollback-proof.json")
SIP_VALUES = ROOT.join(".csdlc/issues/5835/cards/sip.values.json")
STP_VALUES = ROOT.join(".csdlc/issues/5835/cards/stp.values.json")
SPP_VALUES = ROOT.join(".csdlc/issues/5835/cards/spp.values.json")

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
  .csdlc/evidence/209/local-validation-manifest.json
  .csdlc/evidence/209/native-validation-manifest.json
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
  check.call(design.include?("A proposal cannot add, rotate, or replace an anchor.") &&
             design.include?("valid signature under a caller-nominated key are insufficient"),
             "trusted_anchor", "caller claims must not establish source or policy authority")
  check.call(feature.include?("agent-logic/agent-design-language#209") &&
             feature.include?("a77519c3fca9f64752af41c9a2ebd396468891f7") &&
             feature.include?(".csdlc/evidence/209/native-validation-manifest.json"),
             "acip_authority", "ACIP row must bind replacement issue, PR, merge, and native proof")
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

def git_bytes(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  raise "git_failure: #{err.strip}" unless status.success?

  out
end

def validate_dependencies
  packet = JSON.parse(DEPENDENCIES.read)
  review = JSON.parse(REVIEW_INVENTORY.read)
  sprint = JSON.parse(SPRINT_REVIEW.read)
  rejected = JSON.parse(REJECTED_MATRIX.read)
  rollback = JSON.parse(ROLLBACK_PROOF.read)
  sip = JSON.parse(SIP_VALUES.read).dig("content", "values")
  stp = JSON.parse(STP_VALUES.read).dig("content", "values")
  spp = JSON.parse(SPP_VALUES.read).dig("content", "values")
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

  anchor = packet.fetch("trusted_source_anchor")
  assert(anchor["issue_repository"] == "danielbaustin/agent-design-language", "anchor_issue_repository", anchor["issue_repository"])
  assert(anchor["code_repository"] == "agent-logic/agent-design-language", "anchor_code_repository", anchor["code_repository"])
  [[anchor["birthday_manifest_path"], anchor["birthday_manifest_sha256"]],
   [anchor["sprint_review_path"], anchor["sprint_review_sha256"]]].each do |path, digest|
    assert(Digest::SHA256.file(ROOT.join(path)).hexdigest == digest, "anchor_digest", path)
  end

  acip = packet.fetch("acip_authority")
  assert(acip.values_at("issue", "pull_request", "head_sha", "merge_sha") ==
         [209, 215, "c640066f284a915b638add377cc4b0a2e221e6f9", "a77519c3fca9f64752af41c9a2ebd396468891f7"],
         "acip_identity", "replacement authority mismatch")
  assert(ancestor?(acip["merge_sha"]), "acip_ancestry", acip["merge_sha"])
  [[acip["local_manifest_path"], acip["local_manifest_sha256"]],
   [acip["native_manifest_path"], acip["native_manifest_sha256"]]].each do |path, digest|
    assert(Digest::SHA256.file(ROOT.join(path)).hexdigest == digest, "acip_digest", path)
  end
  local = JSON.parse(ROOT.join(acip["local_manifest_path"]).read)
  native = JSON.parse(ROOT.join(acip["native_manifest_path"]).read)
  assert(local["issue"] == 209 && local["status"] == "passed", "acip_local_status", local["status"])
  assert(native["issue"] == 209 && native["pull_request"] == 215 &&
         native.dig("jobs", "linux", "status") == "success" &&
         native.dig("jobs", "macos", "status") == "success" &&
         native.dig("jobs", "aggregate", "status") == "success" &&
         native.dig("independent_validation", "status") == "passed",
         "acip_native_status", "required native authority is incomplete")

  assert(rejected["schema"] == "adl.v092.wp17_rejected_transfer_matrix.v1" &&
         rejected.fetch("cases").map { |item| item["id"] }.sort ==
           %w[attacker-corpus competing-heads copied-state missing-governance raw-private-memory superseded-acip].sort,
         "rollback_matrix", "rejected matrix roster mismatch")
  assert(rollback["schema"] == "adl.v092.wp17_rollback_proof.v1" && rollback["result"] == "passed" &&
         rollback["base_revision"] == packet["base_revision"] && ancestor?(rollback["base_revision"]),
         "rollback_proof", "rollback proof or base revision missing")
  owned = [
    "docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md",
    "docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md"
  ]
  assert(rollback.fetch("owned_restore_paths") == owned, "rollback_owned_paths", "owned restore paths drifted")
  simulations = rollback.fetch("restore_simulation").to_h { |entry| [entry.fetch("path"), entry] }
  feature_restore = simulations.fetch(owned[0])
  feature_base = git_bytes("show", "#{rollback['base_revision']}:#{owned[0]}")
  assert(feature_restore["operation"] == "restore_base_bytes" &&
         Digest::SHA256.file(ROOT.join(owned[0])).hexdigest == feature_restore["before_sha256"] &&
         Digest::SHA256.hexdigest(feature_base) == feature_restore["after_sha256"] &&
         feature_restore["after_sha256"] == feature_restore["base_sha256"],
         "rollback_feature_simulation", "feature restore does not reproduce base bytes")
  design_restore = simulations.fetch(owned[1])
  design_in_base = system("git", "cat-file", "-e", "#{rollback['base_revision']}:#{owned[1]}",
                          chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
  assert(design_restore["operation"] == "remove_added_path" &&
         Digest::SHA256.file(ROOT.join(owned[1])).hexdigest == design_restore["before_sha256"] &&
         !design_in_base && design_restore.values_at("after_state", "base_state") == ["absent", "absent"],
         "rollback_design_simulation", "added design is not removed to the base state")
  expected_read_only = ["docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md"]
  assert(rollback.fetch("read_only_paths").map { |entry| entry["path"] } == expected_read_only,
         "rollback_read_only_paths", "read-only path roster drifted")
  rollback.fetch("read_only_paths").each do |entry|
    base_bytes = git_bytes("show", "#{rollback['base_revision']}:#{entry['path']}")
    current_digest = Digest::SHA256.file(ROOT.join(entry["path"])).hexdigest
    assert(current_digest == entry["current_sha256"] &&
           Digest::SHA256.hexdigest(base_bytes) == entry["base_sha256"] &&
           entry["current_sha256"] == entry["base_sha256"],
           "rollback_read_only_digest", entry["path"])
  end
  rollback.fetch("retained_evidence").each do |entry|
    path = entry.fetch("path")
    assert(ROOT.join(path).file? && Digest::SHA256.file(ROOT.join(path)).hexdigest == entry["sha256"],
           "rollback_retained_digest", path)
  end
  rollback.fetch("preserved_authority").each do |entry|
    path = entry.fetch("path")
    assert(!owned.include?(path) && Digest::SHA256.file(ROOT.join(path)).hexdigest == entry["sha256"],
           "rollback_authority_digest", path)
  end
  handoff = "docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md"
  assert(sip.fetch("declared_scope") == [
           "docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md",
           "docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md",
           ".csdlc/evidence/5835/"
         ] && !sip.fetch("declared_scope").include?(handoff),
         "lifecycle_declared_scope", "read-only handoff entered declared write scope")
  assert(stp.fetch("deliverables").none? { |value| value.include?("handoff") } &&
         stp.fetch("acceptance_criteria").any? { |value| value.include?("handoff remains byte-identical and read-only") },
         "lifecycle_stp_handoff", "STP deliverable or acceptance truth authorizes handoff edits")
  handoff_step = spp.fetch("steps").find { |step| step["id"] == "S3" }
  assert(handoff_step && handoff_step["status"] == "completed" &&
         handoff_step["action"].include?("remains unchanged") && handoff_step["action"].include?("read-only"),
         "lifecycle_spp_handoff", "SPP does not prove the handoff remained read-only")
end

def negative_suite(feature, design, handoff)
  mutations = {
    "copied_state" => [feature.sub("Byte-identical state in a second location is still copied state.", "Copied state is approved."), design, handoff],
    "ambiguous_head" => [feature.gsub("remain quarantined", "select the newest head"), design.gsub("quarantined", "selected by timestamp"), handoff],
    "raw_private" => [feature, design.sub("cannot contain raw memory", "may contain raw memory"), handoff],
    "wp04_capture" => [feature, design.sub("WP-17 does not define or implement:", "WP-17 now implements:"), handoff],
    "governance_default" => [feature, design.sub("Missing governance authority produces\n`defer`, never implicit acceptance.", "Missing governance authority produces implicit acceptance."), handoff],
    "production_overclaim" => [feature.sub("No production migration or federation is implemented.", "Production migration is implemented."), design, handoff],
    "caller_anchor" => [feature, design.sub("A proposal cannot add, rotate, or replace an anchor.", "A proposal may establish its own anchor."), handoff],
    "superseded_acip" => [feature.sub("agent-logic/agent-design-language#209", "danielbaustin/agent-design-language#5832"), design, handoff]
  }
  outcomes = mutations.transform_values do |texts|
    errors = validate_docs(*texts)
    assert(!errors.empty?, "negative_false_green", "mutation unexpectedly passed")
    errors.first
  end
  puts JSON.pretty_generate("schema" => "adl.v092.wp17_negative_validation.v1", "result" => "passed", "mutations" => outcomes)
end

[FEATURE, DESIGN, HANDOFF, DEPENDENCIES, REVIEW_INVENTORY, SPRINT_REVIEW, REJECTED_MATRIX, ROLLBACK_PROOF].each do |path|
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
  "landed_schemas" => EXPECTED_SCHEMAS.length, "dependencies" => [5826, 5827, 5834],
  "trusted_authorities" => [209], "rollback_cases" => 6
)

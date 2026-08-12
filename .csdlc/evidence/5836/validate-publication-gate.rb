#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"
require "pathname"

abort "usage: validate-publication-gate.rb --check-only" unless ARGV == ["--check-only"]
root = Pathname.new(__dir__).join("../../..").cleanpath

def read_json(root, path)
  JSON.parse(root.join(path).read)
rescue Errno::ENOENT, JSON::ParserError
  nil
end

def canonical_json(value)
  case value
  when Hash
    "{" + value.keys.sort.map { |key| "#{JSON.generate(key)}:#{canonical_json(value[key])}" }.join(",") + "}"
  when Array
    "[" + value.map { |item| canonical_json(item) }.join(",") + "]"
  else
    JSON.generate(value)
  end
end

def valid_packet?(packet, expected)
  return false unless packet.is_a?(Hash) && packet["schema"] == "adl.first_birthday.demo_packet.v1"
  return false unless packet["status"] == expected
  recorded = packet["packet_sha256"].to_s
  material = Marshal.load(Marshal.dump(packet))
  material["packet_sha256"] = ""
  recorded.match?(/\A[0-9a-f]{64}\z/) && recorded == Digest::SHA256.hexdigest(canonical_json(material))
end

def typed_rejection?(packet, expected)
  packet["rejections"] == [{"code" => "birthday", "rejection" => expected}]
end

def revision_covers?(root, receipt, paths)
  revision = receipt && receipt["source_revision"].to_s
  return false unless revision.match?(/\A[0-9a-f]{40}\z/)
  _out, status = Open3.capture2("git", "merge-base", "--is-ancestor", revision, "HEAD", chdir: root.to_s)
  return false unless status.success?
  _out, status = Open3.capture2("git", "diff", "--quiet", revision, "HEAD", "--", *paths, chdir: root.to_s)
  status.success?
end

positive_path = "demos/v0.92/first-birthday/positive.json"
positive = read_json(root, positive_path)
positive_valid = valid_packet?(positive, "complete") &&
  positive.dig("decision", "accepted") == true &&
  positive.dig("witness_packet", "receipt", "disposition") == "witnesses_accepted" &&
  positive.dig("witness_packet", "receipt", "receipt_sha256").to_s.match?(/\A[0-9a-f]{64}\z/) &&
  positive.dig("witness_packet", "witness_set", "witnesses").is_a?(Array) &&
  positive.dig("witness_packet", "witness_set", "witnesses").length == 4

lifecycle_cases = {
  "startup" => "process_startup", "wake" => "wake_or_resume",
  "restore" => "restore_from_checkpoint", "snapshot" => "snapshot_creation",
  "admission" => "test_environment_admission", "copied_state" => "copied_state",
  "simulation" => "simulation_run", "named_fixture" => "named_test_fixture"
}
missing_cases = %w[identity_root continuity_head memory_grounding capability_envelope cognitive_profile witness_set receipt reviewer_validation]
negative_valid = lifecycle_cases.all? do |name, event|
  packet = read_json(root, "demos/v0.92/first-birthday/negative-#{name}.json")
  valid_packet?(packet, "rejected") && typed_rejection?(packet, {"code" => "lifecycle_lookalike", "event" => event})
end
negative_valid &&= missing_cases.all? do |kind|
  packet = read_json(root, "demos/v0.92/first-birthday/negative-missing_#{kind}.json")
  valid_packet?(packet, "rejected") && typed_rejection?(packet, {"code" => "missing_evidence", "kind" => kind})
end
interrupted = read_json(root, "demos/v0.92/first-birthday/interrupted.json")
negative_valid &&= valid_packet?(interrupted, "incomplete") &&
  interrupted["rejections"] == [{"code" => "interrupted_before_receipt"}]

implementation_paths = [
  "adl-runtime-kernel/src/birthday_demo.rs",
  "adl-runtime-kernel/src/bin/adl-runtime-birthday-demo.rs",
  "adl-runtime-kernel/tests/birthday_demo.rs",
  "adl/tools/demo_v092_first_birthday.sh",
  "adl/tools/test_v092_first_birthday_demo.sh",
  "adl/tools/validate_v092_first_birthday_packet.py",
  "demos/v0.92/first-birthday"
]
macos = read_json(root, ".csdlc/evidence/5836/native-macos-receipt.json")
linux = read_json(root, ".csdlc/evidence/5836/native-linux-receipt.json")
platform_valid = {"macos" => macos, "linux" => linux}.all? do |platform, receipt|
  receipt && receipt["schema"] == "adl.first_birthday.native_receipt.v1" &&
    receipt["platform"] == platform && receipt["result"] == "passed" &&
    receipt["packet_sha256"] == Digest::SHA256.file(root.join(positive_path)).hexdigest &&
    revision_covers?(root, receipt, implementation_paths)
end

head, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: root.to_s)
index = read_json(root, ".csdlc/issues/5836/index.json")
review_revision = index&.dig("review", "reviewed_revision").to_s
review_sha = review_revision.match(/\Agit-blake3:([0-9a-f]{40}):[0-9a-f]{64}\z/)&.captures&.first
review_valid = head_status.success? && index&.dig("review", "completed") == true &&
  index&.dig("review", "findings") == [] && review_sha == head.strip

docs = [
  "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
  "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
  "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
  "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
]
unsupported_claims_resolved = docs.all? { |path| root.join(path).file? } &&
  positive["non_claims"].is_a?(Array) && positive["non_claims"].include?("no_publication_authorization")

gate = read_json(root, ".csdlc/evidence/5836/publication-gate.json") || {}
authorization_path = gate.dig("operator_publication_authorization", "evidence").to_s
authorization = authorization_path.empty? ? nil : read_json(root, authorization_path)
operator_authorized = authorization &&
  authorization["schema"] == "adl.first_birthday.operator_publication_authorization.v1" &&
  authorization["issue"] == 5836 && authorization["authorized"] == true &&
  authorization["revision"] == head.strip

checks = {
  "missing_accepted_witness_receipt_proof" => positive_valid,
  "invalid_or_stale_native_platform_receipt" => platform_valid,
  "stale_or_missing_exact_head_review" => review_valid,
  "unsupported_claims_unresolved" => unsupported_claims_resolved,
  "unresolved_negative_suite" => negative_valid,
  "absent_operator_authorization" => operator_authorized
}
blockers = checks.reject { |_name, passed| passed }.keys
puts JSON.generate({
  "schema" => "adl.first_birthday.publication_gate_result.v1",
  "decision" => blockers.empty? ? "eligible_for_operator_publication" : "do_not_publish",
  "blockers" => blockers,
  "can_publish" => false
})
exit(blockers.empty? ? 0 : 65)

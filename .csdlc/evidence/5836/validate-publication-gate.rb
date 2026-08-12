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

def packet_evidence_valid?(root, packet)
  evidence = packet.dig("candidate", "evidence")
  return false unless evidence.is_a?(Array) && evidence.length == 10
  by_kind = evidence.to_h { |entry| [entry["kind"], entry] }
  return false unless by_kind.length == 10
  packet_path = "demos/v0.92/first-birthday/positive.json"
  expected_packet_digests = {
    "stable_name" => Digest::SHA256.hexdigest(packet.dig("identity", "stable_name").to_s),
    "identity_root" => packet.dig("identity", "record_sha256"),
    "continuity_head" => packet.dig("continuity", "record_sha256"),
    "memory_grounding" => packet.dig("identity", "continuity", "reference", "sha256"),
    "witness_set" => packet.dig("witness_packet", "witness_set", "roster_sha256")
  }
  valid = by_kind.all? do |kind, entry|
    path = entry["path"].to_s
    digest = entry["sha256"].to_s
    if path == packet_path
      digest == expected_packet_digests[kind]
    else
      root.join(path).file? && digest == Digest::SHA256.file(root.join(path)).hexdigest
    end
  end
  public_evidence = packet.dig("witness_packet", "receipt", "public_evidence")
  valid && public_evidence.is_a?(Array) && public_evidence.length == evidence.length &&
    public_evidence.all? do |entry|
      retained = by_kind[entry["kind"]]
      retained && entry["path"] == retained["path"] && entry["sha256"] == retained["sha256"]
    end
end

def nested_evidence_valid?(root, packet)
  revision_path = "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json"
  revision_digest = Digest::SHA256.file(root.join(revision_path)).hexdigest
  packet_path = "demos/v0.92/first-birthday/positive.json"
  launch_path = "docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md"
  receipt_path = "docs/milestones/v0.91.8/review/v092_handoff_4762/birth-receipt-4762.v1.json"
  file_digest = lambda { |path| Digest::SHA256.file(root.join(path)).hexdigest }
  expected_capability = {
    "birthday" => ["birthday", packet_path, packet.dig("candidate", "packet_sha256")],
    "identity" => ["identity", packet_path, packet.dig("identity", "record_sha256")],
    "retained" => ["retained_capability", launch_path, file_digest.call(launch_path)],
    "provider" => ["provider", revision_path, revision_digest],
    "model" => ["model", revision_path, revision_digest],
    "authority" => ["authority", receipt_path, file_digest.call(receipt_path)]
  }
  expected_cognitive = {
    "identity" => ["identity", packet_path, packet.dig("identity", "record_sha256")],
    "continuity" => ["continuity", packet_path, packet.dig("continuity", "record_sha256")],
    "memory" => ["memory", launch_path, file_digest.call(launch_path)],
    "capability" => ["capability", packet_path, packet.dig("capability", "envelope_sha256")],
    "tom" => ["theory_of_mind", revision_path, revision_digest],
    "intelligence" => ["intelligence", revision_path, revision_digest],
    "learning" => ["governed_learning", launch_path, file_digest.call(launch_path)]
  }
  validate = lambda do |references, expected, discriminator|
    references.is_a?(Array) && references.length == expected.length &&
      references.map { |entry| entry["id"] }.sort == expected.keys.sort && references.all? do |entry|
      contract = expected[entry["id"]]
      contract && entry[discriminator] == contract[0] && entry["path"] == contract[1] &&
        entry["sha256"] == contract[2] && entry["revision_sha256"] == revision_digest
    end
  end
  validate.call(packet.dig("capability", "evidence"), expected_capability, "kind") &&
    validate.call(packet.dig("cognitive_profile", "evidence"), expected_cognitive, "category")
end

positive_path = "demos/v0.92/first-birthday/positive.json"
positive = read_json(root, positive_path)
positive_valid = valid_packet?(positive, "complete") &&
  positive.dig("decision", "accepted") == true &&
  positive.dig("witness_packet", "receipt", "disposition") == "witnesses_accepted" &&
  positive.dig("witness_packet", "receipt", "receipt_sha256").to_s.match?(/\A[0-9a-f]{64}\z/) &&
  positive.dig("witness_packet", "witness_set", "witnesses").is_a?(Array) &&
  positive.dig("witness_packet", "witness_set", "witnesses").length == 4 &&
  packet_evidence_valid?(root, positive) && nested_evidence_valid?(root, positive)

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
status_text, status_status = Open3.capture2("git", "status", "--porcelain", chdir: root.to_s)
index = read_json(root, ".csdlc/issues/5836/index.json")
review_revision = index&.dig("review", "reviewed_revision").to_s
review_sha = review_revision.match(/\Agit-blake3:([0-9a-f]{40}):[0-9a-f]{64}\z/)&.captures&.first
reviewed_paths = implementation_paths + [
  ".csdlc/evidence/5836",
  "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
  "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
  "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
  "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
]
review_covers_head = review_sha && begin
  _out, ancestor = Open3.capture2("git", "merge-base", "--is-ancestor", review_sha, "HEAD", chdir: root.to_s)
  _out, unchanged = Open3.capture2("git", "diff", "--quiet", review_sha, "HEAD", "--", *reviewed_paths, chdir: root.to_s)
  ancestor.success? && unchanged.success?
end
common_dir, common_status = Open3.capture2("git", "rev-parse", "--git-common-dir", chdir: root.to_s)
doctor = common_status.success? ? Pathname.new(common_dir.strip).cleanpath.parent.join(".adl/bin/csdlc-v2/csdlc-doctor") : nil
doctor_output, doctor_status = doctor&.executable? ? Open3.capture2(doctor.to_s, "--repo", root.to_s, "--issue", "5836") : ["", nil]
doctor_pass = doctor_status&.success? && JSON.parse(doctor_output)["status"] == "pass" rescue false
review_valid = head_status.success? && status_status.success? && status_text.empty? && doctor_pass &&
  index&.dig("review", "completed") == true && index&.dig("review", "findings") == [] && review_covers_head

docs = [
  "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
  "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
  "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
  "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
]
required_doc_boundaries = {
  docs[1] => "does not claim the birthday has been publicly accepted or launched",
  docs[2] => "public launch approval exists without operator authorization",
  docs[3] => "operator"
}
contradictory_claim = docs.any? do |path|
  root.join(path).each_line.any? do |line|
    text = line.downcase
    birthday_claim = text.include?("the first birthday has happened") &&
      !text.include?("the first birthday has happened before retained witness/receipt proof")
    launch_claim = text.match?(/\bpublic launch (?:is|has been) approved\b/)
    authorization_claim = text.match?(/\bauthorized for (?:publication|public launch)\b/) &&
      !text.match?(/\b(?:not|never) authorized for (?:publication|public launch)\b/)
    birthday_claim || launch_claim || authorization_claim
  end
end
unsupported_claims_resolved = docs.all? { |path| root.join(path).file? } &&
  required_doc_boundaries.all? { |path, phrase| root.join(path).read.downcase.include?(phrase) } &&
  !contradictory_claim &&
  positive["non_claims"].is_a?(Array) && positive["non_claims"].include?("no_publication_authorization")

# WP-18 cannot grant publication authority. That remains an external operator
# gate owned by the release tail, so this check-only validator always blocks it.
operator_authorized = false

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

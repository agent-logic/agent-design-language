#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "open3"

architecture = "docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md"
threat_model = "docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md"
review = ".csdlc/evidence/5821/architecture-security-review.json"
review_report = ".csdlc/evidence/5821/architecture-security-review.md"
feature = "docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md"
adr = "docs/adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md"
design = ".csdlc/prepared/issues/5821/design.md"
validator = ".csdlc/prepared/issues/5821/validate-architecture-security-review.rb"
authoritative = [architecture, threat_model, design, feature, adr, validator]
[review, review_report, *authoritative].each { |path| abort "missing #{path}" unless File.file?(path) }

arch_text = File.read(architecture)
%w[Guardian identity enrollment discovery membership transport epoch lease fencing placement snapshot migration rollback observability].each do |term|
  abort "architecture omits #{term}" unless arch_text.downcase.include?(term.downcase)
end
abort "architecture must require maintained QUIC/TLS" unless arch_text.match?(/maintained.*QUIC.*TLS/im)
abort "custom cryptography boundary missing" unless arch_text.downcase.include?("custom cryptography")

threat_text = File.read(threat_model).downcase
["partition", "replay", "stale lease", "cloned state", "wrong trust domain", "certificate compromise", "certificate expiry", "relocation failure", "rollback failure", "split-brain"].each do |threat|
  abort "threat model omits #{threat}" unless threat_text.include?(threat)
end

packet = JSON.parse(File.read(review))
abort "wrong review schema" unless packet["schema"] == "adl.wp04.architecture_security_review.v1"
abort "review not accepted" unless packet["outcome"] == "accepted"
abort "reviewer identity missing" if packet["reviewer"].to_s.empty?
abort "review revision missing" unless packet["reviewed_revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
abort "actionable findings remain" unless Array(packet["unresolved_actionable_findings"]).empty?
provenance = packet.fetch("reviewer_provenance")
abort "review is not independently attributed" unless provenance["role"] == "independent_architecture_security_reviewer" && provenance["independent_from_author"] == true
abort "review agent identity missing" unless provenance["agent_id"].to_s.match?(/\A[0-9a-f-]{20,}\z/)
abort "review author and reviewer collide" if provenance["author_identity"].to_s.empty? || provenance["author_identity"] == packet["reviewer"]
abort "wrong retained review report" unless provenance["report_path"] == review_report
abort "review report digest mismatch" unless provenance["report_sha256"] == Digest::SHA256.file(review_report).hexdigest
report_text = File.read(review_report)
abort "review report lacks exact revision" unless report_text.include?(packet["reviewed_revision"])
abort "review report lacks reviewer identity" unless report_text.include?(packet["reviewer"])
abort "review report is not accepted" unless report_text.include?("Verdict: accepted")

_, status = Open3.capture2("git", "merge-base", "--is-ancestor", packet["reviewed_revision"], "HEAD")
abort "review revision is not ancestral to HEAD" unless status.success?
expected = authoritative.to_h do |path|
  bytes, stderr, git_status = Open3.capture3("git", "show", "#{packet['reviewed_revision']}:#{path}")
  abort "review revision does not contain #{path}: #{stderr}" unless git_status.success?
  [path, Digest::SHA256.hexdigest(bytes)]
end
abort "review packet digest mismatch" unless packet["artifact_sha256"] == expected

puts "PASS: exact architecture, threat model, and independent accepted review packet"

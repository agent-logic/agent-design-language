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
child_validator = ".csdlc/prepared/issues/5821/validate-child-wave.rb"
authority_children = [5869, 5870, 5875, 5876]
child_authoritative = authority_children.flat_map do |issue|
  [
    ".csdlc/prepared/issues/#{issue}/design.md",
    ".csdlc/issues/#{issue}/index.json",
    ".csdlc/issues/#{issue}/cards/sip.values.json",
    ".csdlc/issues/#{issue}/cards/stp.values.json",
    ".csdlc/issues/#{issue}/cards/vpp.values.json"
  ]
end
authoritative = [architecture, threat_model, design, feature, adr, validator, child_validator, *child_authoritative]
[review, review_report, *authoritative].each { |path| abort "missing #{path}" unless File.file?(path) }

def section(text, heading)
  text[/^#{Regexp.escape(heading)}\s*$\n(.*?)(?=^## |\z)/m, 1].to_s
end

arch_text = File.read(architecture)
required_architecture = {
  "## Invariants" => ["at most one active authoritative Guardian", "Availability cannot override fencing"],
  "## Identity And Enrollment" => ["proves possession", "nonce", "Wrong-domain", "replayed"],
  "## Certificate Purposes And Lifecycle" => ["not interchangeable", "actively closes affected sessions", "revalidates", "revocation"],
  "## Maintained QUIC/TLS Transport" => ["quinn", "rustls", "prost", "custom cryptography", "custom wire framing"],
  "## Discovery, Join, And Membership" => ["seeds as addresses, never trust anchors", "deterministic order", "committed epoch"],
  "## Epochs, Leases, And Fencing" => ["openraft", "at least three voters", "non-voting learners", "joint membership", "majority", "linearization point", "leader term", "committed log index", "certificate generation", "Quorum loss", "numerically highest local epoch"],
  "## Advertisements And Placement" => ["deterministic placement", "stable tie-break order", "fenced node", "no eligible target"],
  "## Snapshot And Migration Protocol" => ["prepare -> quiesce -> checkpoint -> transfer -> validate -> fence -> activate -> commit", "prior lease safety", "source permit"],
  "## Rollback And Recovery" => ["Before `fence`", "After `fence`, before `activate`", "both remain fenced"],
  "## Projection And Observability" => ["authenticated, redacted projection", "correlation ID", "diagnostic evidence only"],
  "## Child Ownership And Integration" => ["sole manifest and lockfile owner", "quinn", "openraft", "owns no product path"]
}
required_architecture.each do |heading, terms|
  body = section(arch_text, heading)
  abort "architecture omits section #{heading}" if body.empty?
  normalized = body.gsub(/\s+/, " ").downcase
  terms.each { |term| abort "#{heading} omits #{term}" unless normalized.include?(term.gsub(/\s+/, " ").downcase) }
end

threat_text = File.read(threat_model)
required_threats = {
  "### T1: Unauthorized or wrong-domain enrollment" => ["proof of possession", "one-time nonce", "trust-domain binding"],
  "### T2: Replay and stale lease activation" => ["replay", "stale authority", "fencing-token checks"],
  "### T3: Partition-induced split brain" => ["OpenRaft", "majority", "joint membership", "quorum or clock uncertainty halts mutation"],
  "### T4: Cloned state and identity collision" => ["non-persistent activation key", "cannot renew", "newer committed epoch"],
  "### T5: Certificate compromise or certificate expiry" => ["active session closure", "per-authority-operation certificate checks", "no verification bypass"],
  "### T6: Transport downgrade, malformed input, or resource exhaustion" => ["Maintained QUIC/TLS only", "stream limits", "per-peer quotas"],
  "### T7: Forged capability or resource-weather evidence" => ["signatures", "freshness", "never grant authority"],
  "### T8: Snapshot substitution or disclosure" => ["chunk digests", "authenticated encrypted transport", "isolated restore"],
  "### T9: Relocation failure" => ["source authority retained", "target activation only after fence", "failure-stage recovery"],
  "### T10: Rollback failure or ambiguous commit" => ["majority-committed", "minority cannot renew", "both candidates fenced"],
  "### T11: Projection, log, or audit leakage and poisoning" => ["field-level redaction", "bounded labels", "diagnostic evidence"]
}
required_threats.each do |heading, terms|
  body = section(threat_text, heading)
  abort "threat model omits section #{heading}" if body.empty?
  normalized = body.gsub(/\s+/, " ").downcase
  terms.each { |term| abort "#{heading} omits #{term}" unless normalized.include?(term.gsub(/\s+/, " ").downcase) }
end

packet = JSON.parse(File.read(review))
expected_reviewer = "openai-codex:gpt-5:wp04-architecture-security-independent-review:2026-08-07"
expected_agent = "019fdf69-5a0f-7e31-ba34-419c135eb7e8"
abort "wrong review schema" unless packet["schema"] == "adl.wp04.architecture_security_review.v1"
abort "review not accepted" unless packet["outcome"] == "accepted"
abort "wrong retained reviewer identity" unless packet["reviewer"] == expected_reviewer
abort "review revision missing" unless packet["reviewed_revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
abort "actionable findings remain" unless Array(packet["unresolved_actionable_findings"]).empty?
dispositions = packet.fetch("finding_dispositions")
abort "review finding dispositions missing" unless dispositions.is_a?(Array) && !dispositions.empty?
dispositions.each do |finding|
  abort "review finding id missing" if finding["id"].to_s.empty?
  abort "review finding severity missing" unless finding["severity"].to_s.match?(/\AP[0-3]\z/)
  abort "review finding unresolved" unless finding["disposition"] == "resolved"
  abort "review finding evidence missing" if Array(finding["evidence"]).empty?
end
provenance = packet.fetch("reviewer_provenance")
abort "review is not independently attributed" unless provenance["role"] == "independent_architecture_security_reviewer" && provenance["independent_from_author"] == true
abort "wrong retained review agent" unless provenance["agent_id"] == expected_agent
abort "review author and reviewer collide" if provenance["author_identity"].to_s.empty? || provenance["author_identity"] == packet["reviewer"]
abort "wrong retained review report" unless provenance["report_path"] == review_report
abort "review report digest mismatch" unless provenance["report_sha256"] == Digest::SHA256.file(review_report).hexdigest
report_text = File.read(review_report)
abort "review report lacks exact revision" unless report_text.include?(packet["reviewed_revision"])
abort "review report lacks reviewer identity" unless report_text.include?(packet["reviewer"])
abort "review report is not accepted" unless report_text.include?("Verdict: accepted")

_, status = Open3.capture2("git", "merge-base", "--is-ancestor", packet["reviewed_revision"], "HEAD")
abort "review revision is not ancestral to HEAD" unless status.success?
latest_authoritative_revision, latest_status = Open3.capture2("git", "rev-list", "-1", "HEAD", "--", *authoritative)
abort "cannot resolve authoritative revision" unless latest_status.success?
abort "review does not cover the latest authoritative revision" unless packet["reviewed_revision"] == latest_authoritative_revision.strip
_, dirty_status = Open3.capture2("git", "diff", "--quiet", "--", *authoritative)
abort "authoritative review surface has uncommitted changes" unless dirty_status.success?
_, staged_status = Open3.capture2("git", "diff", "--cached", "--quiet", "--", *authoritative)
abort "authoritative review surface has staged changes" unless staged_status.success?
_, drift_status = Open3.capture2("git", "diff", "--quiet", packet["reviewed_revision"], "HEAD", "--", *authoritative)
abort "authoritative review surface changed after review" unless drift_status.success?
expected = authoritative.to_h do |path|
  bytes, stderr, git_status = Open3.capture3("git", "show", "#{packet['reviewed_revision']}:#{path}")
  abort "review revision does not contain #{path}: #{stderr}" unless git_status.success?
  [path, Digest::SHA256.hexdigest(bytes)]
end
abort "review packet digest mismatch" unless packet["artifact_sha256"] == expected

puts "PASS: exact architecture, threat model, and independent accepted review packet"

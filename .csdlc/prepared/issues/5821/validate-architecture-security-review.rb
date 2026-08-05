#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "open3"

architecture = "docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md"
threat_model = "docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md"
review = ".csdlc/evidence/5821/architecture-security-review.json"
[architecture, threat_model, review].each { |path| abort "missing #{path}" unless File.file?(path) }

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
author = packet["author"].to_s.strip
reviewer = packet["reviewer"].to_s.strip
abort "author identity missing" if author.empty?
abort "reviewer identity missing" if reviewer.empty?
abort "reviewer must be independent from author" if reviewer == author
head, status = Open3.capture2("git", "rev-parse", "HEAD")
abort "cannot resolve current HEAD" unless status.success?
head = head.strip
abort "current HEAD is invalid" unless head.match?(/\A[0-9a-f]{40}\z/)
abort "review revision does not match current exact HEAD" unless packet["reviewed_revision"] == head
abort "actionable findings remain" unless Array(packet["unresolved_actionable_findings"]).empty?
expected = {architecture => Digest::SHA256.file(architecture).hexdigest, threat_model => Digest::SHA256.file(threat_model).hexdigest, ".csdlc/prepared/issues/5821/design.md" => Digest::SHA256.file(".csdlc/prepared/issues/5821/design.md").hexdigest}
abort "review packet digest mismatch" unless packet["artifact_sha256"] == expected

puts "PASS: exact architecture, threat model, and independent accepted review packet"

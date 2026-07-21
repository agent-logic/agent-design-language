#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"

ROOT = File.expand_path("../../../..", __dir__)
DEPENDENCIES = [5346, 5344, 5343, 5358, 5361].freeze
ORDER_RESOLUTION = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13/dependency-order-resolution.json")
CORE_MANIFEST = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-core/final-core-deletion-manifest.json")
REPOSITORY = "danielbaustin/agent-design-language"
RECEIPT_VERIFIER = File.join(__dir__, "verify-terminal-receipt.rb")

def fail!(message)
  warn("#5347 dependency gate blocked: #{message}")
  exit(1)
end

DEPENDENCIES.each do |issue|
  _out, err, status = Open3.capture3("ruby", RECEIPT_VERIFIER, issue.to_s, chdir: ROOT)
  fail!("##{issue} terminal verification failed: #{err.lines.first}") unless status.success?
end

fail!("missing authoritative #5346/#5347 dependency-order resolution") unless File.file?(ORDER_RESOLUTION)
resolution = JSON.parse(File.read(ORDER_RESOLUTION))
fail!("dependency-order schema mismatch") unless resolution["schema"] == "adl.wp13.dependency_order.v1"
fail!("dependency order does not require terminal #5346 before #5347") unless resolution["order"] == [5346, 5347]
fail!("dependency-order resolution is not reviewed") unless resolution["review_status"] == "accepted"
fail!("dependency-order reviewer missing") if resolution["reviewer"].to_s.empty?
fail!("dependency-order revision malformed") unless resolution["revision"].to_s.match?(/\A[0-9a-f]{40}\z/)
fail!("dependency-order revision is not current ancestry") unless Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", resolution["revision"], "HEAD").last.success?
fail!("missing exact #5346 manifest") unless File.file?(CORE_MANIFEST)
core_sha256 = Digest::SHA256.file(CORE_MANIFEST).hexdigest
fail!("dependency-order #5346 manifest digest mismatch") unless resolution["core_manifest_sha256"] == core_sha256

puts(JSON.generate({schema: "adl.wp13.external_band_dependency_gate.v1", issue: 5347, status: "pass", dependencies: DEPENDENCIES}))

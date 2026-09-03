#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

root = File.expand_path("../../../..", __dir__)
manifest_path = File.join(root, ".csdlc/prepared/issues/660/delete-public-podcast-prefix.json")
evidence_path = File.join(root, ".csdlc/evidence/660/emergency-exposure-rollback.json")
preview_path = File.join(root, "demos/_preview/podcast/index.html")
logo_path = File.join(root, "demos/_preview/podcast/assets/agent-logic-logo.svg")
launch_readiness_path = File.join(root, "demos/podcast/LAUNCH_READINESS.md")
publication_hold_path = File.join(root, "demos/podcast/PUBLICATION_HOLD.md")
runbook_path = File.join(root, "demos/podcast/S3_CLOUDFRONT_RUNBOOK.md")

def fail_with(message)
  warn(message)
  exit 1
end

manifest = JSON.parse(File.read(manifest_path))
evidence = JSON.parse(File.read(evidence_path))
preview = File.read(preview_path)

expected_keys = Set.new([
  "podcast/",
  "podcast/artwork.png",
  "podcast/audio/meet-the-ai-coworkers.mp3",
  "podcast/audio/meet-the-ai-coworkers.wav",
  "podcast/episodes/meet-the-ai-coworkers/",
  "podcast/episodes/meet-the-ai-coworkers/index.html",
  "podcast/feed.xml",
  "podcast/index.html",
  "podcast/studio/image-slot.js",
  "podcast/studio/support.js",
  "podcast/studio/uploads/agent-logic-logo.svg",
  "podcast/studio/vendor/react-dom.production.min.js",
  "podcast/studio/vendor/react.production.min.js"
])

manifest_keys = Set.new(manifest.fetch("Objects").map { |object| object.fetch("Key") })
fail_with("delete manifest key set drifted") unless manifest_keys == expected_keys
fail_with("delete manifest must be explicit, not quiet") unless manifest["Quiet"] == false
fail_with("wildcard or non-podcast delete target present") if manifest_keys.any? { |key| key.include?("*") || !key.start_with?("podcast/") }

fail_with("wrong issue") unless evidence.fetch("issue") == 660
hosting = evidence.fetch("hosting_context")
fail_with("wrong origin bucket") unless hosting.fetch("origin_bucket") == "agent-logic-ai-origin-agentlogic"
fail_with("wrong distribution") unless hosting.fetch("distribution_id") == "E3C29FMX32KDDU"
fail_with("private archive bucket missing") unless hosting.fetch("private_archive_bucket") == "agent-logic-podcast-archive-agentlogic"

public_delete = evidence.dig("remote_actions", "public_prefix_delete")
fail_with("delete object count mismatch") unless public_delete.fetch("objects_deleted") == expected_keys.size
fail_with("versions must not be purged") unless public_delete.fetch("purged_versions") == false
fail_with("delete marker count mismatch") unless public_delete.fetch("delete_markers").size == expected_keys.size
fail_with("delete marker key set mismatch") unless Set.new(public_delete.fetch("delete_markers").map { |entry| entry.fetch("key") }) == expected_keys
fail_with("public invalidation must be completed") unless public_delete.dig("cloudfront_invalidation", "status") == "Completed"
preview_invalidations = evidence.dig("remote_actions", "hidden_preview_update", "cloudfront_invalidations")
fail_with("preview invalidation evidence missing") unless preview_invalidations.is_a?(Array) && !preview_invalidations.empty?
preview_invalidations.each do |entry|
  fail_with("preview invalidation must be completed: #{entry.fetch("id")}") unless entry.fetch("status") == "Completed"
end

verification = evidence.fetch("verification")
verification.fetch("public_urls").each do |entry|
  fail_with("public URL not blocked: #{entry.fetch("url")}") unless entry.fetch("observed_status") == 403
end
preview_entry = verification.fetch("hidden_preview_urls").find { |entry| entry.fetch("url") == "https://agent-logic.ai/_preview/podcast/" }
fail_with("preview verification missing") unless preview_entry
fail_with("preview URL not live") unless preview_entry.fetch("observed_status") == 200
fail_with("preview title mismatch") unless preview_entry.fetch("title") == "The Cognitive Stack"
fail_with("preview robots mismatch") unless preview_entry.fetch("robots") == "noindex,nofollow"
fail_with("preview must not link public feed") unless preview_entry.fetch("contains_public_feed_link") == false
fail_with("preview must not link public audio") unless preview_entry.fetch("contains_public_audio_link") == false
fail_with("preview must not show old name") unless preview_entry.fetch("contains_old_show_name") == false
fail_with("latest public versions must be hidden by delete markers") unless verification.dig("s3_origin_state", "public_prefix_latest_versions") == 0
fail_with("latest public delete markers mismatch") unless verification.dig("s3_origin_state", "public_prefix_latest_delete_markers") == expected_keys.size

negative = evidence.fetch("negative_authority")
%w[
  provider_submission_performed
  provider_directory_mutation_performed
  private_archive_bucket_deleted_or_purged
  s3_version_purge_performed
  credentials_or_private_receipts_committed
].each do |field|
  fail_with("negative authority violation: #{field}") unless negative.fetch(field) == false
end

fail_with("preview missing noindex") unless preview.include?('<meta name="robots" content="noindex,nofollow">')
fail_with("preview missing The Cognitive Stack title") unless preview.include?("<title>The Cognitive Stack</title>")
fail_with("preview still references public podcast path") if preview.include?("../../podcast") || preview.include?("/podcast/feed.xml") || preview.include?("podcast/audio/")
fail_with("preview still contains old show name") if preview.include?("Cognitive Spacetime") || preview.include?("Synthetic Minds")
fail_with("preview logo must be local preview asset") unless preview.include?("./assets/agent-logic-logo.svg")
fail_with("preview audio withholding notice missing") unless preview.include?("audio remains withheld until public launch approval")
fail_with("preview logo asset missing") unless File.file?(logo_path)

launch_readiness = File.read(launch_readiness_path)
fail_with("launch readiness still claims production publication") if launch_readiness.include?("Published to production")
fail_with("launch readiness must name public route as withheld") unless launch_readiness.include?("Public production route | Withheld")
fail_with("launch readiness must preserve production-ready candidate truth") unless launch_readiness.include?("Ready but withheld")
fail_with("launch readiness must point to hidden preview") unless launch_readiness.include?("https://agent-logic.ai/_preview/podcast/")

publication_hold = File.read(publication_hold_path)
fail_with("publication hold must prohibit public deployment") unless publication_hold.include?("Do not copy, sync, or publish")
fail_with("publication hold must name the hidden preview route") unless publication_hold.include?("https://agent-logic.ai/_preview/podcast/")
fail_with("publication hold must name public route as withheld") unless publication_hold.include?("https://agent-logic.ai/podcast/")

runbook = File.read(runbook_path)
fail_with("runbook must gate promotion after #660") unless runbook.include?("As of issue\n#660") && runbook.include?("explicit operator approval for public launch")

puts JSON.pretty_generate(
  {
    status: "pass",
    issue: 660,
    checked: {
      delete_manifest_keys: expected_keys.size,
      public_urls_blocked: verification.fetch("public_urls").size,
      hidden_preview_verified: true,
      public_launch_candidate_preserved_but_withheld: true,
      negative_authority_verified: true
    }
  }
)

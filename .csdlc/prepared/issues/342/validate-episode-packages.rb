#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
PACKAGES = ROOT.join("demos/podcast/episode-packages")
EXPECTED = [
  "001-meet-the-ai-coworkers",
  "002-can-an-ai-be-a-good-teammate",
  "003-the-promise-and-weirdness-of-talking-to-machines",
  "004-what-should-we-let-ai-do-for-us",
  "005-can-ai-help-us-think-better",
  "006-the-new-creative-room",
  "007-trust-receipts-and-proof",
  "008-local-ai-vs-cloud-ai",
  "009-when-ai-gets-stuck",
  "010-what-does-a-weekly-ai-studio-look-like"
].freeze

def fail!(reason)
  warn JSON.generate(schema: "adl.wp24a.episode_packages_validation.v1", status: "fail", reason: reason)
  exit 1
end

index_path = PACKAGES.join("package-index.json")
fail!("package index missing") unless index_path.file?
index = JSON.parse(index_path.read)

fail!("index schema mismatch") unless index["schema"] == "agent_logic.podcast.episode_package_index.v1"
fail!("wrong issue") unless index["issue"] == 342
fail!("wrong working title") unless index["show_working_title"] == "The Cognitive Stack"
fail!("index must remain source checkpoint") unless index["status"] == "source_package_checkpoint"
fail!("publication overclaim") unless index["publication_claimed"] == false
fail!("production feed overclaim") unless index["production_feed_claimed"] == false
fail!("directory submission overclaim") unless index["directory_submission_claimed"] == false
fail!("episode denominator mismatch") unless index["episode_count"] == 10 && index["episodes"] == EXPECTED

ready = []
EXPECTED.each_with_index do |slug, offset|
  path = PACKAGES.join(slug, "package.json")
  fail!("missing package #{slug}") unless path.file?
  package = JSON.parse(path.read)
  fail!("schema mismatch #{slug}") unless package["schema"] == "agent_logic.podcast.episode_source_package.v1"
  fail!("episode number mismatch #{slug}") unless package["episode"] == offset + 1
  fail!("slug mismatch #{slug}") unless package["slug"] == slug
  fail!("title missing #{slug}") unless package["title"].is_a?(String) && !package["title"].strip.empty?
  fail!("wrong show title #{slug}") unless package["show_working_title"] == "The Cognitive Stack"
  fail!("status mismatch #{slug}") unless package["status"] == "source_package_ready_audio_pending"
  fail!("premise missing #{slug}") unless package["premise"].is_a?(String) && package["premise"].length >= 40
  fail!("agent panel too small #{slug}") unless package["agent_panel"].is_a?(Array) && package["agent_panel"].length >= 3
  fail!("takeaway missing #{slug}") unless package["listener_takeaway"].is_a?(String) && package["listener_takeaway"].length >= 40
  fail!("promotion hooks missing #{slug}") unless package["promotes"].is_a?(Array) && package["promotes"].length >= 2
  fail!("publication overclaim #{slug}") unless package["publication_claimed"] == false
  audio = package["audio"]
  fail!("audio object mismatch #{slug}") unless audio.is_a?(Hash)
  fail!("audio must remain pending #{slug}") unless audio["final_mp3"].nil? && audio["archive_audio"].nil? && audio["status"] == "pending_final_package_proof"
  ready << slug
end

puts JSON.generate(
  schema: "adl.wp24a.episode_packages_validation.v1",
  status: "pass",
  checkpoint: "source_package_ready_audio_pending",
  source_packages: ready.length,
  final_audio_packages: 0,
  terminal_ready: false,
  publication_claimed: false
)

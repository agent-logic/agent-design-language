#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "rexml/document"

ROOT = File.expand_path("../../../../", __dir__)
PACKET = File.join(ROOT, "docs/milestones/v0.92.1/review/podcast_submission_264")
FILES = {
  "README.md" => File.join(PACKET, "README.md"),
  "operator-authorization-template.md" => File.join(PACKET, "operator-authorization-template.md"),
  "submission-ledger.json" => File.join(PACKET, "submission-ledger.json"),
  "monitoring-and-rollback.md" => File.join(PACKET, "monitoring-and-rollback.md"),
  "parent-51-handoff.md" => File.join(PACKET, "parent-51-handoff.md")
}.freeze
FEED = File.join(ROOT, "demos/podcast/feed.xml")
IDENTITY = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/show-identity.json")

EXPECTED_PROVIDERS = %w[
  apple_podcasts
  spotify_for_creators
  amazon_music_for_podcasters
  youtube_rss_ingestion
].freeze

def fail!(reason)
  warn JSON.generate(schema: "agent_logic.podcast.submission_gate_validation.v1", status: "failed", reason: reason)
  exit 1
end

def read(path)
  File.read(path, encoding: "UTF-8")
rescue Errno::ENOENT
  fail!("missing required file: #{path.sub(ROOT + "/", "")}")
end

texts = FILES.transform_values { |path| read(path) }
combined = texts.values.join("\n")
identity = JSON.parse(read(IDENTITY))
feed = REXML::Document.new(read(FEED))
ledger = JSON.parse(texts["submission-ledger.json"])

fail!("identity packet is not approved for The Cognitive Stack") unless identity["approval_status"] == "operator_approved" && identity.dig("show", "title") == "The Cognitive Stack"
fail!("feed title mismatch") unless feed.elements["rss/channel/title"]&.text == "The Cognitive Stack"

required_phrases = [
  "Status: non-submission gate complete",
  "No provider submission has been performed",
  "Explicit future operator authorization is still required",
  "https://agent-logic.ai/podcast/feed.xml",
  "The Cognitive Stack",
  "podcast@agent-logic.ai",
  "Do not retain credentials, verification codes, recovery codes, mailbox contents, cookies, tokens, or private screenshots",
  "Do not activate destination links until the provider listing is live and verified",
  "Issue #51 remains open unless the operator explicitly accepts this blocked disposition for parent routing"
]
required_phrases.each do |phrase|
  fail!("missing required boundary phrase #{phrase}") unless combined.include?(phrase)
end

EXPECTED_PROVIDERS.each do |provider|
  fail!("missing provider #{provider}") unless combined.include?(provider)
end

prohibited_patterns = [
  /(password|api[_ -]?key|oauth[_ -]?token|cookie|recovery[_ -]?code|verification[_ -]?code)\s*[:=]\s*[^,\s`]+/i,
  /submitted_at_utc"\s*:\s*"20[0-9]{2}-/i,
  /"status"\s*:\s*"(submitted|verification_pending|pending_review|active|rejected|private_or_scheduled)"/i,
  /(apple|spotify|amazon|youtube).*(accepted|approved|live listing|published in directory)/i
]
prohibited_patterns.each do |pattern|
  fail!("packet appears to retain prohibited submission or secret material: #{pattern.source}") if combined.match?(pattern)
end

fail!("ledger schema mismatch") unless ledger["schema"] == "agent_logic.podcast.submission_ledger.v1"
fail!("ledger show mismatch") unless ledger["show"] == "The Cognitive Stack"
fail!("ledger feed mismatch") unless ledger["feed_url"] == "https://agent-logic.ai/podcast/feed.xml"
entries = ledger["entries"]
fail!("ledger entries missing") unless entries.is_a?(Array)
fail!("ledger provider count mismatch") unless entries.size == EXPECTED_PROVIDERS.size

providers = entries.map { |entry| entry["provider"] }
fail!("ledger provider set mismatch") unless providers == EXPECTED_PROVIDERS

entries.each do |entry|
  fail!("ledger entry submitted unexpectedly") unless entry["status"] == "not_authorized"
  fail!("ledger submitted_at_utc must be null before authorization") unless entry["submitted_at_utc"].nil?
  fail!("ledger canonical_url_or_id must be null before authorization") unless entry["canonical_url_or_id"].nil?
  fail!("ledger must not retain secret material") unless entry.dig("evidence", "secret_material_retained") == false
  fail!("ledger redacted summary missing") unless entry.dig("evidence", "redacted_summary").is_a?(String) && !entry.dig("evidence", "redacted_summary").empty?
end

puts JSON.generate(
  schema: "agent_logic.podcast.submission_gate_validation.v1",
  status: "passed",
  show: "The Cognitive Stack",
  feed: "https://agent-logic.ai/podcast/feed.xml",
  providers: EXPECTED_PROVIDERS,
  submission_claimed: false,
  public_launch_claimed: false,
  destination_links_activated: false,
  operator_authorization_required: true
)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "rexml/document"

ROOT = File.expand_path("../../../../", __dir__)
PACKET = File.join(ROOT, "docs/milestones/v0.92.1/review/podcast_directory_263")
RUNBOOK = File.join(PACKET, "provider-runbooks.md")
PREFLIGHT = File.join(PACKET, "operator-preflight.md")
README = File.join(PACKET, "README.md")
SCHEMA = File.join(PACKET, "submission-ledger.schema.json")
FEED = File.join(ROOT, "demos/podcast/feed.xml")
IDENTITY = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/show-identity.json")
HOSTING_VALIDATOR = File.join(ROOT, ".csdlc/prepared/issues/262/validate-podcast-hosting.rb")

def fail!(reason)
  warn JSON.generate(schema: "agent_logic.podcast.directory_runbooks_validation.v1", status: "failed", reason: reason)
  exit 1
end

def read(path)
  File.read(path, encoding: "UTF-8")
rescue Errno::ENOENT
  fail!("missing required file: #{path.sub(ROOT + "/", "")}")
end

runbook = read(RUNBOOK)
preflight = read(PREFLIGHT)
readme = read(README)
schema = JSON.parse(read(SCHEMA))
identity = JSON.parse(read(IDENTITY))
feed_text = read(FEED)

fail!("identity packet is not approved for The Cognitive Stack") unless identity["approval_status"] == "operator_approved" && identity.dig("show", "title") == "The Cognitive Stack"
feed = REXML::Document.new(feed_text)
fail!("feed title mismatch") unless feed.elements["rss/channel/title"]&.text == "The Cognitive Stack"
fail!("hosting validator missing") unless File.executable?(HOSTING_VALIDATOR) || File.file?(HOSTING_VALIDATOR)

official_sources = [
  "https://podcasters.apple.com/support/897-submit-a-show",
  "https://support.spotify.com/us/creators/article/getting-your-show-on-spotify/",
  "https://support.spotify.com/us/creators/article/multiple-shows-under-one-account/",
  "https://support.spotify.com/us/creators/article/finding-and-enabling-your-rss-feed/",
  "https://support.spotify.com/sg-en/creators/article/claiming-your-podcast-on-spotify-for-creators/",
  "https://support.spotify.com/mw/creators/article/adding-a-new-show/",
  "https://podcasters.amazon.com/submit-rss",
  "https://podcasters.amazon.com/frequently-asked-questions",
  "https://support.google.com/youtube/answer/13525207?hl=en"
]
official_sources.each do |url|
  fail!("missing official source #{url}") unless runbook.include?(url)
end

%w[Apple\ Podcasts Spotify\ for\ Creators Amazon\ Music\ for\ Podcasters YouTube\ RSS\ ingestion].each do |provider|
  fail!("missing provider section #{provider}") unless runbook.include?(provider)
end

required_phrases = [
  "Status: prepared 2026-09-02 from current official provider instructions",
  "This is not submission authority",
  "https://agent-logic.ai/podcast/feed.xml",
  "https://agent-logic.ai/podcast/",
  "podcast@agent-logic.ai",
  "Stop before Publish",
  "verification email/code",
  "do not retain the code",
  "private or otherwise explicitly selected visibility",
  "No directory submission",
  "credentials",
  "verification codes"
]
combined = [runbook, preflight, readme].join("\n")
required_phrases.each do |phrase|
  fail!("missing required boundary phrase #{phrase}") unless combined.include?(phrase)
end

prohibited = /(password|api[_ -]?key|oauth[_ -]?token|recovery[_ -]?code|verification[_ -]?code)\s*[:=]\s*[^,\s`]+/i
fail!("packet appears to retain secret-like material") if combined.match?(prohibited)

fail!("ledger schema id mismatch") unless schema["$id"] == "agent_logic.podcast.submission_ledger.v1"
entries = schema.dig("properties", "entries", "items", "properties") || fail!("ledger entries schema missing")
%w[provider owner submitted_at_utc status canonical_url_or_id evidence follow_up].each do |field|
  fail!("ledger field missing #{field}") unless entries.key?(field)
end
providers = entries.dig("provider", "enum")
expected_providers = %w[apple_podcasts spotify_for_creators amazon_music_for_podcasters youtube_rss_ingestion]
fail!("ledger provider enum mismatch") unless providers == expected_providers
fail!("ledger does not prohibit secret material") unless entries.dig("evidence", "properties", "secret_material_retained", "const") == false

puts JSON.generate(
  schema: "agent_logic.podcast.directory_runbooks_validation.v1",
  status: "passed",
  show: "The Cognitive Stack",
  feed: "https://agent-logic.ai/podcast/feed.xml",
  official_sources_sampled_utc_date: "2026-09-02",
  providers: expected_providers,
  submission_claimed: false,
  public_launch_claimed: false
)

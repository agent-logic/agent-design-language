#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "rexml/document"

ROOT = File.expand_path("../../../../", __dir__)
FEED = File.join(ROOT, "demos/podcast/feed.xml")
SHOW_PAGE = File.join(ROOT, "demos/podcast/index.html")
PREVIEW_PAGE = File.join(ROOT, "demos/_preview/podcast/index.html")
EPISODE_PAGE = File.join(ROOT, "demos/podcast/episodes/meet-the-ai-coworkers/index.html")
EPISODE_JSON = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json")
ENCLOSURE_JSON = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/rss-enclosure.json")
IDENTITY = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/show-identity.json")
RIGHTS = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/artwork-rights.json")
MAILBOX = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/mailbox-readiness.json")
QA_REPORT = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/qa-report.md")

def fail!(reason)
  warn JSON.generate(schema: "agent_logic.podcast.hosting_validation.v1", status: "failed", reason: reason)
  exit 1
end

def read(path)
  File.read(path, encoding: "UTF-8")
rescue Errno::ENOENT
  fail!("missing required file: #{path.sub(ROOT + "/", "")}")
end

def load_json(path)
  JSON.parse(read(path))
rescue JSON::ParserError => e
  fail!("invalid JSON #{path.sub(ROOT + "/", "")}: #{e.message}")
end

identity = load_json(IDENTITY)
rights = load_json(RIGHTS)
mailbox = load_json(MAILBOX)
episode = load_json(EPISODE_JSON)
enclosure = load_json(ENCLOSURE_JSON)

fail!("identity packet title is not The Cognitive Stack") unless identity.dig("show", "title") == "The Cognitive Stack"
fail!("identity packet is not operator approved") unless identity["approval_status"] == "operator_approved"
fail!("artwork rights are not operator confirmed") unless rights["status"] == "operator_confirmed" && rights["publication_authorized"] == true
fail!("mailbox is not verified") unless mailbox["status"] == "verified_received" && mailbox["publication_authorized"] == true

feed_text = read(FEED)
begin
  feed = REXML::Document.new(feed_text)
rescue REXML::ParseException => e
  fail!("RSS XML parse failed: #{e.message}")
end

channel = feed.elements["rss/channel"] || fail!("RSS channel missing")
fail!("RSS title mismatch") unless channel.elements["title"]&.text == "The Cognitive Stack"
fail!("RSS link mismatch") unless channel.elements["link"]&.text == "https://agent-logic.ai/podcast/"
fail!("RSS owner email mismatch") unless feed_text.include?("<itunes:email>podcast@agent-logic.ai</itunes:email>")
fail!("RSS artwork URL mismatch") unless feed_text.include?('<itunes:image href="https://agent-logic.ai/podcast/artwork.png" />')
fail!("RSS contains prohibited local or preview URL") if feed_text.match?(%r{(?:localhost|127\.0\.0\.1|file:|_preview|/private/tmp|/var/folders)})

items = channel.get_elements("item")
fail!("expected exactly one launch-feed item") unless items.length == 1
item = items.first
expected_guid = "agent-logic-the-cognitive-stack-episode-001"
fail!("episode GUID mismatch") unless item.elements["guid"]&.text == expected_guid
enclosure_element = item.elements["enclosure"] || fail!("enclosure missing")
fail!("enclosure URL mismatch") unless enclosure_element.attributes["url"] == "https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3"
fail!("enclosure type mismatch") unless enclosure_element.attributes["type"] == "audio/mpeg"
fail!("enclosure length mismatch") unless enclosure_element.attributes["length"] == "22804249"
fail!("itunes duration mismatch") unless feed_text.include?("<itunes:duration>00:18:32</itunes:duration>")

fail!("episode JSON show title mismatch") unless episode["show_title"] == "The Cognitive Stack"
fail!("episode JSON GUID mismatch") unless episode["guid"] == expected_guid
fail!("enclosure JSON GUID mismatch") unless enclosure["guid"] == expected_guid
fail!("enclosure JSON URL mismatch") unless enclosure["url"] == enclosure_element.attributes["url"]
fail!("episode/enclosure byte mismatch") unless episode["audio_bytes"] == enclosure["bytes"] && enclosure["bytes"] == 22_804_249
fail!("episode/enclosure duration mismatch") unless episode["audio_duration"] == enclosure["duration"] && enclosure["duration"] == "00:18:32"

audio_path = File.join(ROOT, "demos/podcast/audio/meet-the-ai-coworkers.mp3")
audio = File.binread(audio_path)
fail!("audio byte length mismatch") unless audio.bytesize == enclosure["bytes"]
fail!("audio SHA-256 mismatch") unless Digest::SHA256.hexdigest(audio) == enclosure["sha256"]
fail!("audio ID3 metadata still names Cognitive Spacetime") if audio.include?("Cognitive Spacetime")
fail!("audio ID3 metadata missing The Cognitive Stack") unless audio.include?("The Cognitive Stack")

artwork_path = File.join(ROOT, "demos/podcast/artwork.png")
artwork = File.binread(artwork_path)
fail!("artwork byte length mismatch") unless artwork.bytesize == identity.dig("artwork", "bytes")
fail!("artwork SHA-256 mismatch") unless Digest::SHA256.hexdigest(artwork) == identity.dig("artwork", "sha256")

{
  "demos/podcast/index.html" => SHOW_PAGE,
  "demos/_preview/podcast/index.html" => PREVIEW_PAGE,
  "demos/podcast/episodes/meet-the-ai-coworkers/index.html" => EPISODE_PAGE
}.each do |label, path|
  html = read(path)
  fail!("#{label} missing The Cognitive Stack") unless html.include?("The Cognitive Stack")
  fail!("#{label} has stale page title") if html.match?(%r{<title>[^<]*Cognitive Spacetime}i)
  fail!("#{label} has stale show transcript copy") if html.include?("Welcome back to Cognitive Spacetime")
  fail!("#{label} contains prohibited local URL") if html.match?(%r{(?:localhost|127\.0\.0\.1|file:|/private/tmp|/var/folders)})
end

qa_report = read(QA_REPORT)
fail!("QA report audio hash mismatch") unless qa_report.include?(enclosure["sha256"])
fail!("QA report still names Cognitive Spacetime metadata") if qa_report.include?("Artist: Cognitive Spacetime") || qa_report.include?("Album: Cognitive Spacetime")
fail!("QA report missing The Cognitive Stack metadata") unless qa_report.include?("Artist: The Cognitive Stack") && qa_report.include?("Album: The Cognitive Stack")

puts JSON.generate(
  schema: "agent_logic.podcast.hosting_validation.v1",
  status: "passed",
  show: "The Cognitive Stack",
  feed: "https://agent-logic.ai/podcast/feed.xml",
  page: "https://agent-logic.ai/podcast/",
  guid: expected_guid,
  enclosure_bytes: enclosure["bytes"],
  audio_sha256: enclosure["sha256"],
  artwork_sha256: identity.dig("artwork", "sha256"),
  publication_claimed: false
)

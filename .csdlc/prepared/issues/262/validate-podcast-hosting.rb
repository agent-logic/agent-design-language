#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "base64"
require "json"
require "rexml/document"
require "time"

ROOT = File.expand_path("../../../../", __dir__)
FEED = File.join(ROOT, "demos/podcast/feed.xml")
SHOW_PAGE = File.join(ROOT, "demos/podcast/index.html")
PREVIEW_PAGE = File.join(ROOT, "demos/_preview/podcast/index.html")
EPISODE_PAGE = File.join(ROOT, "demos/podcast/episodes/meet-the-ai-coworkers/index.html")
EPISODE_JSON = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json")
ENCLOSURE_JSON = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/rss-enclosure.json")
SOURCE_PACKET = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/source-packet.md")
RUNBOOK = File.join(ROOT, "demos/podcast/S3_CLOUDFRONT_RUNBOOK.md")
CREATOR_WORKFLOW = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/CREATOR_WORKFLOW.md")
STORAGE_MANIFEST = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/storage-manifest.json")
S3_OBJECT_INVENTORY = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/s3-object-inventory.json")
IDENTITY = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/show-identity.json")
RIGHTS = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/artwork-rights.json")
MAILBOX = File.join(ROOT, "docs/milestones/v0.92/review/podcast_identity_261/mailbox-readiness.json")
QA_REPORT = File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers/qa-report.md")
HTTP_PLAYBACK_PROOF = File.join(ROOT, ".csdlc/evidence/262/http-playback-proof.json")
HTTP_PLAYBACK_NATIVE_PROOF = File.join(ROOT, ".csdlc/evidence/262/http-playback-native-proof.json")
HTTP_PLAYBACK_BROWSER_PROOF = File.join(ROOT, ".csdlc/evidence/262/http-playback-browser-proof.json")
HTTP_PLAYBACK_IOS_SAFARI_PROOF = File.join(ROOT, ".csdlc/evidence/262/http-playback-ios-safari-proof.json")
SOURCE_MANIFEST_FILES = [
  "demos/podcast/feed.xml",
  "demos/podcast/index.html",
  "demos/podcast/episodes/meet-the-ai-coworkers/index.html",
  "demos/podcast/audio/meet-the-ai-coworkers.mp3",
  ".csdlc/prepared/issues/262/record-podcast-http-playback.rb",
  ".csdlc/prepared/issues/262/validate-podcast-hosting.rb",
  "adl/tools/record_podcast_native_playback.sh",
  "adl/tools/record_podcast_browser_playback.mjs",
  "adl/tools/record_podcast_ios_safari_playback.sh",
  "demos/podcast/S3_CLOUDFRONT_RUNBOOK.md",
  "demos/podcast/episodes/001-meet-the-ai-coworkers/CREATOR_WORKFLOW.md",
  "demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json",
  "demos/podcast/episodes/001-meet-the-ai-coworkers/source-packet.md",
  "demos/podcast/episodes/001-meet-the-ai-coworkers/storage-manifest.json",
  "demos/podcast/episodes/001-meet-the-ai-coworkers/s3-object-inventory.json"
].freeze

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

def canonical_json(value)
  case value
  when Hash
    "{" + value.keys.sort.map { |key| JSON.generate(key.to_s) + ":" + canonical_json(value[key]) }.join(",") + "}"
  when Array
    "[" + value.map { |entry| canonical_json(entry) }.join(",") + "]"
  else
    JSON.generate(value)
  end
end

def source_manifest
  files = SOURCE_MANIFEST_FILES.map do |rel|
    full = File.join(ROOT, rel)
    fail!("source manifest file missing: #{rel}") unless File.file?(full)
    {
      "path" => rel,
      "bytes" => File.size(full),
      "sha256" => Digest::SHA256.file(full).hexdigest
    }
  end
  payload = {
    "schema" => "agent_logic.podcast.source_manifest.v1",
    "digest_algorithm" => "sha256(canonical-json)",
    "files" => files
  }
  payload["digest"] = Digest::SHA256.hexdigest(canonical_json(payload))
  payload
end

def local_archive_path_for_key(key)
  if key.include?("/package/")
    File.join(ROOT, "demos/podcast/episodes/001-meet-the-ai-coworkers", key.split("/package/", 2).last)
  elsif key.include?("/media/")
    File.join(ROOT, "demos/podcast/audio", key.split("/media/", 2).last)
  end
end

identity = load_json(IDENTITY)
rights = load_json(RIGHTS)
mailbox = load_json(MAILBOX)
episode = load_json(EPISODE_JSON)
enclosure = load_json(ENCLOSURE_JSON)
storage_manifest = load_json(STORAGE_MANIFEST)
s3_object_inventory = load_json(S3_OBJECT_INVENTORY)
http_playback_proof = load_json(HTTP_PLAYBACK_PROOF)
wrapper_playback_proofs = {
  HTTP_PLAYBACK_NATIVE_PROOF => %w[desktop-safari desktop-chrome],
  HTTP_PLAYBACK_BROWSER_PROOF => %w[desktop-safari desktop-chrome mobile-safari android-chrome],
  HTTP_PLAYBACK_IOS_SAFARI_PROOF => %w[mobile-safari]
}.transform_keys { |path| path.sub(ROOT + "/", "") }

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

source_packet = read(SOURCE_PACKET)
fail!("source packet canonical GUID mismatch") unless source_packet.include?("Canonical GUID: `#{expected_guid}`")
fail!("source packet still names old canonical GUID") if source_packet.include?("agent-logic-cognitive-spacetime-episode-001")

{
  "demos/podcast/S3_CLOUDFRONT_RUNBOOK.md" => RUNBOOK,
  "demos/podcast/episodes/001-meet-the-ai-coworkers/CREATOR_WORKFLOW.md" => CREATOR_WORKFLOW,
  "demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json" => EPISODE_JSON,
  "demos/podcast/episodes/001-meet-the-ai-coworkers/source-packet.md" => SOURCE_PACKET,
  "demos/podcast/episodes/001-meet-the-ai-coworkers/storage-manifest.json" => STORAGE_MANIFEST,
  "demos/podcast/episodes/001-meet-the-ai-coworkers/s3-object-inventory.json" => S3_OBJECT_INVENTORY
}.each do |label, path|
  text = read(path)
  fail!("#{label} still contains stale production token CognitiveSpacetime") if text.include?("CognitiveSpacetime")
  fail!("#{label} still contains stale production token cognitive-spacetime") if text.include?("cognitive-spacetime")
  fail!("#{label} contains prohibited local temp path") if text.match?(%r{/private/tmp|/var/folders})
end

inventory_objects = s3_object_inventory.fetch("objects")
inventory_size_sum = inventory_objects.sum { |object| object.fetch("size") }
fail!("S3 object inventory object count mismatch") unless s3_object_inventory["object_count"] == inventory_objects.length
fail!("S3 object inventory total byte sum mismatch") unless s3_object_inventory["total_bytes"] == inventory_size_sum
fail!("storage manifest archive object count mismatch") unless storage_manifest["archive_object_count"] == s3_object_inventory["object_count"]
fail!("storage manifest archive total bytes mismatch") unless storage_manifest["archive_total_bytes"] == s3_object_inventory["total_bytes"]

storage_manifest.fetch("critical_objects").each do |object|
  key = object.fetch("key")
  local_path = local_archive_path_for_key(key)
  next unless local_path && File.file?(local_path)

  label = key.sub("archive/the-cognitive-stack/episodes/001/", "")
  local_bytes = File.size(local_path)
  local_sha256 = Digest::SHA256.file(local_path).hexdigest
  fail!("storage manifest #{label} byte count mismatch") unless object["bytes"] == local_bytes
  fail!("storage manifest #{label} SHA-256 mismatch") unless object["sha256"] == local_sha256
  expected_s3_checksum = Base64.strict_encode64([local_sha256].pack("H*"))
  fail!("storage manifest #{label} checksum mismatch") unless object["s3_checksum_sha256"] == expected_s3_checksum
end

inventory_objects.each do |object|
  key = object.fetch("key")
  local_path = local_archive_path_for_key(key)
  next unless local_path && File.file?(local_path)

  label = key.sub("archive/the-cognitive-stack/episodes/001/", "")
  fail!("S3 object inventory #{label} size mismatch") unless object["size"] == File.size(local_path)
  next if key.include?("/media/")

  expected_etag = "\"#{Digest::MD5.file(local_path).hexdigest}\""
  fail!("S3 object inventory #{label} etag mismatch") unless object["etag"] == expected_etag
end

fail!("HTTP playback proof schema mismatch") unless http_playback_proof["schema"] == "agent_logic.podcast.http_playback_proof.v1"
fail!("HTTP playback proof did not pass") unless http_playback_proof["status"] == "passed"
fail!("HTTP playback proof candidate audio path mismatch") unless http_playback_proof.dig("candidate", "audio") == "demos/podcast/audio/meet-the-ai-coworkers.mp3"
fail!("HTTP playback proof candidate feed path mismatch") unless http_playback_proof.dig("candidate", "feed") == "demos/podcast/feed.xml"
fail!("HTTP playback proof candidate podcast page mismatch") unless http_playback_proof.dig("candidate", "podcast_page") == "demos/podcast/index.html"
fail!("HTTP playback proof candidate episode page mismatch") unless http_playback_proof.dig("candidate", "episode_page") == "demos/podcast/episodes/meet-the-ai-coworkers/index.html"
fail!("HTTP playback proof byte count mismatch") unless http_playback_proof.dig("candidate", "audio_bytes") == enclosure["bytes"]
fail!("HTTP playback proof audio SHA mismatch") unless http_playback_proof.dig("candidate", "audio_sha256") == enclosure["sha256"]
fail!("HTTP playback proof server binding mismatch") unless http_playback_proof.dig("server", "bind") == "127.0.0.1"
fail!("HTTP playback proof range support mismatch") unless http_playback_proof.dig("server", "range_support") == "single-range bytes"

expected_source_manifest = source_manifest
fail!("HTTP playback proof source manifest mismatch") unless http_playback_proof["source_manifest"] == expected_source_manifest
fail!("HTTP playback proof binding digest mismatch") unless http_playback_proof.dig("proof_binding", "source_manifest_digest") == expected_source_manifest.fetch("digest")
fail!("HTTP playback proof producer missing") unless http_playback_proof.dig("proof_binding", "producer").to_s.start_with?("codex:")

begin
  proof_generated_at = Time.parse(http_playback_proof["generated_at"])
  fail!("HTTP playback proof timestamp missing") unless proof_generated_at
  fail!("HTTP playback proof timestamp is not UTC") unless http_playback_proof["generated_at"].end_with?("Z")
rescue ArgumentError, TypeError
  fail!("HTTP playback proof timestamp invalid")
end

expected_profiles = %w[desktop-safari desktop-chrome mobile-safari android-chrome]
profiles = http_playback_proof["profiles"]
fail!("HTTP playback proof profiles missing") unless profiles.is_a?(Hash)
fail!("HTTP playback proof profiles mismatch") unless profiles.keys.sort == expected_profiles.sort

expected_file_sha = {
  "/podcast/feed.xml" => Digest::SHA256.hexdigest(File.binread(FEED)),
  "/podcast/" => Digest::SHA256.hexdigest(File.binread(SHOW_PAGE)),
  "/podcast/episodes/meet-the-ai-coworkers/" => Digest::SHA256.hexdigest(File.binread(EPISODE_PAGE))
}
expected_first_range_sha = Digest::SHA256.hexdigest(audio.byteslice(0, 1024))
expected_tail_start = enclosure["bytes"] - 1024
expected_tail_range_sha = Digest::SHA256.hexdigest(audio.byteslice(expected_tail_start, 1024))

expected_profiles.each do |profile|
  profile_receipt = profiles[profile]
  fail!("HTTP playback proof #{profile} receipt missing") unless profile_receipt.is_a?(Hash)
  fail!("HTTP playback proof #{profile} user agent missing") unless profile_receipt["user_agent"].to_s.length >= 20
  checks = profile_receipt["checks"]
  fail!("HTTP playback proof #{profile} checks missing") unless checks.is_a?(Array)

  head_feed = checks.find { |check| check["method"] == "HEAD" && check["path"] == "/podcast/feed.xml" }
  fail!("HTTP playback proof #{profile} missing HEAD feed check") unless head_feed
  fail!("HTTP playback proof #{profile} HEAD feed did not return 200") unless head_feed["status"] == 200
  fail!("HTTP playback proof #{profile} HEAD feed returned a body") unless head_feed["body_bytes"] == 0
  fail!("HTTP playback proof #{profile} HEAD feed lacks byte-range support") unless head_feed["accept_ranges"] == "bytes"

  expected_file_sha.each do |path, sha256|
    get_check = checks.find { |check| check["method"] == "GET" && check["path"] == path && check["status"] == 200 }
    fail!("HTTP playback proof #{profile} missing GET #{path} check") unless get_check
    fail!("HTTP playback proof #{profile} GET #{path} SHA mismatch") unless get_check["body_sha256"] == sha256
  end

  head_audio = checks.find { |check| check["method"] == "HEAD" && check["path"] == "/podcast/audio/meet-the-ai-coworkers.mp3" }
  fail!("HTTP playback proof #{profile} missing HEAD audio check") unless head_audio
  fail!("HTTP playback proof #{profile} HEAD audio did not return 200") unless head_audio["status"] == 200
  fail!("HTTP playback proof #{profile} HEAD audio type mismatch") unless head_audio["content_type"] == "audio/mpeg"
  fail!("HTTP playback proof #{profile} HEAD audio length mismatch") unless head_audio["content_length_header"] == enclosure["bytes"].to_s
  fail!("HTTP playback proof #{profile} HEAD audio returned a body") unless head_audio["body_bytes"] == 0
  fail!("HTTP playback proof #{profile} HEAD audio lacks byte-range support") unless head_audio["accept_ranges"] == "bytes"

  first_range = checks.find do |check|
    check["method"] == "GET" &&
      check["path"] == "/podcast/audio/meet-the-ai-coworkers.mp3" &&
      check["status"] == 206 &&
      check["content_range"] == "bytes 0-1023/#{enclosure["bytes"]}"
  end
  fail!("HTTP playback proof #{profile} missing first-byte-range audio check") unless first_range
  fail!("HTTP playback proof #{profile} first range length mismatch") unless first_range["body_bytes"] == 1024 && first_range["content_length_header"] == "1024"
  fail!("HTTP playback proof #{profile} first range SHA mismatch") unless first_range["body_sha256"] == expected_first_range_sha

  tail_range = checks.find do |check|
    check["method"] == "GET" &&
      check["path"] == "/podcast/audio/meet-the-ai-coworkers.mp3" &&
      check["status"] == 206 &&
      check["content_range"] == "bytes #{expected_tail_start}-#{enclosure["bytes"] - 1}/#{enclosure["bytes"]}"
  end
  fail!("HTTP playback proof #{profile} missing tail-byte-range audio check") unless tail_range
  fail!("HTTP playback proof #{profile} tail range length mismatch") unless tail_range["body_bytes"] == 1024 && tail_range["content_length_header"] == "1024"
  fail!("HTTP playback proof #{profile} tail range SHA mismatch") unless tail_range["body_sha256"] == expected_tail_range_sha
end

wrapper_playback_proofs.each do |relative_proof_path, expected_wrapper_profiles|
  wrapper_proof = load_json(File.join(ROOT, relative_proof_path))
  fail!("#{relative_proof_path} schema mismatch") unless wrapper_proof["schema"] == "agent_logic.podcast.http_playback_proof.v1"
  fail!("#{relative_proof_path} did not pass") unless wrapper_proof["status"] == "passed"
  fail!("#{relative_proof_path} audio path mismatch") unless wrapper_proof.dig("candidate", "audio") == "demos/podcast/audio/meet-the-ai-coworkers.mp3"
  fail!("#{relative_proof_path} audio SHA mismatch") unless wrapper_proof.dig("candidate", "audio_sha256") == enclosure["sha256"]
  fail!("#{relative_proof_path} source manifest mismatch") unless wrapper_proof["source_manifest"] == expected_source_manifest
  fail!("#{relative_proof_path} binding digest mismatch") unless wrapper_proof.dig("proof_binding", "source_manifest_digest") == expected_source_manifest.fetch("digest")
  wrapper_profiles = wrapper_proof["profiles"]
  fail!("#{relative_proof_path} profiles missing") unless wrapper_profiles.is_a?(Hash)
  fail!("#{relative_proof_path} profiles mismatch") unless wrapper_profiles.keys.sort == expected_wrapper_profiles.sort
end

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

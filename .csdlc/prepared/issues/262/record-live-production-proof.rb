#!/usr/bin/env ruby
# frozen_string_literal: true

require "base64"
require "digest"
require "fileutils"
require "json"
require "net/http"
require "optparse"
require "time"
require "uri"

ROOT = File.expand_path("../../../../", __dir__)
DEFAULT_OUTPUT = File.join(ROOT, ".csdlc/evidence/262/live-production/public-production-proof.json")
PUBLIC_BASE_URL = "https://agent-logic.ai/podcast/"
BUCKET = "agent-logic-ai-origin-agentlogic"
CLOUDFRONT_DISTRIBUTION_ID = "E3C29FMX32KDDU"
AUDIO_REL = "demos/podcast/audio/meet-the-ai-coworkers.mp3"
AUDIO_PATH = File.join(ROOT, AUDIO_REL)
FEED_REL = "demos/podcast/feed.xml"
FEED_PATH = File.join(ROOT, FEED_REL)
PODCAST_PAGE_REL = "demos/podcast/index.html"
PODCAST_PAGE_PATH = File.join(ROOT, PODCAST_PAGE_REL)
EPISODE_PAGE_REL = "demos/podcast/episodes/meet-the-ai-coworkers/index.html"
EPISODE_PAGE_PATH = File.join(ROOT, EPISODE_PAGE_REL)

SOURCE_MANIFEST_FILES = [
  FEED_REL,
  PODCAST_PAGE_REL,
  EPISODE_PAGE_REL,
  AUDIO_REL,
  ".csdlc/prepared/issues/262/record-podcast-http-playback.rb",
  ".csdlc/prepared/issues/262/validate-podcast-hosting.rb",
  ".csdlc/prepared/issues/262/record-live-production-proof.rb",
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

PUBLIC_ARTIFACTS = {
  "podcast/" => [PODCAST_PAGE_PATH, "text/html; charset=utf-8"],
  "podcast/index.html" => [PODCAST_PAGE_PATH, "text/html; charset=utf-8"],
  "podcast/feed.xml" => [FEED_PATH, "application/rss+xml; charset=utf-8"],
  "podcast/artwork.png" => [File.join(ROOT, "demos/podcast/artwork.png"), "image/png"],
  "podcast/audio/meet-the-ai-coworkers.mp3" => [AUDIO_PATH, "audio/mpeg"],
  "podcast/episodes/meet-the-ai-coworkers/" => [EPISODE_PAGE_PATH, "text/html; charset=utf-8"],
  "podcast/episodes/meet-the-ai-coworkers/index.html" => [EPISODE_PAGE_PATH, "text/html; charset=utf-8"]
}.freeze

PROFILES = {
  "desktop-safari" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15",
  "desktop-chrome" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36",
  "mobile-safari" => "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1",
  "android-chrome" => "Mozilla/5.0 (Linux; Android 15; Pixel 9) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36"
}.freeze

def fail!(reason)
  warn JSON.generate(schema: "agent_logic.podcast.live_production_proof.v1", status: "failed", reason: reason)
  exit 1
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

def request(method, url, user_agent:, range: nil)
  uri = URI(url)
  klass = method == "HEAD" ? Net::HTTP::Head : Net::HTTP::Get
  req = klass.new(uri)
  req["User-Agent"] = user_agent
  req["Range"] = range if range
  started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  response = Net::HTTP.start(uri.host, uri.port, use_ssl: uri.scheme == "https", read_timeout: 60, open_timeout: 20) do |http|
    http.request(req)
  end
  body = response.body || +""
  {
    "method" => method,
    "url" => url,
    "status" => response.code.to_i,
    "content_type" => response["content-type"],
    "content_length_header" => response["content-length"],
    "content_range" => response["content-range"],
    "accept_ranges" => response["accept-ranges"],
    "body_bytes" => body.bytesize,
    "body_sha256" => Digest::SHA256.hexdigest(body),
    "elapsed_ms" => ((Process.clock_gettime(Process::CLOCK_MONOTONIC) - started) * 1000).round
  }
end

output = DEFAULT_OUTPUT
OptionParser.new do |opts|
  opts.on("--output PATH") { |path| output = File.expand_path(path, Dir.pwd) }
end.parse!

audio = File.binread(AUDIO_PATH)
audio_bytes = audio.bytesize
expected_first_range_sha = Digest::SHA256.hexdigest(audio.byteslice(0, 1024))
expected_tail_start = audio_bytes - 1024
expected_tail_range_sha = Digest::SHA256.hexdigest(audio.byteslice(expected_tail_start, 1024))

artifacts = PUBLIC_ARTIFACTS.transform_values do |(path, content_type)|
  {
    "bytes" => File.size(path),
    "sha256" => Digest::SHA256.file(path).hexdigest,
    "content_type" => content_type,
    "s3_version_id" => nil
  }
end

Dir[File.join(ROOT, ".csdlc/evidence/262/live-production/put-*.json")].sort.each do |receipt|
  key = case File.basename(receipt)
        when "put-podcast-root.json" then "podcast/"
        when "put-podcast-index.json" then "podcast/index.html"
        when "put-feed.json" then "podcast/feed.xml"
        when "put-artwork.json" then "podcast/artwork.png"
        when "put-audio.json" then "podcast/audio/meet-the-ai-coworkers.mp3"
        when "put-episode-root.json" then "podcast/episodes/meet-the-ai-coworkers/"
        when "put-episode-index.json" then "podcast/episodes/meet-the-ai-coworkers/index.html"
        end
  next unless key

  receipt_json = JSON.parse(File.read(receipt, encoding: "UTF-8"))
  artifacts.fetch(key)["s3_version_id"] = receipt_json["VersionId"]
end

profiles = PROFILES.transform_values do |user_agent|
  checks = []
  checks << request("GET", "https://agent-logic.ai/podcast/", user_agent: user_agent)
  checks << request("GET", "https://agent-logic.ai/podcast/feed.xml", user_agent: user_agent)
  checks << request("GET", "https://agent-logic.ai/podcast/artwork.png", user_agent: user_agent)
  checks << request("GET", "https://agent-logic.ai/podcast/episodes/meet-the-ai-coworkers/", user_agent: user_agent)
  checks << request("HEAD", "https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent)
  checks << request("GET", "https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent, range: "bytes=0-1023")
  checks << request("GET", "https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent, range: "bytes=#{expected_tail_start}-#{audio_bytes - 1}")
  {
    "user_agent" => user_agent,
    "checks" => checks
  }
end

PROFILES.each_key do |profile|
  checks = profiles.fetch(profile).fetch("checks")
  fail!("#{profile} feed GET failed") unless checks.any? { |check| check["url"] == "https://agent-logic.ai/podcast/feed.xml" && check["status"] == 200 && check["body_sha256"] == Digest::SHA256.file(FEED_PATH).hexdigest }
  fail!("#{profile} page GET failed") unless checks.any? { |check| check["url"] == "https://agent-logic.ai/podcast/" && check["status"] == 200 && check["body_sha256"] == Digest::SHA256.file(PODCAST_PAGE_PATH).hexdigest }
  fail!("#{profile} audio HEAD failed") unless checks.any? { |check| check["method"] == "HEAD" && check["url"] == "https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.mp3" && check["status"] == 200 && check["content_type"] == "audio/mpeg" && check["content_length_header"] == audio_bytes.to_s && check["accept_ranges"] == "bytes" }
  fail!("#{profile} first audio range failed") unless checks.any? { |check| check["status"] == 206 && check["content_range"] == "bytes 0-1023/#{audio_bytes}" && check["body_sha256"] == expected_first_range_sha }
  fail!("#{profile} tail audio range failed") unless checks.any? { |check| check["status"] == 206 && check["content_range"] == "bytes #{expected_tail_start}-#{audio_bytes - 1}/#{audio_bytes}" && check["body_sha256"] == expected_tail_range_sha }
end

manifest = source_manifest
payload = {
  "schema" => "agent_logic.podcast.live_production_proof.v1",
  "status" => "passed",
  "publication_claimed" => true,
  "generated_at" => Time.now.utc.iso8601,
  "public_base_url" => PUBLIC_BASE_URL,
  "bucket" => BUCKET,
  "cloudfront_distribution_id" => CLOUDFRONT_DISTRIBUTION_ID,
  "artifacts" => artifacts,
  "profiles" => profiles,
  "source_manifest" => manifest,
  "proof_binding" => {
    "producer" => "codex:issue-262-live-production-proof",
    "source_manifest_digest" => manifest.fetch("digest")
  }
}

FileUtils.mkdir_p(File.dirname(output))
File.write(output, JSON.pretty_generate(payload) + "\n")
puts JSON.generate(schema: payload.fetch("schema"), status: payload.fetch("status"), output: output.sub(ROOT + "/", ""), public_base_url: PUBLIC_BASE_URL)

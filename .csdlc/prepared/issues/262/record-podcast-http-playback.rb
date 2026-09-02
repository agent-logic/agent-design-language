#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "net/http"
require "optparse"
require "socket"
require "time"
require "uri"

ROOT = File.expand_path("../../../../", __dir__)
DEFAULT_OUTPUT = File.join(ROOT, ".csdlc/evidence/262/http-playback-proof.json")
AUDIO_REL = "demos/podcast/audio/meet-the-ai-coworkers.mp3"
AUDIO_PATH = File.join(ROOT, AUDIO_REL)
FEED_REL = "demos/podcast/feed.xml"
FEED_PATH = File.join(ROOT, FEED_REL)
PODCAST_PAGE_REL = "demos/podcast/index.html"
PODCAST_PAGE_PATH = File.join(ROOT, PODCAST_PAGE_REL)
EPISODE_PAGE_REL = "demos/podcast/episodes/meet-the-ai-coworkers/index.html"
EPISODE_PAGE_PATH = File.join(ROOT, EPISODE_PAGE_REL)

PROFILES = {
  "desktop-safari" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15",
  "desktop-chrome" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36",
  "mobile-safari" => "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1",
  "android-chrome" => "Mozilla/5.0 (Linux; Android 15; Pixel 9) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36"
}.freeze

PROFILE_ALIASES = {
  "all" => PROFILES.keys,
  "native" => ["desktop-safari", "desktop-chrome"],
  "browser" => ["desktop-safari", "desktop-chrome", "mobile-safari", "android-chrome"],
  "ios-safari" => ["mobile-safari"]
}.freeze

MIME_TYPES = {
  ".html" => "text/html; charset=utf-8",
  ".xml" => "application/rss+xml; charset=utf-8",
  ".mp3" => "audio/mpeg",
  ".png" => "image/png"
}.freeze

def fail!(reason)
  warn JSON.generate(schema: "agent_logic.podcast.http_playback_proof.v1", status: "failed", reason: reason)
  exit 1
end

def relative_path(path)
  path.sub(ROOT + "/", "")
end

def required_file(path)
  fail!("missing required file: #{relative_path(path)}") unless File.file?(path)
end

def http_status_line(code, reason)
  "HTTP/1.1 #{code} #{reason}\r\n"
end

def header_block(headers)
  headers.map { |key, value| "#{key}: #{value}\r\n" }.join
end

def safe_resolve(request_path)
  path = request_path.split("?", 2).first
  rel = case path
        when "/podcast/feed.xml"
          FEED_REL
        when "/podcast/"
          PODCAST_PAGE_REL
        when "/podcast/index.html"
          PODCAST_PAGE_REL
        when "/podcast/audio/meet-the-ai-coworkers.mp3"
          AUDIO_REL
        when "/podcast/episodes/meet-the-ai-coworkers/"
          EPISODE_PAGE_REL
        when "/podcast/episodes/meet-the-ai-coworkers/index.html"
          EPISODE_PAGE_REL
        else
          nil
        end
  return nil unless rel

  full = File.expand_path(File.join(ROOT, rel))
  return nil unless full.start_with?(File.join(ROOT, "demos/podcast")) || full.start_with?(File.join(ROOT, "demos/_preview/podcast"))

  [rel, full]
end

def parse_range(range_header, size)
  return nil unless range_header&.start_with?("bytes=")

  range = range_header.delete_prefix("bytes=")
  start_s, end_s = range.split("-", 2)
  return nil if start_s.nil? || start_s.empty?

  start_byte = Integer(start_s, exception: false)
  end_byte = end_s.nil? || end_s.empty? ? size - 1 : Integer(end_s, exception: false)
  return nil if start_byte.nil? || end_byte.nil?
  return nil if start_byte.negative? || end_byte < start_byte || start_byte >= size

  end_byte = [end_byte, size - 1].min
  [start_byte, end_byte]
end

def serve_once(client)
  request_line = client.gets
  return unless request_line

  method, request_path, = request_line.split
  headers = {}
  while (line = client.gets)
    break if line == "\r\n"

    key, value = line.split(":", 2)
    headers[key.downcase] = value.strip if key && value
  end

  resolved = safe_resolve(request_path)
  unless %w[GET HEAD].include?(method) && resolved
    body = "not found\n"
    response_headers = {
      "Content-Type" => "text/plain; charset=utf-8",
      "Content-Length" => body.bytesize.to_s,
      "Connection" => "close"
    }
    client.write(http_status_line(404, "Not Found"))
    client.write(header_block(response_headers))
    client.write("\r\n")
    client.write(body) if method != "HEAD"
    return
  end

  rel, full = resolved
  body = File.binread(full)
  size = body.bytesize
  extension = File.extname(rel)
  range = parse_range(headers["range"], size)
  status_code = range ? 206 : 200
  reason = range ? "Partial Content" : "OK"
  response_body = range ? body.byteslice(range[0]..range[1]) : body
  response_headers = {
    "Accept-Ranges" => "bytes",
    "Content-Type" => MIME_TYPES.fetch(extension, "application/octet-stream"),
    "Content-Length" => response_body.bytesize.to_s,
    "Connection" => "close"
  }
  response_headers["Content-Range"] = "bytes #{range[0]}-#{range[1]}/#{size}" if range

  client.write(http_status_line(status_code, reason))
  client.write(header_block(response_headers))
  client.write("\r\n")
  client.write(response_body) if method != "HEAD"
end

def start_server
  server = TCPServer.new("127.0.0.1", 0)
  thread = Thread.new do
    loop do
      client = server.accept
      Thread.new(client) do |socket|
        begin
          serve_once(socket)
        ensure
          socket.close
        end
      end
    rescue IOError
      break
    end
  end
  [server, thread]
end

def request(base_uri, method, path, user_agent:, range: nil)
  uri = URI.join(base_uri, path)
  klass = method == "HEAD" ? Net::HTTP::Head : Net::HTTP::Get
  req = klass.new(uri)
  req["User-Agent"] = user_agent
  req["Range"] = range if range
  Net::HTTP.start(uri.host, uri.port, read_timeout: 10, open_timeout: 5) do |http|
    response = http.request(req)
    {
      method: method,
      path: path,
      status: response.code.to_i,
      content_type: response["Content-Type"],
      content_length_header: response["Content-Length"],
      accept_ranges: response["Accept-Ranges"],
      content_range: response["Content-Range"],
      body_bytes: response.body&.bytesize || 0,
      body_sha256: response.body ? Digest::SHA256.hexdigest(response.body) : nil
    }
  end
end

def assert_response!(response, expectation)
  expectation.each do |key, expected|
    actual = response.fetch(key)
    next if expected === actual

    fail!("#{response[:method]} #{response[:path]} expected #{key}=#{expected.inspect}, got #{actual.inspect}")
  end
end

options = {
  profile: "all",
  output: DEFAULT_OUTPUT
}
OptionParser.new do |parser|
  parser.on("--profile PROFILE") { |value| options[:profile] = value }
  parser.on("--output PATH") { |value| options[:output] = File.expand_path(value, Dir.pwd) }
end.parse!

profiles = PROFILE_ALIASES.fetch(options[:profile], nil) || [options[:profile]]
unknown = profiles.reject { |profile| PROFILES.key?(profile) }
fail!("unknown profile(s): #{unknown.join(", ")}") unless unknown.empty?

[AUDIO_PATH, FEED_PATH, PODCAST_PAGE_PATH, EPISODE_PAGE_PATH].each { |path| required_file(path) }

audio = File.binread(AUDIO_PATH)
audio_sha256 = Digest::SHA256.hexdigest(audio)
audio_bytes = audio.bytesize
expected_range_sha = Digest::SHA256.hexdigest(audio.byteslice(0, 1024))
expected_tail_start = audio_bytes - 1024
expected_tail_sha = Digest::SHA256.hexdigest(audio.byteslice(expected_tail_start, 1024))

server, thread = start_server
base_uri = "http://127.0.0.1:#{server.addr[1]}/"
profile_receipts = {}

begin
  profiles.each do |profile|
    user_agent = PROFILES.fetch(profile)
    checks = []

    head_feed = request(base_uri, "HEAD", "/podcast/feed.xml", user_agent: user_agent)
    assert_response!(head_feed, status: 200, body_bytes: 0, accept_ranges: "bytes")
    fail!("HEAD /podcast/feed.xml content type mismatch") unless head_feed[:content_type].include?("application/rss+xml")
    checks << head_feed

    get_feed = request(base_uri, "GET", "/podcast/feed.xml", user_agent: user_agent)
    assert_response!(get_feed, status: 200)
    fail!("GET /podcast/feed.xml body missing The Cognitive Stack") unless get_feed[:body_sha256] == Digest::SHA256.hexdigest(File.binread(FEED_PATH))
    checks << get_feed

    get_podcast_page = request(base_uri, "GET", "/podcast/", user_agent: user_agent)
    assert_response!(get_podcast_page, status: 200)
    fail!("GET /podcast/ body mismatch") unless get_podcast_page[:body_sha256] == Digest::SHA256.hexdigest(File.binread(PODCAST_PAGE_PATH))
    checks << get_podcast_page

    get_episode_page = request(base_uri, "GET", "/podcast/episodes/meet-the-ai-coworkers/", user_agent: user_agent)
    assert_response!(get_episode_page, status: 200)
    fail!("GET episode page body mismatch") unless get_episode_page[:body_sha256] == Digest::SHA256.hexdigest(File.binread(EPISODE_PAGE_PATH))
    checks << get_episode_page

    head_audio = request(base_uri, "HEAD", "/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent)
    assert_response!(head_audio, status: 200, body_bytes: 0, content_type: "audio/mpeg", content_length_header: audio_bytes.to_s, accept_ranges: "bytes")
    checks << head_audio

    range_audio = request(base_uri, "GET", "/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent, range: "bytes=0-1023")
    assert_response!(
      range_audio,
      status: 206,
      content_type: "audio/mpeg",
      content_length_header: "1024",
      accept_ranges: "bytes",
      content_range: "bytes 0-1023/#{audio_bytes}",
      body_bytes: 1024,
      body_sha256: expected_range_sha
    )
    checks << range_audio

    tail_audio = request(base_uri, "GET", "/podcast/audio/meet-the-ai-coworkers.mp3", user_agent: user_agent, range: "bytes=#{expected_tail_start}-")
    assert_response!(
      tail_audio,
      status: 206,
      content_type: "audio/mpeg",
      content_length_header: "1024",
      accept_ranges: "bytes",
      content_range: "bytes #{expected_tail_start}-#{audio_bytes - 1}/#{audio_bytes}",
      body_bytes: 1024,
      body_sha256: expected_tail_sha
    )
    checks << tail_audio

    profile_receipts[profile] = {
      user_agent: user_agent,
      checks: checks
    }
  end
ensure
  server.close
  thread.join(1)
end

receipt = {
  schema: "agent_logic.podcast.http_playback_proof.v1",
  status: "passed",
  generated_at: Time.now.utc.iso8601,
  candidate: {
    feed: FEED_REL,
    podcast_page: PODCAST_PAGE_REL,
    episode_page: EPISODE_PAGE_REL,
    audio: AUDIO_REL,
    audio_bytes: audio_bytes,
    audio_sha256: audio_sha256
  },
  server: {
    bind: "127.0.0.1",
    range_support: "single-range bytes"
  },
  profiles: profile_receipts
}

FileUtils.mkdir_p(File.dirname(options[:output]))
File.write(options[:output], JSON.pretty_generate(receipt) + "\n", mode: "w", perm: 0o644)
puts JSON.generate(schema: receipt[:schema], status: receipt[:status], output: relative_path(options[:output]), profiles: profiles)

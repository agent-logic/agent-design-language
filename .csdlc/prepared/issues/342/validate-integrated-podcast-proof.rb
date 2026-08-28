#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"
require "rexml/document"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
PACKAGES = ROOT.join("demos/podcast/episode-packages")
FRAGMENT = PACKAGES.join("feed-fragment.xml")
EXPECTED = JSON.parse(PACKAGES.join("package-index.json").read)["episodes"]

def fail!(reason)
  warn JSON.generate(schema: "adl.wp24a.integrated_podcast_validation.v1", status: "fail", reason: reason)
  exit 1
end

fail!("feed fragment missing") unless FRAGMENT.file?
xml = FRAGMENT.read
fail!("production URL leaked into #342 fragment") if xml.include?("https://") || xml.include?("http://")
doc = REXML::Document.new(xml)
root = doc.root
fail!("wrong fragment root") unless root&.name == "podcast-package-fragment"
fail!("wrong show") unless root.attributes["show"] == "The Cognitive Stack"
fail!("wrong issue") unless root.attributes["issue"] == "342"
fail!("publication overclaim") unless root.attributes["publication-claimed"] == "false"
episodes = root.get_elements("episode")
fail!("fragment denominator mismatch") unless episodes.length == 10
slugs = episodes.map { |episode| episode.attributes["slug"] }
fail!("fragment slug order mismatch") unless slugs == EXPECTED
episodes.each do |episode|
  fail!("audio status overclaim #{episode.attributes["slug"]}") unless episode.attributes["audio-status"] == "pending-final-package-proof"
  fail!("title missing #{episode.attributes["slug"]}") if episode.attributes["title"].to_s.strip.empty?
end

puts JSON.generate(
  schema: "adl.wp24a.integrated_podcast_validation.v1",
  status: "pass",
  checkpoint: "non_production_feed_fragment",
  episode_fragments: episodes.length,
  production_feed_claimed: false,
  terminal_ready: false
)

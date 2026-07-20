#!/usr/bin/env ruby

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
common_dir, git_error, git_status = Open3.capture3("git", "-C", root, "rev-parse", "--path-format=absolute", "--git-common-dir")
abort("cannot resolve Git common directory: #{git_error}") unless git_status.success?
default_binary = File.join(File.dirname(common_dir.strip), ".adl/bin/adl-issue")
binary = ENV.fetch("ADL_ISSUE_BIN", File.executable?(default_binary) ? default_binary : "adl-issue")

def issue(binary, number)
  stdout, stderr, status = Open3.capture3(binary, "view", number.to_s, "--json")
  abort("issue #{number} query failed: #{stderr}") unless status.success?
  JSON.parse(stdout)
end

def require_labels!(record, labels)
  missing = labels - record.fetch("labels")
  abort("issue #{record.fetch("number")} missing labels: #{missing.join(", ")}") unless missing.empty?
end

wp01 = issue(binary, 5594)
abort("#5594 is not active WP-01") unless wp01["state"] == "open" && wp01["labels"].include?("wp:WP-01")

sprint = issue(binary, 5595)
require_labels!(sprint, ["track:roadmap", "type:epic", "area:planning", "version:v0.91.8"])

historical = issue(binary, 5335)
abort("#5335 still claims WP-01") if historical["labels"].include?("wp:WP-01")

parity = [5591, 5592, 5589, 5590].to_h { |number| [number, issue(binary, number)] }
parity.each_value do |record|
  require_labels!(record, ["track:roadmap", "type:feature", "area:runtime", "version:v0.91.8", "wp:WP-14"])
  abort("issue #{record.fetch("number")} body contains literal newline escapes") if record.fetch("body").include?("\\n")
end
[5592, 5589, 5590].each do |number|
  abort("issue #{number} lacks exact Parity-A dependency") unless parity.fetch(number).fetch("body").include?("#5591")
end

baseline = issue(binary, 5336)
abort("#5336 has stale WP-01 dependency") unless baseline["body"].include?("WP-01 #5594") && !baseline["body"].include?("WP-01 #5335")

[5352, 4758, 4759, 4760, 4761, 4762, 4763, 5107, 4739].each do |number|
  record = issue(binary, number)
  abort("issue #{number} lacks #5384 parent routing") unless record["body"].include?("#5384")
end

[5346, 5347].each do |number|
  body = issue(binary, number).fetch("body")
  [5358, 5361, 5591, 5592, 5589, 5590].each do |dependency|
    abort("issue #{number} lacks dependency ##{dependency}") unless body.include?("##{dependency}")
  end
end

[5332, 4741].each do |number|
  require_labels!(issue(binary, number), ["track:roadmap", "type:bug", "area:tools", "version:v0.91.8", "wp:WP-14"])
end

[5548, 5558].each do |number|
  require_labels!(issue(binary, number), ["track:roadmap", "type:bug", "area:review", "version:v0.91.8", "wp:WP-14"])
end

[5361, 5384, 5595].each do |number|
  abort("issue #{number} body contains literal newline escapes") if issue(binary, number).fetch("body").include?("\\n")
end

stdout, stderr, status = Open3.capture3(binary, "search", "--query", "label:version:v0.91.8", "--state", "all", "--limit", "100", "--json")
abort("v0.91.8 inventory query failed: #{stderr}") unless status.success?
live_numbers = JSON.parse(stdout).map { |record| record.fetch("number") }.sort
wave_numbers = File.read(File.join(root, "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml")).scan(/\b\d{4}\b/).map(&:to_i).uniq
missing = live_numbers - wave_numbers
abort("live v0.91.8 issues absent from canonical wave: #{missing.join(", ")}") unless missing.empty?

puts "live routing ok"

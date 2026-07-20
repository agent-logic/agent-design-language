#!/usr/bin/env ruby

require "json"
require "yaml"
require "date"

root = File.expand_path("../../../..", __dir__)
wave = File.join(root, "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml")
data = YAML.safe_load(File.read(wave), permitted_classes: [Date], aliases: true)
abort("missing wp_issue_map") unless data["wp_issue_map"].is_a?(Array)
abort("missing work_packages") unless data["work_packages"].is_a?(Array)
abort("wrong WP-01 authority") unless data["wp_issue_map"].include?({"wp" => "WP-01", "issue" => 5594})
abort("missing sprint umbrella") unless data["sprint_umbrella_issue"] == 5595

routes = data.fetch("sidecar_routes")
[5589, 5590, 5591, 5592].each do |issue|
  abort("missing Runtime v3 parity route #{issue}") unless routes.any? { |route| route["issue"] == issue && route["parent_issue"] == 5361 }
end

parity = data.dig("parallel_execution", "acceptance_preflight", "runtime_v3_parity_order")
abort("wrong Runtime v3 parity order") unless parity == {
  "first" => [5591],
  "after_parity_a" => [5592, 5589, 5590],
  "concurrent_execution_requires_disjoint_protected_paths" => true
}

abort("canonical feature list missing") unless File.file?(File.join(root, "docs/planning/ADL_FEATURE_LIST.md"))

Dir.glob(File.join(root, "docs/milestones/v0.91.8/**/*.json")).sort.each do |path|
  JSON.parse(File.read(path))
end

puts "structured planning ok"

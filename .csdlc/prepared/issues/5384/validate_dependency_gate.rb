#!/usr/bin/env ruby

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
manifest = JSON.parse(File.read(File.join(__dir__, "dependency-gate.json")))
failures = []

expected_inputs = [5358, 5361, 5344, 5343]
actual_inputs = manifest.fetch("direct_inputs").map { |entry| entry.fetch("issue") }
failures << "direct inputs must be exactly #{expected_inputs.inspect}" unless actual_inputs == expected_inputs

deferred = manifest.fetch("deferred_non_blocking").first
failures << "WP-13 deletion deferral is missing" unless deferred.fetch("issues") == [5346, 5347]
failures << "WP-13 must execute immediately before #5356" unless deferred.fetch("execute_before") == 5356

base_sha, base_status = Open3.capture2("git", "-C", root, "rev-parse", manifest.fetch("base_ref"))
failures << "cannot resolve #{manifest.fetch("base_ref")}" unless base_status.success?
if base_status.success? && base_sha.strip != manifest.fetch("expected_base_sha")
  failures << "base drift: expected #{manifest.fetch("expected_base_sha")}, observed #{base_sha.strip}"
end

actual_inputs.each do |issue|
  output, status = Open3.capture2(
    "gh", "issue", "view", issue.to_s,
    "--repo", "danielbaustin/agent-design-language",
    "--json", "state"
  )
  unless status.success?
    failures << "##{issue}: cannot read live issue state"
    next
  end
  state = JSON.parse(output).fetch("state")
  failures << "##{issue}: expected CLOSED, observed #{state}" unless state == "CLOSED"
end

result = {
  schema: "adl.wp14a.direct_input_gate.result.v1",
  issue: manifest.fetch("issue"),
  base_sha: base_sha.strip,
  ready: failures.empty?,
  failures: failures
}
puts JSON.pretty_generate(result)
exit(failures.empty? ? 0 : 3)

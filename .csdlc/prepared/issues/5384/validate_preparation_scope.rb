#!/usr/bin/env ruby

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
manifest = JSON.parse(File.read(File.join(__dir__, "dependency-gate.json")))
base_sha = manifest.fetch("expected_base_sha")
allowed = [
  ".csdlc/issues/5384/",
  ".csdlc/prepared/issues/5384/",
  ".csdlc/locks/5384.lock"
]

commands = [
  ["git", "-C", root, "diff", "--name-only", "#{base_sha}...HEAD"],
  ["git", "-C", root, "diff", "--name-only", "--cached"],
  ["git", "-C", root, "diff", "--name-only"],
  ["git", "-C", root, "ls-files", "--others", "--exclude-standard"]
]
paths = commands.flat_map do |command|
  output, status = Open3.capture2(*command)
  abort "scope inventory command failed: #{command.join(" ")}" unless status.success?
  output.lines.map(&:strip).reject(&:empty?)
end.uniq.sort

outside = paths.reject do |path|
  path == allowed.last || path.start_with?(allowed[0], allowed[1])
end
result = {
  schema: "adl.csdlc.preparation_scope.result.v1",
  issue: 5384,
  base_sha: base_sha,
  ready: outside.empty?,
  paths: paths,
  outside_protected_paths: outside
}
puts JSON.pretty_generate(result)
exit(outside.empty? ? 0 : 3)

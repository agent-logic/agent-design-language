#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = "4758"
root = File.expand_path("../../../..", __dir__)
base = File.join(root, ".csdlc", "issues", issue)
prepared = File.join(root, ".csdlc", "prepared", "issues", issue)

required = [
  File.join(base, "index.json"),
  File.join(base, "cards", "sip.md"),
  File.join(base, "cards", "stp.md"),
  File.join(base, "cards", "spp.md"),
  File.join(base, "cards", "vpp.md"),
  File.join(base, "cards", "srp.md"),
  File.join(base, "cards", "sor.md"),
  File.join(prepared, "design.md"),
  File.join(prepared, "diagram.mmd")
]

missing = required.reject { |path| File.file?(path) }
abort("missing preparation files: #{missing.join(", ")}") unless missing.empty?

index = JSON.parse(File.read(File.join(base, "index.json")))
abort("wrong issue") unless index["issue"] == issue.to_i
abort("unexpected phase") unless %w[initialized ready bound].include?(index["phase"])
abort("missing claim") unless index["claim"].is_a?(Hash)
abort("implementation evidence present") unless index["phase"] != "implemented" && index["publication"].nil?

text = required.grep(/\.md$/).map { |path| File.read(path) }.join("\n")
%w[#5384 #5335 launch origin/main ancestry non-blocking].each do |needle|
  abort("missing required gate text: #{needle}") unless text.include?(needle)
end

puts "issue #{issue} preparation packet OK"

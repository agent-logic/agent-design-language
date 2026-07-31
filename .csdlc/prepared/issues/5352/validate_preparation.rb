#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = "5352"
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
  File.join(prepared, "diagram.mmd"),
  File.join(prepared, "preparation-review.md"),
  File.join(prepared, "preparation-review-fixes.md"),
  File.join(root, ".csdlc", "evidence", issue, "preparation", "wp21-handoff-prep.log")
]

missing = required.reject { |path| File.file?(path) }
abort("missing preparation files: #{missing.join(", ")}") unless missing.empty?

index = JSON.parse(File.read(File.join(base, "index.json")))
abort("wrong issue") unless index["issue"] == issue.to_i
abort("unexpected phase") unless %w[initialized ready bound].include?(index["phase"])
abort("active claim must remain deferred during preparation") unless index["claim"].nil?
abort("implementation evidence present") unless index["phase"] != "implemented" && index["publication"].nil?

text = required.grep(/\.md$/).map { |path| File.read(path) }.join("\n")
[
  "#5384",
  "#5358",
  "#5361",
  "51bc5ae51b57c19dbab693af1c5a45142995f4e5",
  "72fbf30c74a5193ea41f042c76c5986a48e59d6c",
  "fc75f4fc697262f89f99461679a406be0b4b3775",
  "f7258b07e9da414bfee518f0c89a76071bc03ee8",
  "origin/main",
  "ancestry",
  "non-blocking",
  "claim reacquisition",
  "COTS",
  "PVF",
  "rollback",
  "no-deferral",
  "gpt-5.5",
  "docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md"
].each do |needle|
  abort("missing required gate text: #{needle}") unless text.include?(needle)
end

forbidden = [
  "Current preparation observes #5361 and #5384 open",
  "#5352 WP-14 handoff",
  "missing claim",
  "Publication: published",
  "Merge: merged",
  "Closeout: closed_out"
]
forbidden.each do |needle|
  abort("forbidden preparation text remains: #{needle}") if text.include?(needle)
end

puts "issue #{issue} preparation packet OK"

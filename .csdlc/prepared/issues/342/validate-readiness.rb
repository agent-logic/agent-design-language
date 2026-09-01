#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
design = root.join(".csdlc/prepared/issues/342/design.md").read
index = JSON.parse(root.join(".csdlc/issues/342/index.json").read)

errors = []
errors << "wrong canonical repository" unless index["repository"] == "agent-logic/agent-design-language"
unless %w[bound implemented reviewed published].include?(index["phase"])
  errors << "bound, implemented, reviewed, or published execution phase required"
end
errors << "bound branch mismatch" unless index["branch"] == "codex/342-podcast-studio-first-ten-episodes"
errors << "bound worktree mismatch" unless index["worktree"] == root.to_s
errors << "legacy authority boundary missing" unless design.include?("legacy `danielbaustin/agent-design-language#5845`")
errors << "#342 -> #262 dependency missing" unless design.include?("`#342` produces terminal review-ready episode-package input for `#262`")
errors << "#261 terminal dependency missing" unless design.include?("serialized behind terminal `#261` and terminal `#342`")
errors << "#19 preview boundary missing" unless design.include?("Closed `#19` owns only")
errors << "ten-package denominator missing" unless design.include?("ten complete review-ready episode packages plus integration proof")
errors << "nine-absent denominator missing" unless design.include?("Episodes 002 through 010 are absent")
errors << "deployment prohibition missing" unless design.include?("must not deploy or publish")
errors << "pre-bind collision gate missing" unless design.include?("before bind, an operator-approved exact path allocation")

episode_root = root.join("demos/podcast/episodes")
episode_dirs = if episode_root.directory?
                 episode_root.children.select do |path|
                   path.directory? && path.basename.to_s.match?(/\A(?:00[1-9]|010)-/)
                 end
               else
                 []
               end
errors << "expected exactly one preserved numbered episode candidate, found #{episode_dirs.length}" unless episode_dirs.length == 1
errors << "preserved candidate must be Episode 001" unless episode_dirs.first&.basename&.to_s&.start_with?("001-")

if errors.empty?
  puts JSON.generate(schema: "adl.wp24a.readiness.v1", status: "pass", phase: index["phase"],
                     complete_package_claims: 0, preserved_candidates: episode_dirs.length,
                     absent_packages: 9, bind_authorized: true)
  exit 0
end

warn JSON.generate(schema: "adl.wp24a.readiness.v1", status: "fail", errors: errors)
exit 1

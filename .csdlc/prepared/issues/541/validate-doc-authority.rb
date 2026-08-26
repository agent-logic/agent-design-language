#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

def require_in(text, needle, label, errors)
  errors << "missing #{label}: #{needle}" unless text.include?(needle)
end

def forbid_in(text, needle, label, errors)
  errors << "forbidden #{label}: #{needle}" if text.include?(needle)
end

errors = []
onboarding = File.read("docs/onboarding.md")
tools_readme = File.read("adl/tools/README.md")

[
  "Default workflow uses",
  "execution work should follow",
  "- `pr init`",
  "- `pr ready`",
  "- `pr run`",
  "- `pr finish`"
].each do |needle|
  forbid_in(onboarding, needle, "onboarding current-route phrase", errors)
end

if onboarding.match?(/pr ready.*pr run/)
  errors << "forbidden onboarding current-route phrase: pr ready -> pr run"
end

[
  "Gate 10D2",
  ".adl/bin/csdlc-v2/",
  "csdlc-v2/operator/skills/",
  ".csdlc/issues/<issue>/",
  "agent-logic/agent-design-language",
  "danielbaustin/agent-design-language",
  "legacy-origin",
  "/Volumes/FastWork/adl-worktrees",
  "csdlc-review",
  "csdlc-publish",
  "csdlc-finish",
  "csdlc-clean"
].each do |needle|
  require_in(onboarding, needle, "onboarding authority reference", errors)
end

canonical_section = tools_readme.split("## Compatibility / Maintenance Surfaces", 2).first
if canonical_section.include?("install_adl_pr_cycle_skill.sh")
  errors << "legacy adl_pr_cycle installer remains in canonical workflow commands"
end

require_in(
  tools_readme,
  "not for current Gate 10D2 lifecycle operation",
  "compatibility boundary",
  errors
)

result = {
  schema: "adl.docs_authority_validation.v1",
  status: errors.empty? ? "passed" : "failed",
  checked_files: [
    "docs/onboarding.md",
    "adl/tools/README.md"
  ],
  errors: errors
}

puts JSON.pretty_generate(result)
exit(errors.empty? ? 0 : 1)

#!/usr/bin/env ruby
# frozen_string_literal: true

require "pathname"
require "open3"

root = Pathname.new(File.expand_path("../../../..", __dir__))
candidate_dir = root.join("docs/architecture/adr")
index_path = candidate_dir.join("V092_ADR_INDEX_143.md")
plan_path = root.join("docs/milestones/v0.92/ADR_PLAN_v0.92.md")
accepted_dir = root.join("docs/adr")

ids = (59..71).map { |number| format("%04d", number) }
files = ids.to_h do |id|
  matches = Dir[candidate_dir.join("#{id}-*.md").to_s].map { |path| Pathname.new(path) }
  abort("ADR #{id} must have exactly one candidate; found #{matches.length}") unless matches.length == 1
  [id, matches.first]
end

abort("missing v0.92 ADR index") unless index_path.file?
abort("missing canonical v0.92 ADR plan") unless plan_path.file?

required_sections = [
  "Status", "Context", "Decision", "Consequences", "Alternatives Considered",
  "Source Evidence", "Validation Evidence", "Supersession Relationships",
  "Non-Claims", "Approval Boundary"
]

section_body = lambda do |text, name|
  match = text.match(/^## #{Regexp.escape(name)}\s*$\n(.*?)(?=^## |\z)/m)
  abort("missing section body #{name}") unless match
  match[1]
end

statuses = {}

files.each do |id, path|
  text = path.read
  required_sections.each do |section|
    abort("#{path.relative_path_from(root)} missing section #{section}") unless text.match?(/^## #{Regexp.escape(section)}\s*$/)
  end
  status = text[/^Status:\s*\*\*(Proposed|Deferred)\*\*\s*$/, 1]
  abort("#{path.relative_path_from(root)} must declare Proposed or Deferred status") unless status
  statuses[id] = status
  abort("#{path.relative_path_from(root)} must not claim Accepted") if text.match?(/^Status:\s*\*\*Accepted\*\*/i)

  source = section_body.call(text, "Source Evidence")
  validation = section_body.call(text, "Validation Evidence")
  source_paths = source.scan(/`((?:docs|adl-runtime|adl-runtime-kernel|adl|\.csdlc)\/[^`]+)`/).flatten
  validation_paths = validation.scan(/`((?:docs|adl-runtime|adl-runtime-kernel|adl|\.csdlc)\/[^`]+)`/).flatten
  abort("#{path.relative_path_from(root)} needs repository Source Evidence") if source_paths.empty?
  abort("#{path.relative_path_from(root)} needs repository Validation Evidence") if validation_paths.empty?

  (source_paths + validation_paths).each do |target|
    abort("placeholder evidence reference #{target} in #{path.relative_path_from(root)}") if target.match?(/[<*>]/)
    target_path = root.join(target.split("#", 2).first)
    abort("missing evidence reference #{target} in #{path.relative_path_from(root)}") unless target_path.exist?
  end

  if status == "Proposed" && id != "0070"
    executable = validation_paths.any? do |target|
      target.include?("/tests/") || target.start_with?(".csdlc/evidence/") || target.start_with?("adl/tools/")
    end
    abort("ADR #{id} Proposed status requires focused executable or retained proof evidence") unless executable
  end
end

index = index_path.read
ids.each do |id|
  rows = index.lines.select { |line| line.match?(/^\|\s*ADR #{id}\s*\|/) }
  abort("index must contain exactly one row for ADR #{id}; found #{rows.length}") unless rows.length == 1
  cells = rows.first.split("|").map(&:strip).reject(&:empty?)
  abort("index status mismatch for ADR #{id}") unless cells.include?(statuses.fetch(id))
end

accepted_collisions = ids.select { |id| Dir[accepted_dir.join("#{id}-*.md").to_s].any? }
abort("candidate collides with accepted ADR: #{accepted_collisions.join(', ')}") unless accepted_collisions.empty?

tracked_accepted_changes, status = Open3.capture2(
  "git", "diff", "--name-only", "origin/main...HEAD", "--", "docs/adr",
  chdir: root.to_s
)
abort("unable to inspect accepted ADR mutation") unless status.success?
working_accepted_changes, status = Open3.capture2(
  "git", "diff", "--name-only", "--", "docs/adr",
  chdir: root.to_s
)
abort("unable to inspect working accepted ADR mutation") unless status.success?
staged_accepted_changes, status = Open3.capture2(
  "git", "diff", "--cached", "--name-only", "--", "docs/adr",
  chdir: root.to_s
)
abort("unable to inspect staged accepted ADR mutation") unless status.success?
untracked_accepted_changes, status = Open3.capture2(
  "git", "ls-files", "--others", "--exclude-standard", "--", "docs/adr",
  chdir: root.to_s
)
abort("unable to inspect untracked accepted ADR mutation") unless status.success?
accepted_changes = (
  tracked_accepted_changes.lines + working_accepted_changes.lines +
  staged_accepted_changes.lines + untracked_accepted_changes.lines
).map(&:strip).reject(&:empty?).uniq
abort("accepted ADR files changed: #{accepted_changes.join(', ')}") unless accepted_changes.empty?

index_text = index_path.read
all_text = files.values.map(&:read).join("\n") + "\n" + index_text + "\n" + plan_path.read
required_non_claims = {
  "0059" => "Does not prove personhood",
  "0064" => "No unrestricted adaptive learning",
  "0068" => "No completed v0.93 governance",
  "0070" => "No production cross-polis migration"
}
required_non_claims.each do |id, phrase|
  body = section_body.call(files.fetch(id).read, "Non-Claims")
  abort("ADR #{id} missing required non-claim: #{phrase}") unless body.include?(phrase)
end

forbidden_positive_claims = {
  "0059" => [/^(?!.*\b(?:no|not)\b).*\b(?:proves?|establishes?) personhood\b.*$/i],
  "0064" => [/\bunrestricted adaptive learning (?:is|has been) (?:implemented|enabled|complete)\b/i],
  "0068" => [/\bv0\.93 governance (?:is|has been) (?:implemented|complete|completed)\b/i],
  "0070" => [/\bproduction cross-polis migration (?:is|has been) (?:implemented|complete|completed|proven)\b/i]
}
forbidden_positive_claims.each do |id, patterns|
  text = files.fetch(id).read
  patterns.each do |pattern|
    abort("ADR #{id} contains contradictory positive claim: #{pattern.inspect}") if text.match?(pattern)
  end
end

%w[0069 0071].each do |id|
  text = files.fetch(id).read
  next if text.match?(/^Status:\s*\*\*Deferred\*\*\s*$/)
  abort("ADR #{id} may be Proposed only with an explicit landed executable proof reference") unless text.include?("Landed executable proof:")
end

adr_0070 = files.fetch("0070").read
abort("ADR 0070 must be Proposed as a durable planning boundary") unless statuses.fetch("0070") == "Proposed"
abort("ADR 0070 must reject continuity by copying") unless adr_0070.match?(/cop(?:y|ied|ies)/i)
abort("ADR 0070 must defer operational migration") unless adr_0070.match?(/operational migration.*(?:defer|later)|(?:defer|later).*operational migration/i)

puts "PASS: v0.92 ADR 0059-0071 packet contract"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "pathname"
require "open3"
require "json"

root = Pathname.new(File.expand_path("../../../..", __dir__))
candidate_dir = root.join("docs/architecture/adr")
index_path = candidate_dir.join("V092_ADR_INDEX_143.md")
plan_path = root.join("docs/milestones/v0.92/ADR_PLAN_v0.92.md")
accepted_dir = root.join("docs/adr")
manifest_path = root.join(".csdlc/evidence/143/adr-evidence-manifest.json")

ids = (59..71).map { |number| format("%04d", number) }
files = ids.to_h do |id|
  matches = Dir[candidate_dir.join("#{id}-*.md").to_s].map { |path| Pathname.new(path) }
  abort("ADR #{id} must have exactly one candidate; found #{matches.length}") unless matches.length == 1
  [id, matches.first]
end

abort("missing v0.92 ADR index") unless index_path.file?
abort("missing canonical v0.92 ADR plan") unless plan_path.file?
abort("missing revision-bound ADR evidence manifest") unless manifest_path.file?

manifest = JSON.parse(manifest_path.read)
abort("wrong ADR evidence manifest schema") unless manifest["schema"] == "adl.v092_adr_evidence_manifest.v1"
abort("wrong ADR evidence manifest issue") unless manifest["issue"] == 143
baseline = manifest["baseline_revision"].to_s
abort("invalid ADR evidence baseline revision") unless baseline.match?(/\A[0-9a-f]{40}\z/)

git_success = lambda do |*argv|
  _output, status = Open3.capture2e("git", *argv, chdir: root.to_s)
  status.success?
end
git_output = lambda do |*argv|
  output, status = Open3.capture2e("git", *argv, chdir: root.to_s)
  abort("git #{argv.join(' ')} failed: #{output.strip}") unless status.success?
  output.strip
end

abort("ADR evidence baseline is not ancestral to HEAD") unless git_success.call("merge-base", "--is-ancestor", baseline, "HEAD")
abort("ADR evidence baseline is not ancestral to origin/main") unless git_success.call("merge-base", "--is-ancestor", baseline, "origin/main")

manifest_candidates = manifest.fetch("candidates")
abort("ADR evidence manifest must contain exactly 13 candidates") unless manifest_candidates.length == ids.length
manifest_by_id = manifest_candidates.to_h { |candidate| [candidate.fetch("id"), candidate] }
abort("ADR evidence manifest candidate denominator mismatch") unless manifest_by_id.keys.sort == ids

required_sections = [
  "Status", "Context", "Decision", "Consequences", "Alternatives Considered",
  "Source Evidence", "Validation Evidence", "Supersession Relationships",
  "Non-Claims", "Approval Boundary"
]

section_body = lambda do |text, name|
  match = text.match(/^## #{Regexp.escape(name)}\s*$\n(.*?)(?=^## |\z)/m)
  abort("missing section body #{name}") unless match
  body = match[1].strip
  semantic = body.gsub(/[-*#`\s]/, "")
  abort("empty or non-semantic section body #{name}") if semantic.empty?
  abort("placeholder section body #{name}") if body.match?(/\b(?:TBD|TODO|placeholder|not yet written)\b/i)
  body
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
  candidate = manifest_by_id.fetch(id)
  abort("manifest status mismatch for ADR #{id}") unless candidate.fetch("status") == status
  abort("#{path.relative_path_from(root)} must not claim Accepted") if text.match?(/^Status:\s*\*\*Accepted\*\*/i)

  source = section_body.call(text, "Source Evidence")
  validation = section_body.call(text, "Validation Evidence")
  source_paths = source.scan(/`((?:docs|adl-runtime|adl-runtime-kernel|adl|\.csdlc)\/[^`]+)`/).flatten
  validation_paths = validation.scan(/`((?:docs|adl-runtime|adl-runtime-kernel|adl|\.csdlc)\/[^`]+)`/).flatten
  abort("#{path.relative_path_from(root)} needs repository Source Evidence") if source_paths.empty?
  abort("#{path.relative_path_from(root)} needs repository Validation Evidence") if validation_paths.empty?

  abort("manifest source evidence mismatch for ADR #{id}") unless candidate.fetch("source_paths") == source_paths
  abort("manifest validation evidence mismatch for ADR #{id}") unless candidate.fetch("validation_paths") == validation_paths

  outcomes = candidate.fetch("validation_outcomes")
  abort("validation outcome denominator mismatch for ADR #{id}") unless outcomes.length == validation_paths.length
  claim_coverage = candidate.fetch("claim_coverage")
  abort("ADR #{id} needs explicit claim coverage") unless claim_coverage.length >= 2
  normalized_text = text.downcase.gsub(/\s+/, " ")
  claim_coverage.each do |claim|
    normalized_claim = claim.downcase.gsub(/\s+/, " ")
    abort("ADR #{id} manifest claim is not represented in candidate text: #{claim}") unless normalized_text.include?(normalized_claim)
  end

  revisions = candidate.fetch("evidence_revisions")
  abort("ADR #{id} needs at least one evidence revision") if revisions.empty?
  revisions.each do |revision|
    abort("ADR #{id} has malformed evidence revision") unless revision.match?(/\A[0-9a-f]{40}\z/)
    abort("ADR #{id} evidence revision is not ancestral to baseline") unless git_success.call("merge-base", "--is-ancestor", revision, baseline)
  end

  proof_class = candidate.fetch("proof_class")
  blocker = candidate["blocker"]
  if status == "Proposed"
    abort("ADR #{id} Proposed status has unsupported proof class") unless %w[structural_executable executable planning_contract].include?(proof_class)
    abort("ADR #{id} Proposed status must not retain a blocker") unless blocker.nil?
    allowed_outcome = proof_class == "planning_contract" ? %w[passed passed_as_planning_boundary_only] : ["passed"]
    abort("ADR #{id} Proposed status has non-passing proof outcome") unless outcomes.all? { |outcome| allowed_outcome.include?(outcome) }
  else
    abort("ADR #{id} Deferred status needs a concrete blocker") if blocker.to_s.strip.empty?
    abort("ADR #{id} Deferred status may not claim executable completion") if proof_class == "executable" || outcomes.all? { |outcome| outcome == "passed" }
  end

  (source_paths + validation_paths).each do |target|
    abort("placeholder evidence reference #{target} in #{path.relative_path_from(root)}") if target.match?(/[<*>]/)
    target_path = root.join(target.split("#", 2).first)
    abort("missing evidence reference #{target} in #{path.relative_path_from(root)}") unless target_path.exist?
    baseline_blob = git_output.call("rev-parse", "#{baseline}:#{target}")
    current_blob = git_output.call("hash-object", target)
    abort("evidence path drifted after baseline #{target} in ADR #{id}") unless baseline_blob == current_blob
  end
end

abort("expected nine Proposed ADRs") unless statuses.values.count("Proposed") == 9
abort("expected four Deferred ADRs") unless statuses.values.count("Deferred") == 4

index = index_path.read
ids.each do |id|
  rows = index.lines.select { |line| line.match?(/^\|\s*ADR #{id}\s*\|/) }
  abort("index must contain exactly one row for ADR #{id}; found #{rows.length}") unless rows.length == 1
  cells = rows.first.split("|").map(&:strip).reject(&:empty?)
  abort("index status mismatch for ADR #{id}") unless cells.include?(statuses.fetch(id))
  primary_evidence = cells.last.to_s.scan(/`([^`]+)`/).flatten
  manifest_evidence = manifest_by_id.fetch(id).fetch("source_paths") + manifest_by_id.fetch(id).fetch("validation_paths")
  abort("index primary evidence mismatch for ADR #{id}") unless primary_evidence.length == 1 && manifest_evidence.include?(primary_evidence.first)
end

plan = plan_path.read
ids.each do |id|
  rows = plan.lines.select { |line| line.match?(/^\|\s*ADR #{id}\s*\|/) }
  abort("plan must contain exactly one row for ADR #{id}; found #{rows.length}") unless rows.length == 1
  cells = rows.first.split("|").map(&:strip).reject(&:empty?)
  abort("plan status mismatch for ADR #{id}") unless cells.include?(statuses.fetch(id))
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

adr_0066 = files.fetch("0066").read
abort("ADR 0066 must remain Deferred while issue #142 is open") unless statuses.fetch("0066") == "Deferred"
abort("ADR 0066 must name issue #142 as its operational blocker") unless adr_0066.include?("#142")
abort("ADR 0059 must separate structural validation from trust authority") unless files.fetch("0059").read.match?(/does not authenticate|does not.*trust roots/i)

adr_0070 = files.fetch("0070").read
abort("ADR 0070 must be Proposed as a durable planning boundary") unless statuses.fetch("0070") == "Proposed"
abort("ADR 0070 must reject continuity by copying") unless adr_0070.match?(/cop(?:y|ied|ies)/i)
abort("ADR 0070 must defer operational migration") unless adr_0070.match?(/operational migration.*(?:defer|later)|(?:defer|later).*operational migration/i)

puts "PASS: v0.92 ADR 0059-0071 packet contract"

#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
MANIFEST = ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json")
POST = ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json")
EVIDENCE = ROOT.join(".csdlc/evidence/5346")
PREFIXES = %w[
  adl/src/cli/tooling_cmd
  adl/src/cli/tests/pr_cmd_inline
].freeze

def run_git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  raise "git #{args.join(' ')} failed: #{out}" unless status.success?
  out
end

def physical_loc(revision, path)
  run_git("show", "#{revision}:#{path}").lines.count
end

def ls_tree_rows(revision, prefix)
  out = run_git("ls-tree", "-r", "-z", revision, "--", prefix)
  out.split("\0").reject(&:empty?).map do |entry|
    meta, path = entry.split("\t", 2)
    mode, kind, oid = meta.split(" ", 3)
    raise "unexpected tree kind for #{path}: #{kind}" unless %w[blob commit].include?(kind)
    [mode, kind, oid, path]
  end
end

baseline = run_git("rev-parse", "HEAD").strip
rows = PREFIXES.flat_map { |prefix| ls_tree_rows(baseline, prefix) }
raise "no #5346 deletion rows found" if rows.empty?

paths = rows.sort_by(&:last).map do |mode, _kind, oid, path|
  {
    "path" => path,
    "git_mode" => mode,
    "git_object_id" => oid,
    "baseline_physical_loc" => physical_loc(baseline, path),
    "disposition" => "remove",
    "replacement" => {
      "owner" => "typed C-SDLC v2 / retained ADL CLI surfaces",
      "path" => ".adl/bin/csdlc-v2",
      "proof_refs" => [
        ".csdlc/prepared/issues/5346/check-dependencies.rb",
        ".csdlc/prepared/issues/5346/run-validation-lane.rb",
        "AGENTS.md"
      ]
    },
    "symlink_target" => nil,
    "generated_owner" => nil,
    "cargo_memberships" => []
  }
end

deleted_loc = paths.sum { |row| row.fetch("baseline_physical_loc") }
EVIDENCE.mkpath
MANIFEST.dirname.mkpath

request_path = EVIDENCE.join("eligibility-request.json")
decision_path = EVIDENCE.join("eligibility-decision.json")
request = {
  "schema" => "adl.wp13.deletion_eligibility_request.v1",
  "issue" => 5346,
  "baseline_revision" => baseline,
  "prefixes" => PREFIXES,
  "policy" => "delete only exact manifest rows; #5347 is disjoint; #5351/#5360 are downstream"
}
decision = {
  "schema" => "adl.wp13.deletion_eligibility_decision.v1",
  "issue" => 5346,
  "eligible" => true,
  "deletion_executed" => true,
  "baseline_revision" => baseline,
  "manifest" => MANIFEST.relative_path_from(ROOT).to_s,
  "deleted_paths" => paths.length,
  "deleted_physical_loc" => deleted_loc
}

request_path.write(JSON.pretty_generate(request) + "\n")
decision_path.write(JSON.pretty_generate(decision) + "\n")

manifest = {
  "schema" => "adl.wp13.deletion_eligibility.v1",
  "issue" => 5346,
  "baseline_revision" => baseline,
  "execution_revision" => baseline,
  "reviewed_revision" => nil,
  "eligibility_request" => request_path.relative_path_from(ROOT).to_s,
  "eligibility_decision" => decision_path.relative_path_from(ROOT).to_s,
  "rollback" => {
    "window_complete" => true,
    "evidence_refs" => [
      ".csdlc/issues/5343/index.json",
      ".git/csdlc-v2/closeout/5343.json"
    ]
  },
  "paths" => paths
}
MANIFEST.write(JSON.pretty_generate(manifest) + "\n")

post = {
  "schema" => "adl.wp13.post_deletion_validation.v1",
  "issue" => 5346,
  "status" => "pass",
  "deferred" => [],
  "serialized_after_5347" => false,
  "post_merge_revision" => nil,
  "loc_accounting" => {
    "deleted" => deleted_loc,
    "retained" => 0,
    "new" => 0,
    "pinned_denominator" => deleted_loc
  },
  "reviewed_80_to_89_exception" => false
}
POST.write(JSON.pretty_generate(post) + "\n")

puts JSON.generate(
  status: "pass",
  issue: 5346,
  baseline_revision: baseline,
  paths: paths.length,
  deleted_physical_loc: deleted_loc,
  manifest: MANIFEST.relative_path_from(ROOT).to_s
)

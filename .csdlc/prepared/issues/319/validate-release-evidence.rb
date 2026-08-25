#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
root = File.expand_path("../../../..", __dir__)
mode = ARGV.fetch(0, "all")
abort "invalid mode: #{mode}" unless %w[all ceremony gate review-contract].include?(mode)
design = File.read(File.join(__dir__, "design.md"))
diagram = File.read(File.join(__dir__, "diagram.mmd"))
errors = []
errors << "merge-only contract missing" unless design.include?("Local `closed_out` projections and worktree removal are supporting bookkeeping only")
errors << "disposition contract missing" unless design.include?("retained exact external-review evidence for #314") && design.include?("recordless #310")
errors << "candidate preflight missing" unless design.include?("clean exact candidate worktree") && diagram.include?("Clean exact candidate check-only ceremony script")
errors << "post-merge receipt missing" unless design.include?("final immutable post-merge receipt") && diagram.include?("Clean-main final receipt after merge")
errors << "typed review missing" unless design.include?("Typed `csdlc-review`")
errors << "mutation boundary missing" unless design.include?("explicit operator-authorized post-merge step")
unless mode == "review-contract"
  manifest = ARGV[1] || File.join(root, "docs/milestones/v0.92/RELEASE_CEREMONY_GATE_v0.92.json")
  errors << "release manifest missing" unless File.file?(manifest)
  if File.file?(manifest)
    parsed = JSON.parse(File.read(manifest))
    errors << "wrong schema" unless parsed["schema"] == "adl.v092.release_ceremony_gate.v1"
    errors << "wrong denominator" unless parsed.fetch("predecessors", []).map { |row| row["issue"] } == (308..318).to_a
    errors << "mutation authorized" unless parsed["release_mutation_authorized"] == false
    errors << "wrong validator" unless parsed["validator"] == ".csdlc/prepared/issues/319/validate-release-evidence.rb"
    parsed.fetch("predecessors", []).each do |row|
      evidence = row.fetch("evidence")
      errors << "missing evidence for ##{row['issue']}" unless File.file?(File.join(root, evidence))
      next unless row["merge_commit"]
      ok = system("git", "-C", root, "merge-base", "--is-ancestor", row["merge_commit"], "HEAD", out: File::NULL, err: File::NULL)
      errors << "non-ancestral merge for ##{row['issue']}" unless ok
    end
  end
end
if errors.empty?
  puts JSON.generate(schema: "adl.v092.release_ceremony_validation.v1", mode: mode, status: "pass")
else
  warn JSON.generate(schema: "adl.v092.release_ceremony_validation.v1", mode: mode, status: "fail", errors: errors)
  exit 1
end

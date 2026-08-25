#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "digest"
root = File.expand_path("../../../..", __dir__)
mode = ARGV.fetch(0, "all")
abort "invalid mode: #{mode}" unless %w[all ceremony gate review-contract].include?(mode)
design = File.read(File.join(__dir__, "design.md"))
diagram = File.read(File.join(__dir__, "diagram.mmd"))
errors = []
expected = {
  308 => ["reviewed_green_merge", 447, "9f373f5f04b0f8c9dc6e3e6cbf348fddec98486c"],
  309 => ["reviewed_green_merge", 460, "5b3657582fea2109f000623bb121b7998185ac0a"],
  310 => ["recordless_merge_with_retrospective_wp29_authority", 465, "a06c34774ad88ea8c56a00533f0fcef810fa7441"],
  311 => ["reviewed_green_merge", 466, "035b249096c6a27a6e40af9435d6df8e35090000"],
  312 => ["reviewed_green_merge", 469, "c6792e54df1db5969fa28c59b6dfe4c714ed5559"],
  313 => ["reviewed_green_merge", 470, "c666ff3d411e6062d5cc5750d2f88be9efbcd673"],
  314 => ["external_review_input_no_closing_pr", nil, nil],
  315 => ["reviewed_green_remediation_merge", 473, "e43efbca80cece11b543d931655febd50dfdc755"],
  316 => ["reviewed_green_merge", 472, "5002b387b79f2d8dbf41a8c1a99e5a03bcb5c5d5"],
  317 => ["reviewed_green_merge", 474, "5b035094725d1872b48dda8692ef88f46487f37c"],
  318 => ["reviewed_green_merge", 478, "737257211b08ac90bdb9a2537455de17ac5df1a3"]
}.freeze
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
    rows = parsed.fetch("predecessors", [])
    rows.each do |row|
      issue = row["issue"]
      errors << "wrong authority for ##{issue}" unless [row["disposition"], row["pr"], row["merge_commit"]] == expected[issue]
      evidence = row.fetch("evidence")
      revision = row["evidence_revision"]
      bytes = if revision
                IO.popen(["git", "-C", root, "show", "#{revision}:#{evidence}"], err: File::NULL, &:read)
              elsif File.file?(File.join(root, evidence))
                File.binread(File.join(root, evidence))
              end
      errors << "missing evidence for ##{issue}" unless bytes && !bytes.empty?
      errors << "evidence digest mismatch for ##{issue}" if bytes && Digest::SHA256.hexdigest(bytes) != row["evidence_sha256"]
      if issue == 310 && bytes
        universe = JSON.parse(bytes).fetch("issues").find { |entry| entry["canonical_issue"] == 310 }
        errors << "invalid recordless authority for #310" unless universe && universe.dig("merge", "pr") == 465 && universe.dig("merge", "merge_commit") == expected[310][2] && universe.dig("merge", "checks_status") == "green" && universe.dig("merge", "ancestry") == "ancestral_to_main"
      elsif issue == 314 && bytes
        packet = JSON.parse(bytes)
        errors << "invalid external review input for #314" unless packet["canonical_issue"] == 314 && packet.fetch("non_claims", []).any? { |claim| claim.include?("does not remediate findings") }
      elsif bytes
        errors << "review did not pass for ##{issue}" unless bytes.include?("Result: pass")
      end
      next unless row["merge_commit"]
      ok = system("git", "-C", root, "merge-base", "--is-ancestor", row["merge_commit"], "HEAD", out: File::NULL, err: File::NULL)
      errors << "non-ancestral merge for ##{issue}" unless ok
    end
  end
end
if errors.empty?
  puts JSON.generate(schema: "adl.v092.release_ceremony_validation.v1", mode: mode, status: "pass")
else
  warn JSON.generate(schema: "adl.v092.release_ceremony_validation.v1", mode: mode, status: "fail", errors: errors)
  exit 1
end

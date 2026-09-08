#!/usr/bin/env ruby
# frozen_string_literal: true
TAIL = (1..10).map { |n| format("TAIL-%02d", n) }.freeze
ROOT = File.expand_path("../../../..", __dir__)

def errors_for(plan, checklist)
  errors = []
  [plan, checklist].each_with_index do |text, index|
    errors << "tail sequence mismatch in document #{index + 1}" unless text.scan(/TAIL-\d{2}/).uniq == TAIL
  end
  errors << "merge gate missing" unless plan.match?(/merge-based|reviewed.*merge/im)
  errors << "asynchronous closeout boundary missing" unless [plan, checklist].all? { |text| text.match?(/closeout.*asynchronous|asynchronous.*closeout/im) }
  errors << "operator release gate missing" unless plan.match?(/human release approval|operator authorization/im)
  errors << "completion states not distinguished" unless plan.match?(/candidate is not a release/i)
  errors
end

if ARGV == ["--negative"]
  list = TAIL.map.with_index(1) { |id, n| "#{n}. #{id}" }.join("\n")
  plan = "candidate is not a release\nreviewed merge-based gate\nasynchronous closeout\nhuman release approval\n#{list}"
  checklist = "asynchronous closeout\n#{list}"
  mutations = [
    [plan.sub("4. TAIL-04", "4. TAIL-05").sub("5. TAIL-05", "5. TAIL-04"), checklist],
    [plan.sub("TAIL-10", "TAIL-09"), checklist],
    [plan.sub("human release approval", "automatic release"), checklist],
    [plan, checklist.sub("asynchronous closeout", "closeout gates execution")]
  ]
  abort "negative mutation escaped" unless mutations.all? { |args| !errors_for(*args).empty? }
  puts "issue 524 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

paths = %w[RELEASE_PLAN_v0.92.2.md MILESTONE_CHECKLIST_v0.92.2.md].map { |name| File.join(ROOT, "docs/milestones/v0.92.2", name) }
abort "missing closeout artifact" unless paths.all? { |path| File.file?(path) && !File.zero?(path) }
errors = errors_for(*paths.map { |path| File.read(path) })
abort errors.join("\n") unless errors.empty?
abort "successor package semantics failed" unless system("ruby", ".csdlc/prepared/issues/523/validate-tail07.rb")
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 524 closeout-plan semantics passed"

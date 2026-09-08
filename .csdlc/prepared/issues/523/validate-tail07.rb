#!/usr/bin/env ruby
required = %w[
  docs/milestones/v0.92.2/README.md
  docs/milestones/v0.92.2/WBS_v0.92.2.md
  docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml
  docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
  docs/planning/ADL_FEATURE_LIST.md
]
abort "missing planning artifact" unless required.all? { |path| File.file?(path) && !File.zero?(path) }
abort "planning validator failed" unless system("ruby", ".csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb")
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 523 planning package contract passed"

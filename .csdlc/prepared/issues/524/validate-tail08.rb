#!/usr/bin/env ruby
paths = %w[
  docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md
  docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md
]
abort "missing closeout artifact" unless paths.all? { |path| File.file?(path) && !File.zero?(path) }
content = paths.map { |path| File.read(path) }
(1..10).each do |number|
  id = format("TAIL-%02d", number)
  abort "missing #{id}" unless content.all? { |text| text.include?(id) }
end
abort "planning validator failed" unless system("ruby", ".csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb")
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 524 closeout-plan contract passed"

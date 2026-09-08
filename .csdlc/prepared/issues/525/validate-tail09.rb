#!/usr/bin/env ruby
directory = "docs/milestones/v0.92.1/evidence/release/tail-09"
abort "missing review evidence" unless Dir.exist?(directory) && Dir.glob("#{directory}/**/*").any? { |path| File.file?(path) && !File.zero?(path) }
abort "planning validator failed" unless system("ruby", ".csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb")
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 525 exact-revision review artifact contract passed"

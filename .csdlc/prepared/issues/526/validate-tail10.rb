#!/usr/bin/env ruby
notes = "docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md"
directory = "docs/milestones/v0.92.1/evidence/release/tail-10"
abort "missing release notes" unless File.file?(notes) && !File.zero?(notes)
abort "missing ceremony receipt" unless Dir.exist?(directory) && Dir.glob("#{directory}/**/*").any? { |path| File.file?(path) && !File.zero?(path) }
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 526 ceremony receipt contract passed"

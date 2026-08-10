#!/usr/bin/env ruby
# frozen_string_literal: true

require "pathname"
require "yaml"

root = Pathname.new(File.expand_path("../../../..", __dir__))
milestone = root.join("docs/milestones/v0.92.1")
wave = milestone.join("WP_ISSUE_WAVE_v0.92.1.yaml")
YAML.safe_load(wave.read, permitted_classes: [], aliases: false)

files = Dir[milestone.join("**/*.md").to_s].map { |path| Pathname.new(path) }
abort("no milestone markdown files found") if files.empty?

missing = []
untracked = []
placeholder_hits = []
files.each do |path|
  text = path.read
  text.scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |target|
    next if target.start_with?("http://", "https://", "#", "mailto:")
    relative = target.split("#", 2).first
    next if relative.nil? || relative.empty?
    resolved = path.dirname.join(relative).cleanpath
    unless resolved.exist?
      missing << "#{path.relative_path_from(root)} -> #{target}"
      next
    end
    repo_relative = resolved.relative_path_from(root).to_s
    tracked = system("git", "ls-files", "--error-unmatch", "--", repo_relative,
                     chdir: root.to_s, out: File::NULL, err: File::NULL)
    untracked << "#{path.relative_path_from(root)} -> #{target}" unless tracked
  end
  text.each_line.with_index(1) do |line, number|
    next unless line.match?(/\b(TODO|FIXME|PLACEHOLDER)\b|\bTBD\s*:/)
    placeholder_hits << "#{path.relative_path_from(root)}:#{number}:#{line.strip}"
  end
end

abort("missing local links:\n#{missing.join("\n")}") unless missing.empty?
abort("local links target untracked files:\n#{untracked.join("\n")}") unless untracked.empty?
abort("unresolved placeholders:\n#{placeholder_hits.join("\n")}") unless placeholder_hits.empty?

puts "PASS: v0.92.1 YAML, links, and placeholders"

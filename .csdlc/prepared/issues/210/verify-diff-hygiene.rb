#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  abort("issue 210 diff hygiene git failure: #{err}") unless status.success?
  out
end

base = git("merge-base", "origin/main", "HEAD").strip
head = git("rev-parse", "HEAD").strip
dirty = git("status", "--porcelain=v1", "--untracked-files=all").lines.map(&:strip)
dirty.reject! { |line| line.end_with?(".csdlc/evidence/210/") || line.include?(" .csdlc/evidence/210/") }
abort("issue 210 diff hygiene requires clean worktree") unless dirty.empty?
abort("issue 210 base is not ancestral") unless system("git", "merge-base", "--is-ancestor", base, head, chdir: ROOT.to_s)

changed = git("diff", "--name-only", "#{base}...#{head}").lines.map(&:strip).reject(&:empty?)
offenders = []
changed.each do |path|
  next unless File.file?(ROOT.join(path))
  bytes = File.binread(ROOT.join(path))
  offenders << "#{path}:missing-final-newline" unless bytes.empty? || bytes.end_with?("\n")
  bytes.each_line.with_index(1) do |line, number|
    offenders << "#{path}:#{number}:trailing-whitespace" if line.match?(/[ \t]\r?\n\z/)
  end
end
abort("issue 210 diff hygiene failed:\n#{offenders.join("\n")}") unless offenders.empty?

puts "PASS: issue #210 diff hygiene clean for #{base}...#{head}"

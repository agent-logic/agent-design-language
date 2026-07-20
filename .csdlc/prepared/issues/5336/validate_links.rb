#!/usr/bin/env ruby
# frozen_string_literal: true

root = File.expand_path("../../../..", __dir__)
missing = []

Dir.glob(File.join(root, "docs/milestones/v0.91.8/**/*.md")).sort.each do |source|
  File.read(source).scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |href|
    next if href.match?(/\A(?:https?:|mailto:|#)/)

    target = href.split("#", 2).first
    next if target.nil? || target.empty?

    resolved = File.expand_path(target, File.dirname(source))
    missing << "#{source.delete_prefix(root + "/")}: #{href}" unless File.exist?(resolved)
  end
end

abort(missing.join("\n")) unless missing.empty?
puts "v0.91.8 local links ok"

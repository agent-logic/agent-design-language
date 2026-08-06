#!/usr/bin/env ruby
# frozen_string_literal: true

root = File.expand_path("../../..", __dir__)
articles = File.join(root, "docs/milestones/v0.92/publication/articles")
slugs = %w[
  01-what-is-adl
  02-adl-runtime-and-cognitive-spacetime-model
  03-godel-agents-and-godel-hadamard-bayes-algorithm
  04-the-freedom-gate
  05-uts-and-acc-making-agents-with-tools-safe
  06-codefriend-and-the-cognitive-sdlc
  07-continuous-adversarial-verification-for-continuous-security
  08-agent-economics
  09-adl-and-social-intelligence
  10-whats-next-for-adl
]
required = slugs.flat_map do |slug|
  %w[SOURCE_PACKET.md ARTICLE.md EDITORIAL_REVIEW.md].map do |name|
    File.join(articles, slug, name)
  end
end
required.concat(%w[SERIES_ARC_AND_CLAIM_MATRIX.md PUBLICATION_DISPOSITION.md].map { |name| File.join(articles, name) })

missing = required.reject { |path| File.file?(path) && !File.read(path).strip.empty? }
raise "missing or empty WP-24 artifacts: #{missing.join(', ')}" unless missing.empty?

contents = required.map { |path| [path, File.read(path)] }
forbidden = /\b(TODO|TBD|PLACEHOLDER|lorem ipsum)\b|\/Users\/|file:\/\//i
violations = contents.select { |_path, body| body.match?(forbidden) }.map(&:first)
raise "placeholder or private-path content: #{violations.join(', ')}" unless violations.empty?

if ARGV.include?("--negative")
  disposition = File.read(File.join(articles, "PUBLICATION_DISPOSITION.md"))
  unless disposition.match?(/review-ready|operator-approved/i) &&
         !disposition.match?(/autonomously published|auto-published/i)
    raise "publication disposition must remain review-ready or operator-approved"
  end
end

puts "WP-24 article series contract passed"

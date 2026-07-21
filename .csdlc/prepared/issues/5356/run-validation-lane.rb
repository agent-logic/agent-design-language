#!/usr/bin/env ruby
# frozen_string_literal: true

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
allowed = %w[freeze-corpus code security tests docs architecture evidence synthesis review-quality complete post-merge-exact].freeze
abort("unknown validation lane: #{lane}") unless allowed.include?(lane)

warn("#5356 #{lane}: unavailable during preparation; run only after #5360 is merged and typed closed_out with a retained ancestral receipt and the review-output claim is amended")
exit 1

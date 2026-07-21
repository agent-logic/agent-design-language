#!/usr/bin/env ruby
# frozen_string_literal: true

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
allowed = %w[focused-doc-alignment complete post-merge-exact].freeze
abort("unknown validation lane: #{lane}") unless allowed.include?(lane)

warn("#5360 #{lane}: unavailable during preparation; run after the typed #5351 terminal gate and an exact reviewed claim amendment")
exit 1

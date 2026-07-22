#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[convergence-contract property-matrix budgets post-merge-exact]
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

warn("#{lane} is execution-gated and has no implementation in the preparation-only packet")
puts JSON.pretty_generate(status: "blocked", lane: lane, reason: "#5499 and #5498 must be live merged and ancestral before implementation")
exit 2
